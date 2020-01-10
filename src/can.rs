pub mod control {
    use crate::generic::{Register, SFRAddress};
    use core::convert::TryFrom;
    use num_enum::{IntoPrimitive, TryFromPrimitive, TryFromPrimitiveError};

    #[derive(Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
    #[repr(u8)]
    pub enum OperationMode {
        NormalCanFD = 0,
        Sleep = 1,
        InternalLoopback = 2,
        ListenOnly = 3,
        Configuration = 4,
        ExternalLoopback = 5,
        NormalCan2 = 6,
        Restricted = 7,
        Unknown = 0xff,
    }

    /// All times are in arbitration bit times
    #[derive(Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
    #[repr(u8)]
    pub enum InterTransmissionDelay {
        NoDelay = 0,
        Delay2 = 1,
        Delay4 = 2,
        Delay8 = 3,
        Delay16 = 4,
        Delay32 = 5,
        Delay64 = 6,
        Delay128 = 7,
        Delay256 = 8,
        Delay512 = 9,
        Delay1024 = 10,
        Delay2048 = 11,
        Delay4096 = 12,
    }

    #[derive(Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
    #[repr(u8)]
    pub enum WakeupFilterTime {
        T00Filter = 0,
        T01Filter = 1,
        T10Filter = 2,
        T11Filter = 3,
    }

    bitfield! {
        pub struct C1CON(u32);
        impl Debug;
        u8;
        pub dncnt, set_dncnt: 4, 0;
        pub isocrcen, set_isocrcen: 5;
        pub pxedis, set_pxedis: 6;
        pub wakfil, set_wakfil: 8;
        _wft, _set_wft: 10, 9;
        pub busy, _: 11;
        pub brsdis, set_brsdis: 12;
        pub rtxat, set_rtxat: 16;
        pub esigm, set_esigm: 17;
        pub serr2lom, set_serr2lom: 18;
        pub stef, set_stef: 19;
        pub txqen, set_txqen: 20;
        _opmod, _: 23, 21;
        _, _set_reqop: 26, 24;
        pub abat, set_abat: 27;
        _txbws, _set_txbws: 31, 28;
    }

    impl Register for C1CON {
        fn address() -> SFRAddress {
            SFRAddress::C1CON
        }
    }

    impl From<C1CON> for u32 {
        fn from(reg: C1CON) -> Self {
            reg.0
        }
    }

    impl C1CON {
        pub fn wft(&self) -> Result<WakeupFilterTime, TryFromPrimitiveError<WakeupFilterTime>> {
            WakeupFilterTime::try_from(self._wft())
        }

        pub fn set_wft(&mut self, filter: WakeupFilterTime) {
            self._set_wft(filter.into())
        }

        pub fn opmode(&self) -> OperationMode {
            match OperationMode::try_from(self._opmod()) {
                Ok(val) => val,
                Err(_) => OperationMode::Unknown,
            }
        }

        pub fn set_opmode(&mut self, mode: OperationMode) {
            self._set_reqop(mode.into());
        }

        pub fn txbws(
            &self,
        ) -> Result<InterTransmissionDelay, TryFromPrimitiveError<InterTransmissionDelay>> {
            InterTransmissionDelay::try_from(self._txbws())
        }

        pub fn set_txbws(&mut self, delay: InterTransmissionDelay) {
            self._set_txbws(delay.into());
        }
    }

    #[derive(Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
    #[repr(u8)]
    pub enum RetransmissionAttempts {
        Disabled = 0,
        ThreeRetries = 1,
        UnlimitedRetries = 3,
    }

    #[derive(Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
    #[repr(u8)]
    pub enum PayloadSize {
        Bytes8 = 0,
        Bytes12 = 1,
        Bytes16 = 2,
        Bytes20 = 3,
        Bytes24 = 4,
        Bytes32 = 5,
        Bytes48 = 6,
        Bytes64 = 7,
    }

    bitfield! {
        pub struct C1TXQCON(u32);
        impl Debug;
        u8;
        pub txqnie, set_txqnie: 0;
        pub txqeie, set_txqeie: 2;
        pub txatie, set_txatie: 4;
        pub txen, _: 7;
        pub uinc, set_uinc: 8;
        pub txreq, set_txreq: 9;
        pub freset, _: 10;
        pub txpri, set_txpri: 20, 16;
        _txat, _set_txat: 22, 21;
        _fsize, _set_fsize: 28, 24;
        _plsize, _set_plsize: 31, 29;
    }

    impl C1TXQCON {
        /// 0x1F
        pub fn highest_priority() -> u8 {
            0b11111
        }

        /// 0x0
        pub fn lowest_priority() -> u8 {
            0
        }

        pub fn retransmission_attempts(&self) -> RetransmissionAttempts {
            match RetransmissionAttempts::try_from(self._txat()) {
                Ok(val) => val,
                _ => RetransmissionAttempts::UnlimitedRetries,
            }
        }

        pub fn set_retransmission_attempts(&mut self, value: RetransmissionAttempts) {
            self._set_txat(value.into())
        }

        // Max size is 32
        pub fn fifo_size(&self) -> u8 {
            self._fsize() + 1
        }

        /// Max size is 32.
        pub fn set_fifo_size(&mut self, size: u8) {
            self._set_fsize(match size.cmp(&32u8) {
                core::cmp::Ordering::Greater => 31,
                _ => size - 1,
            });
        }

        pub fn payload_size(&self) -> PayloadSize {
            match PayloadSize::try_from(self._plsize()) {
                Ok(val) => val,
                _ => PayloadSize::Bytes8,
            }
        }

        pub fn set_payload_size(&mut self, size: PayloadSize) {
            self._set_plsize(size.into());
        }
    }

    impl Register for C1TXQCON {
        fn address() -> SFRAddress {
            SFRAddress::C1TXQCON
        }
    }

    impl From<C1TXQCON> for u32 {
        fn from(reg: C1TXQCON) -> Self {
            reg.0
        }
    }
}

pub mod fifo {
    use bitfield::*;

    use crate::generic::SFRAddress;

    pub enum Mode {
        Transmit,
        Receive,
    }

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

    impl From<ControlRegister> for u32 {
        fn from(reg: ControlRegister) -> Self {
            reg.0
        }
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

    impl From<StatusRegister> for u32 {
        fn from(reg: StatusRegister) -> Self {
            reg.0
        }
    }

    bitfield! {
        pub struct UserAddressRegister(u32);
        u32;
        pub fifoua, set_fifoua: 31, 0;
    }

    impl From<UserAddressRegister> for u32 {
        fn from(reg: UserAddressRegister) -> Self {
            reg.0
        }
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
}
