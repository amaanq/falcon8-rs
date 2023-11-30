#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    LayerWrite = 0x01,
    KeyWrite = 0x02,
    MacroWrite = 0x05,
    Finalize = 0x06,

    KeyRead = Mode::KeyWrite as u8 | 0x80,
}
