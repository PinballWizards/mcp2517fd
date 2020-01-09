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

pub struct Settings {
    pub oscillator: Oscillator,
}
