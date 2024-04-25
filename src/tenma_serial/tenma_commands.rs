pub enum TenmaCommand {
    ISet {
        channel: u8,
        current: f32,
    },
    VSet {
        channel: u8,
        voltage: f32,
    },
    Beep(bool),
}
