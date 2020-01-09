pub enum PLL {
    On,
    Off,
}

pub enum SysClkDivider {
    DivByOne,
    DivByTwo,
}

pub struct Oscillator {
    PLL: PLL,
    Divider: SysClkDivider,
}

pub struct Settings {
    oscillator: Oscillator,
}
