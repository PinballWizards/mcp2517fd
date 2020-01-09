use core::cmp::Ord;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::blocking::spi::{Transfer, Write};
use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin};

use crate::can;
use crate::fifo;
use crate::generic::*;
use crate::settings::Settings;

pub enum Error {
    SPIRead,
    SPIWrite,
    InvalidFIFO(u8),
    InvalidRAMAddress(u16),
    Other,
}

pub enum ConfigError {
    ConfigurationModeTimeout,
}

pub struct Controller<T, SS> {
    spi_master: T,
    slave_select: SS,
}

impl<T, SS> Controller<T, SS>
where
    T: Write<u8> + Transfer<u8>,
    SS: StatefulOutputPin,
    <SS as OutputPin>::Error: core::fmt::Debug,
{
    pub fn new(spi_master: T, mut slave_select: SS) -> Controller<T, SS> {
        slave_select.set_high().unwrap();
        Self {
            spi_master,
            slave_select,
        }
    }

    pub fn configure<D: DelayUs<u32>>(
        &mut self,
        settings: Settings,
        delay: &mut D,
    ) -> Result<(), ConfigError> {
        // I'm going to borrow the ordering and logic for this code from pierremolinaro
        // on github: https://github.com/pierremolinaro/acan2517

        let mut c1con = match self.read_sfr(&SFRAddress::C1CON) {
            Ok(val) => can::control::C1CON(val),
            Err(_) => return Err(ConfigError::ConfigurationModeTimeout),
        };
        c1con.set_opmode(can::control::OperationMode::Configuration);
        match self.write_sfr(&SFRAddress::C1CON, c1con.0) {
            Ok(_) => (),
            Err(_) => return Err(ConfigError::ConfigurationModeTimeout),
        };

        // Delay 2ms checking every 500us for config mode
        for i in 0..5 {
            if c1con.opmode() == can::control::OperationMode::Configuration {
                break;
            } else if i == 4 {
                return Err(ConfigError::ConfigurationModeTimeout);
            }
            delay.delay_us(500u32);
            c1con = match self.read_sfr(&SFRAddress::C1CON) {
                Ok(val) => can::control::C1CON(val),
                Err(_) => can::control::C1CON(0),
            };
        }

        // Now in configuration mode --------------------

        // Verify SPI connection is working by writing to an available ram location
        for i in 0..32 {}

        Ok(())
    }

    /// Ready slave select will pull the slave select line to ACTIVE.
    fn ready_slave_select(&mut self) -> () {
        if self.slave_select.is_set_low().unwrap() {
            self.slave_select.set_high().unwrap();
        }
        self.slave_select.set_low().unwrap();
    }

    /// Reset slave select will pull the slave select line to INACTIVE.
    fn reset_slave_select(&mut self) {
        self.slave_select.set_high().unwrap();
    }

    /// Performs a software reset of the MCP2517 chip over SPI.
    pub fn reset(&mut self) -> Result<(), Error> {
        self.ready_slave_select();

        let instruction = Instruction(OpCode::RESET);
        match self.send(&instruction.0.to_be_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => {
                self.slave_select.set_high().unwrap();
                Err(Error::SPIWrite)
            }
        }
    }

    /// Enables the transmit event FIFO by setting C1CON.STEF and C1TEFCON.FSIZE bits.
    /// Be aware that object_count MUST be <= 31, any other values will be disregarded.
    ///
    /// Also please keep in mind that the total RAM size is 2K and this code does absolutely
    /// zero validation that your configuration is under this limit. The documentation recommends
    /// configuring the TEF first, then TEQ, then FIFOs as necessary.
    pub fn enable_transmit_event_fifo(&mut self, object_count: u32) -> Result<(), Error> {
        let mut c1con = self.read_sfr(&SFRAddress::C1CON)?;

        // Enable TEF
        c1con |= 1 << 19;
        self.write_sfr(&SFRAddress::C1CON, c1con)?;

        let mut c1tefcon = self.read_sfr(&SFRAddress::C1TEFCON)?;

        // Reserve space in RAM, max 31 objects
        c1tefcon |= object_count & 0b1111;
        self.write_sfr(&SFRAddress::C1TEFCON, c1tefcon)?;

        Ok(())
    }

    /// Configures a FIFO based on the settings provided. As per documentation, a single FIFO must
    /// be dedicated to RX or TX and all objects in that queue must have the same payload size.
    ///
    /// fifo_number may be between 1 and 31 inclusive, this function will return Ok(()) if it
    /// is passed an invalid number.
    pub fn configure_fifo_control<F>(&mut self, fifo_number: u8, f: F) -> Result<(), Error>
    where
        F: FnOnce(&mut fifo::ControlRegister) -> &mut fifo::ControlRegister,
    {
        let address = match fifo::get_fifo_control_address(fifo_number) {
            Ok(addr) => addr,
            Err(e) => return Err(Error::InvalidFIFO(e)),
        };

        let raw_register: u32 = self.read_sfr(&address)?;

        let mut control_register = fifo::ControlRegister(raw_register);

        self.write_sfr(&address, f(&mut control_register).0)
    }

    pub fn read_fifo_status(&mut self, fifo_number: u8) -> Result<fifo::StatusRegister, Error> {
        let address = match fifo::get_fifo_status_address(fifo_number) {
            Ok(addr) => addr,
            Err(e) => return Err(Error::InvalidFIFO(e)),
        };

        match self.read_sfr(&address) {
            Ok(val) => Ok(fifo::StatusRegister(val)),
            Err(err) => Err(err),
        }
    }

    pub fn write_fifo_status<F>(&mut self, fifo_number: u8, f: F) -> Result<(), Error>
    where
        F: FnOnce(&mut fifo::StatusRegister) -> &mut fifo::StatusRegister,
    {
        let mut status_register = match self.read_fifo_status(fifo_number) {
            Ok(reg) => reg,
            Err(err) => return Err(err),
        };

        let address = match fifo::get_fifo_status_address(fifo_number) {
            Ok(val) => val,
            Err(e) => return Err(Error::InvalidFIFO(e)),
        };

        self.write_sfr(&address, f(&mut status_register).0)
    }

    pub fn read_fifo_user_address(
        &mut self,
        fifo_number: u8,
    ) -> Result<fifo::UserAddressRegister, Error> {
        let address = match fifo::get_fifo_status_address(fifo_number) {
            Ok(val) => val,
            Err(e) => return Err(Error::InvalidFIFO(e)),
        };

        match self.read_sfr(&address) {
            Ok(val) => Ok(fifo::UserAddressRegister(val)),
            Err(err) => Err(err),
        }
    }

    pub fn write_fifo_user_address<F>(&mut self, fifo_number: u8, f: F) -> Result<(), Error>
    where
        F: FnOnce(&mut fifo::UserAddressRegister) -> fifo::UserAddressRegister,
    {
        let address = match fifo::get_fifo_status_address(fifo_number) {
            Ok(val) => val,
            Err(e) => return Err(Error::InvalidFIFO(e)),
        };

        let mut register = match self.read_sfr(&address) {
            Ok(val) => fifo::UserAddressRegister(val),
            Err(err) => return Err(err),
        };

        self.write_sfr(&address, f(&mut register).0)
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        match self.spi_master.transfer(buf) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::SPIRead),
        }
    }

    fn read32(&mut self) -> Result<u32, Error> {
        let mut buf = [0u8; 4];
        self.read(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    fn send(&mut self, data: &[u8]) -> Result<(), Error> {
        match self.spi_master.write(data) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::SPIWrite),
        }
    }

    pub fn read_sfr(&mut self, address: &SFRAddress) -> Result<u32, Error> {
        self.ready_slave_select();
        let mut instruction = Instruction(OpCode::READ);
        instruction.set_address(*address as u16);
        match self.send(&instruction.to_spi_data()) {
            Ok(_) => (),
            Err(e) => {
                self.reset_slave_select();
                return Err(e);
            }
        }

        let read_value = self.read32();
        self.reset_slave_select();
        read_value
    }

    pub fn write_sfr(&mut self, address: &SFRAddress, value: u32) -> Result<(), Error> {
        self.ready_slave_select();
        let mut instruction = Instruction(OpCode::WRITE);
        instruction.set_address(*address as u16);
        match self.send(&instruction.to_spi_data()) {
            Ok(_) => (),
            Err(e) => {
                self.reset_slave_select();
                return Err(e);
            }
        }

        // The "instruction" needs to be converted to BE bytes but the actual SFR register
        // needs to be in LE format!!!
        let ret = self.send(&value.to_le_bytes());
        self.reset_slave_select();
        ret
    }

    fn verify_ram_address(&self, address: u16, data_size: usize) -> Result<(), Error> {
        let low_address = 0x400;
        let high_address = 0xBFF;

        if address < low_address {
            return Err(Error::InvalidRAMAddress(address));
        }

        match (address + data_size as u16).cmp(&high_address) {
            core::cmp::Ordering::Greater => return Err(Error::InvalidRAMAddress(address)),
            _ => Ok(()),
        }
    }

    pub fn read_ram(&mut self, address: u16, data: &mut [u8]) -> Result<(), Error> {
        self.verify_ram_address(address, data.len())?;
        self.ready_slave_select();

        let mut instruction = Instruction(OpCode::READ);
        instruction.set_address(address);

        match self.send(&instruction.to_spi_data()) {
            Ok(_) => (),
            Err(_) => {
                self.reset_slave_select();
                return Err(Error::SPIWrite);
            }
        }

        let result = self.read(data);
        self.reset_slave_select();
        result
    }

    pub fn write_ram(&mut self, address: u16, data: &[u8]) -> Result<(), Error> {
        self.verify_ram_address(address, data.len())?;
        self.ready_slave_select();

        let mut instruction = Instruction(OpCode::WRITE);
        instruction.set_address(address);

        match self.send(&instruction.to_spi_data()) {
            Ok(_) => (),
            Err(_) => {
                self.reset_slave_select();
                return Err(Error::SPIWrite);
            }
        };

        let result = self.send(data);
        self.reset_slave_select();
        result
    }

    pub fn free(mut self) -> (T, SS) {
        self.slave_select.set_high().unwrap();
        (self.spi_master, self.slave_select)
    }
}
