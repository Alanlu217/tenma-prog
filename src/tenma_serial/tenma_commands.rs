pub enum TenmaCommand {
    ISet {
        channel: u8,
        current: f64,
    },
    VSet {
        channel: u8,
        voltage: f64,
    },
    #[allow(dead_code)]
    Beep(bool),
    Out(bool),
}
