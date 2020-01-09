pub mod control {
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
}
