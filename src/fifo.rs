use bitfield::*;

use crate::generic::SFRAddress;

bitfield! {
    pub struct ControlRegister(u32);
    u8;
    pub tfnrfnie, set_tfnrnfie: 0;
    pub tfhrfhie, set_tfhrfhie: 1;
    pub tfhrffie, set_tfhrffie: 2;
    pub rxovie, set_rxovie: 3;
    pub txatie, set_txatie: 4;
    pub rxtsen, set_rxtsen: 5;
    pub rtren, set_rtren: 6;
    pub txen, set_txen: 7;
    pub uinc, set_uinc: 8;
    pub txreq, set_txreq: 9;
    pub freset, set_freset: 10;
    pub txpri, set_txpri: 20, 16;
    pub txat, set_txat: 22, 21;
    pub fsize, set_fsize: 28, 24;
    pub plsize, set_plsize: 31, 29;
}

bitfield! {
    pub struct StatusRegister(u32);
    u8;
    pub tfnrfnif, set_tfnrfnif: 0;
    pub tfhrfhif, set_tfhrfhif: 1;
    pub tferffif, set_tferffif: 2;
    pub rxovif, set_rxovif: 3;
    pub txatif, set_txatif: 4;
    pub txerr, set_txerr: 5;
    pub txlarb, set_txlarb: 6;
    pub txabt, set_txabt: 7;
    pub fifoci, _: 12, 8;
}

bitfield! {
    pub struct UserAddressRegister(u32);
    u32;
    pub fifoua, set_fifoua: 31, 0;
}

pub fn get_fifo_control_address(fifo_number: u8) -> Result<SFRAddress, u8> {
    match fifo_number {
        1 => Ok(SFRAddress::C1FIFOCON1),
        2 => Ok(SFRAddress::C1FIFOCON2),
        3 => Ok(SFRAddress::C1FIFOCON3),
        4 => Ok(SFRAddress::C1FIFOCON4),
        5 => Ok(SFRAddress::C1FIFOCON5),
        6 => Ok(SFRAddress::C1FIFOCON6),
        7 => Ok(SFRAddress::C1FIFOCON7),
        8 => Ok(SFRAddress::C1FIFOCON8),
        9 => Ok(SFRAddress::C1FIFOCON9),
        10 => Ok(SFRAddress::C1FIFOCON10),
        11 => Ok(SFRAddress::C1FIFOCON11),
        12 => Ok(SFRAddress::C1FIFOCON12),
        13 => Ok(SFRAddress::C1FIFOCON13),
        14 => Ok(SFRAddress::C1FIFOCON14),
        15 => Ok(SFRAddress::C1FIFOCON15),
        16 => Ok(SFRAddress::C1FIFOCON16),
        17 => Ok(SFRAddress::C1FIFOCON17),
        18 => Ok(SFRAddress::C1FIFOCON18),
        19 => Ok(SFRAddress::C1FIFOCON19),
        20 => Ok(SFRAddress::C1FIFOCON20),
        21 => Ok(SFRAddress::C1FIFOCON21),
        22 => Ok(SFRAddress::C1FIFOCON22),
        23 => Ok(SFRAddress::C1FIFOCON23),
        24 => Ok(SFRAddress::C1FIFOCON24),
        25 => Ok(SFRAddress::C1FIFOCON25),
        26 => Ok(SFRAddress::C1FIFOCON26),
        27 => Ok(SFRAddress::C1FIFOCON27),
        28 => Ok(SFRAddress::C1FIFOCON28),
        29 => Ok(SFRAddress::C1FIFOCON29),
        30 => Ok(SFRAddress::C1FIFOCON30),
        31 => Ok(SFRAddress::C1FIFOCON31),
        _ => Err(fifo_number),
    }
}

pub fn get_fifo_status_address(fifo_number: u8) -> Result<SFRAddress, u8> {
    match fifo_number {
        1 => Ok(SFRAddress::C1FIFOSTA1),
        2 => Ok(SFRAddress::C1FIFOSTA2),
        3 => Ok(SFRAddress::C1FIFOSTA3),
        4 => Ok(SFRAddress::C1FIFOSTA4),
        5 => Ok(SFRAddress::C1FIFOSTA5),
        6 => Ok(SFRAddress::C1FIFOSTA6),
        7 => Ok(SFRAddress::C1FIFOSTA7),
        8 => Ok(SFRAddress::C1FIFOSTA8),
        9 => Ok(SFRAddress::C1FIFOSTA9),
        10 => Ok(SFRAddress::C1FIFOSTA10),
        11 => Ok(SFRAddress::C1FIFOSTA11),
        12 => Ok(SFRAddress::C1FIFOSTA12),
        13 => Ok(SFRAddress::C1FIFOSTA13),
        14 => Ok(SFRAddress::C1FIFOSTA14),
        15 => Ok(SFRAddress::C1FIFOSTA15),
        16 => Ok(SFRAddress::C1FIFOSTA16),
        17 => Ok(SFRAddress::C1FIFOSTA17),
        18 => Ok(SFRAddress::C1FIFOSTA18),
        19 => Ok(SFRAddress::C1FIFOSTA19),
        20 => Ok(SFRAddress::C1FIFOSTA20),
        21 => Ok(SFRAddress::C1FIFOSTA21),
        22 => Ok(SFRAddress::C1FIFOSTA22),
        23 => Ok(SFRAddress::C1FIFOSTA23),
        24 => Ok(SFRAddress::C1FIFOSTA24),
        25 => Ok(SFRAddress::C1FIFOSTA25),
        26 => Ok(SFRAddress::C1FIFOSTA26),
        27 => Ok(SFRAddress::C1FIFOSTA27),
        28 => Ok(SFRAddress::C1FIFOSTA28),
        29 => Ok(SFRAddress::C1FIFOSTA29),
        30 => Ok(SFRAddress::C1FIFOSTA30),
        31 => Ok(SFRAddress::C1FIFOSTA31),
        _ => Err(fifo_number),
    }
}
