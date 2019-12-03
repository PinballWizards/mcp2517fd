use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin};
use embedded_hal::spi::FullDuplex;

use super::generic::*;

pub struct Controller<T, SS> {
    spi_master: T,
    slave_select: SS,
}

impl<T, SS> Controller<T, SS>
where
    T: FullDuplex<u8>,
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

    pub fn reset(&mut self) -> Result<(), T::Error> {
        self.ready_slave_select();

        let instruction = Instruction(OpCode::RESET);
        match self.send(&instruction.0.to_be_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => {
                self.slave_select.set_low().unwrap();
                Err(err)
            }
        }
    }

    pub fn read_sfr(&mut self, address: SFRAddress) -> Result<u32, T::Error> {
        self.ready_slave_select();
        let mut instruction = Instruction(OpCode::READ_SFR);
        instruction.set_address(address as u16);
        match self.send(&instruction.0.to_be_bytes()) {
            Ok(_) => (),
            Err(err) => {
                self.slave_select.set_low().unwrap();
                return Err(err);
            }
        }

        let mut read_value: u32 = 0;

        // Read four bytes back
        for i in 0..4 {
            match self.read() {
                Ok(val) => read_value |= (val as u32) << 8 * i,
                Err(val) => {
                    self.slave_select.set_low().unwrap();
                    return Err(val);
                }
            }
        }
        self.slave_select.set_high().unwrap();
        Ok(read_value)
    }

    pub fn write_sfr(&mut self, address: SFRAddress, value: u32) -> Result<u32, T::Error> {
        self.ready_slave_select();
        let mut instruction = Instruction(OpCode::WRITE_SFR);
        instruction.set_address(address as u16);
        let ret = match self.send(&instruction.0.to_be_bytes()) {
            Ok(_) => Ok(value),
            Err(err) => Err(err),
        };
        self.slave_select.set_high().unwrap();
        ret
    }

    fn read(&mut self) -> Result<u8, T::Error> {
        block!(self.spi_master.read())
    }

    fn send(&mut self, data: &[u8]) -> Result<(), <T as FullDuplex<u8>>::Error> {
        data.iter()
            .try_for_each(|v| block!(self.spi_master.send(*v)))
    }

    pub fn free(mut self) -> (T, SS) {
        self.slave_select.set_high().unwrap();
        (self.spi_master, self.slave_select)
    }
}
