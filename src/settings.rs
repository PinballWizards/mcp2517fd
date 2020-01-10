use crate::can;

pub enum PLL {
    On,
    Off,
}

pub enum SysClkDivider {
    DivByOne,
    DivByTwo,
}

pub struct Oscillator {
    pub pll: PLL,
    pub divider: SysClkDivider,
}

pub struct IOConfiguration {
    pub enable_tx_standby_pin: bool,
    pub txcan_open_drain: bool,
    pub sof_on_clko: bool,
    pub interrupt_pin_open_drain: bool,
}

pub struct TxQueueConfiguration {
    pub message_priority: u8,
    pub retransmission_attempts: can::control::RetransmissionAttempts,
    pub fifo_size: u8,
    pub payload_size: can::control::PayloadSize,
}

pub struct FIFOConfiguration {
    /// 0 to 31
    pub fifo_number: u8,
    /// See can::control::C1TXQCON::highest_priority()
    pub priority: u8,
    pub payload_size: can::control::PayloadSize,
    /// 0 to 32
    pub fifo_size: u8,
    pub retry_attempt: can::control::RetransmissionAttempts,
    pub mode: can::fifo::Mode,
}

pub struct Settings<'a> {
    pub oscillator: Oscillator,
    pub ioconfiguration: IOConfiguration,
    pub txqueue: TxQueueConfiguration,
    pub fifoconfigs: &'a [FIFOConfiguration],
}
