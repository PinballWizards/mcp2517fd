use embedded_hal::blocking::spi::{Transfer, Write};
use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin};

use crate::fifo;
use crate::generic::*;

pub enum Error<TR, TW, E> {
    SPIRead(TR),
    SPIWrite(TW),
    Other(E),
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
        slave_select.set_low().unwrap();
        Self {
            spi_master,
            slave_select,
        }
    }

    fn ready_slave_select(&mut self) -> () {
        if self.slave_select.is_set_low().unwrap() {
            self.slave_select.set_high().unwrap();
        }
        self.slave_select.set_low().unwrap();
    }

    pub fn reset(
        &mut self,
    ) -> Result<(), Error<<T as Transfer<u8>>::Error, <T as Write<u8>>::Error, u8>> {
        self.ready_slave_select();

        let instruction = Instruction(OpCode::RESET);
        match self.send(&instruction.0.to_be_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => {
                self.slave_select.set_low().unwrap();
                Err(Error::SPIWrite(err))
            }
        }
    }

    pub fn read_sfr(
        &mut self,
        address: &SFRAddress,
    ) -> Result<u32, Error<<T as Transfer<u8>>::Error, <T as Write<u8>>::Error, u8>> {
        self.ready_slave_select();
        let mut instruction = Instruction(OpCode::READ_SFR);
        instruction.set_address(*address as u16);
        match self.send(&instruction.0.to_le_bytes()) {
            Ok(_) => (),
            Err(err) => {
                self.slave_select.set_high().unwrap();
                return Err(Error::SPIWrite(err));
            }
        }

        let mut read_value: u32 = 0;

        // Read four bytes back
        for i in 0..4 {
            match self.read() {
                Ok(val) => read_value |= (val as u32) << (8 * i),
                Err(val) => {
                    self.slave_select.set_high().unwrap();
                    return Err(Error::SPIRead(val));
                }
            }
        }
        self.slave_select.set_high().unwrap();
        Ok(read_value)
    }

    pub fn write_sfr(
        &mut self,
        address: &SFRAddress,
        value: u32,
    ) -> Result<(), Error<<T as Transfer<u8>>::Error, <T as Write<u8>>::Error, u8>> {
        self.ready_slave_select();
        let mut instruction = Instruction(OpCode::WRITE_SFR);
        instruction.set_address(*address as u16);
        match self.send(&instruction.0.to_le_bytes()) {
            Ok(_) => (),
            Err(err) => {
                self.slave_select.set_high().unwrap();
                return Err(Error::SPIWrite(err));
            }
        }

        match self.send(&value.to_be_bytes()) {
            Ok(_) => (),
            Err(err) => {
                self.slave_select.set_high().unwrap();
                return Err(Error::SPIWrite(err));
            }
        }
        self.slave_select.set_high().unwrap();
        Ok(())
    }

    /// Enables the transmit event FIFO by setting C1CON.STEF and C1TEFCON.FSIZE bits.
    /// Be aware that object_count MUST be <= 31, any other values will be disregarded.
    ///
    /// Also please keep in mind that the total RAM size is 2K and this code does absolutely
    /// zero validation that your configuration is under this limit. The documentation recommends
    /// configuring the TEF first, then TEQ, then FIFOs as necessary.
    pub fn enable_transmit_event_fifo(
        &mut self,
        object_count: u32,
    ) -> Result<(), Error<<T as Transfer<u8>>::Error, <T as Write<u8>>::Error, u8>> {
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
    pub fn configure_fifo_control<F>(
        &mut self,
        fifo_number: u8,
        f: F,
    ) -> Result<(), Error<<T as Transfer<u8>>::Error, <T as Write<u8>>::Error, u8>>
    where
        F: FnOnce(&mut fifo::ControlRegister) -> &mut fifo::ControlRegister,
    {
        let address = match fifo::get_fifo_control_address(fifo_number) {
            Ok(addr) => addr,
            Err(e) => return Err(Error::Other(e)),
        };

        let raw_register: u32 = self.read_sfr(&address)?;

        let mut control_register = fifo::ControlRegister(raw_register);

        self.write_sfr(&address, f(&mut control_register).0)
    }

    pub fn read_fifo_status(
        &mut self,
        fifo_number: u8,
    ) -> Result<fifo::StatusRegister, Error<<T as Transfer<u8>>::Error, <T as Write<u8>>::Error, u8>>
    {
        let address = match fifo::get_fifo_status_address(fifo_number) {
            Ok(addr) => addr,
            Err(e) => return Err(Error::Other(e)),
        };

        match self.read_sfr(&address) {
            Ok(val) => Ok(fifo::StatusRegister(val)),
            Err(err) => Err(err),
        }
    }

    pub fn write_fifo_status<F>(
        &mut self,
        fifo_number: u8,
        f: F,
    ) -> Result<(), Error<<T as Transfer<u8>>::Error, <T as Write<u8>>::Error, u8>>
    where
        F: FnOnce(&mut fifo::StatusRegister) -> &mut fifo::StatusRegister,
    {
        let mut status_register = match self.read_fifo_status(fifo_number) {
            Ok(reg) => reg,
            Err(err) => return Err(err),
        };

        let address = match fifo::get_fifo_status_address(fifo_number) {
            Ok(val) => val,
            Err(err) => return Err(Error::Other(err)),
        };

        self.write_sfr(&address, f(&mut status_register).0)
    }

    pub fn read_fifo_user_address(
        &mut self,
        fifo_number: u8,
    ) -> Result<
        fifo::UserAddressRegister,
        Error<<T as Transfer<u8>>::Error, <T as Write<u8>>::Error, u8>,
    > {
        let address = match fifo::get_fifo_status_address(fifo_number) {
            Ok(val) => val,
            Err(err) => return Err(Error::Other(err)),
        };

        match self.read_sfr(&address) {
            Ok(val) => Ok(fifo::UserAddressRegister(val)),
            Err(err) => Err(err),
        }
    }

    pub fn write_fifo_user_address<F>(
        &mut self,
        fifo_number: u8,
        f: F,
    ) -> Result<(), Error<<T as Transfer<u8>>::Error, <T as Write<u8>>::Error, u8>>
    where
        F: FnOnce(&mut fifo::UserAddressRegister) -> fifo::UserAddressRegister,
    {
        let address = match fifo::get_fifo_status_address(fifo_number) {
            Ok(val) => val,
            Err(err) => return Err(Error::Other(err)),
        };

        let mut register = match self.read_sfr(&address) {
            Ok(val) => fifo::UserAddressRegister(val),
            Err(err) => return Err(err),
        };

        self.write_sfr(&address, f(&mut register).0)
    }

    fn read(&mut self) -> Result<u8, <T as Transfer<u8>>::Error> {
        let mut buf = [0u8; 1];
        match self.spi_master.transfer(&mut buf) {
            Ok(val) => Ok(val[0]),
            Err(err) => Err(err),
        }
    }

    fn send(&mut self, data: &[u8]) -> Result<(), <T as Write<u8>>::Error> {
        self.spi_master.write(data)
    }

    pub fn free(mut self) -> (T, SS) {
        self.slave_select.set_high().unwrap();
        (self.spi_master, self.slave_select)
    }
}
