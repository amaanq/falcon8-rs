use rusb::UsbContext;

use crate::{Brightness, Falcon8, Flow, KeyCode, LEDMode, Layer, Mode};

#[repr(C)]
#[derive(Clone, Copy)]
pub union Report {
    bytes: [u8; 264],
    data: ReportData,
}

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct ReportData {
    pub zeroth_byte: u8,
    pub mode: Mode,
    pub active_layer: Layer,

    // --------------------
    // Keys
    // --------------------
    _s1: [u8; 5],
    pub key_one: KeyCode,
    pub key_five: KeyCode,
    pub macro_key_three: u8,
    pub macro_key_seven: u8,
    _s2: u8,
    pub key_two: KeyCode,
    pub key_six: KeyCode,
    _s3: [u8; 3],
    pub key_three: KeyCode,
    pub key_seven: KeyCode,
    _s4: [u8; 3],
    pub key_four: KeyCode,
    pub key_eight: KeyCode,

    // skip to colors
    _s5: [u8; 33],

    // --------------------
    // Colors
    // --------------------

    // Red
    pub key_one_red: u8,
    pub key_two_red: u8,
    pub key_four_red: u8,
    _s6: [u8; 2],
    pub key_five_red: u8,
    pub key_three_red: u8,
    pub key_eight_red: u8,
    _s7: [u8; 3],
    pub key_six_red: u8,
    _s8: [u8; 4],
    pub key_seven_red: u8,

    _s9: [u8; 8],

    // Green
    pub key_one_green: u8,
    pub key_two_green: u8,
    pub key_four_green: u8,
    _s10: [u8; 2],
    pub key_five_green: u8,
    pub key_three_green: u8,
    pub key_eight_green: u8,
    _s11: [u8; 3],
    pub key_six_green: u8,
    _s12: [u8; 4],
    pub key_seven_green: u8,

    _s13: [u8; 8],

    // Blue
    pub key_one_blue: u8,
    pub key_two_blue: u8,
    pub key_four_blue: u8,
    _s14: [u8; 2],
    pub key_five_blue: u8,
    pub key_three_blue: u8,
    pub key_eight_blue: u8,
    _s15: [u8; 3],
    pub key_six_blue: u8,
    _s16: [u8; 4],
    pub key_seven_blue: u8,

    // --------------------
    // LED Stuff
    // --------------------
    _s17: [u8; 8],
    pub led_mode: LEDMode,
    pub brightness: Brightness,
    pub flow: Flow,
    pub led_red: u8,
    pub led_green: u8,
    pub led_blue: u8,
}

pub const DEFAULT_REPORT: [u8; 264] = [
    0x07, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa9, 0xa4, 0xbb, 0x00, 0x00, 0xa7, 0xa6, 0xbc,
    0x00, 0x00, 0xa5, 0xa8, 0xb9, 0x00, 0x00, 0xab, 0xaa, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00,
    0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x45, 0xff, 0x00, 0x00, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0xff, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x01, 0xff, 0xff, 0xff, 0x01, 0x00, 0x05, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xc0, 0x06, 0x00, 0x10, 0xa2, 0x04, 0x00, 0x10, 0xb4, 0x04, 0x00, 0x10, 0x0d, 0x01, 0x00, 0x10,
    0x02, 0x00, 0x00, 0x00, 0x53, 0x3a, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x54, 0x09, 0x00, 0x10,
    0xc0, 0x06, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x06, 0x00, 0x10, 0x4f, 0x4b, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[allow(unused)]
pub const DEFAULT_REPORT_DATA: ReportData = ReportData {
    zeroth_byte: 0x07,
    mode: Mode::KeyRead,
    active_layer: Layer::One,

    key_one: KeyCode::Mute,
    key_two: KeyCode::PlayPause,
    key_three: KeyCode::MediaStop,
    key_four: KeyCode::VolumeUp,
    key_five: KeyCode::MediaLaunch,
    key_six: KeyCode::MediaPrevious,
    key_seven: KeyCode::MediaNext,
    key_eight: KeyCode::VolumeDown,

    key_one_red: 0,
    key_two_red: 0,
    key_three_red: 0,
    key_four_red: 0,
    key_five_red: 0,
    key_six_red: 0,
    key_seven_red: 0,
    key_eight_red: 0,

    key_one_blue: 0,
    key_two_blue: 0,
    key_three_blue: 0,
    key_four_blue: 0,
    key_five_blue: 0,
    key_six_blue: 0,
    key_seven_blue: 0,
    key_eight_blue: 0,

    key_one_green: 0,
    key_two_green: 0,
    key_three_green: 0,
    key_four_green: 0,
    key_five_green: 0,
    key_six_green: 0,
    key_seven_green: 0,
    key_eight_green: 0,

    macro_key_three: 0,
    macro_key_seven: 0,

    led_mode: LEDMode::Static,
    brightness: Brightness::Max,
    flow: Flow::Constant,
    led_red: 0,
    led_green: 0,
    led_blue: 0,

    _s1: [0; 5],
    _s2: 0,
    _s3: [0; 3],
    _s4: [0; 3],
    _s5: [0; 0x21],
    _s6: [0; 2],
    _s7: [0; 3],
    _s8: [0; 4],
    _s9: [0; 8],
    _s10: [0; 2],
    _s11: [0; 3],
    _s12: [0; 4],
    _s13: [0; 8],
    _s14: [0; 2],
    _s15: [0; 3],
    _s16: [0; 4],
    _s17: [0; 8],
};

impl Report {
    pub fn new() -> Self {
        Self { bytes: [0; 264] }
    }

    pub fn from_falcon<T: UsbContext>(falcon: &Falcon8<T>) -> Self {
        let mut report = Self { bytes: [0; 264] };
        report.data_mut().zeroth_byte = 0x07;
        report.data_mut().mode = Mode::KeyRead;
        report.data_mut().active_layer = falcon.active_layer;
        report
    }

    pub fn empty() -> Self {
        Self { bytes: [0; 264] }
    }

    pub fn as_bytes(&self) -> &[u8; 264] {
        unsafe { &self.bytes }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8; 264] {
        unsafe { &mut self.bytes }
    }

    /// Gets the report data
    pub fn data(&self) -> &ReportData {
        unsafe { &self.data }
    }

    /// Gets the report data mutably
    pub fn data_mut(&mut self) -> &mut ReportData {
        unsafe { &mut self.data }
    }

    /// Resets the report to the default state, `DEFAULT_REPORT`
    pub fn reset(&mut self) {
        self.bytes = DEFAULT_REPORT;
    }

    /// Clears the report, setting all bytes to 0 except the first 3
    pub fn clear(mut self, layer: Option<Layer>) -> Self {
        self.fill(0);
        self.data.zeroth_byte = 0x07;
        self.data.mode = Mode::KeyRead;
        self.data.active_layer = layer.unwrap_or(Layer::One);
        self
    }

    /// Clears the last 56 bytes of the report, setting them to 0, leave the rest alone (208)
    pub fn clear_end(&mut self) -> &mut Self {
        self[208..].fill(0);
        self
    }

    // First Byte

    ///  TODO: figure out what these mean, 0x07, 0x82/0x02
    pub fn first_byte(&self) -> u8 {
        self.data().zeroth_byte
    }

    // Second Byte

    pub fn mode(&self) -> Mode {
        self.data().mode
    }

    pub fn set_mode(&mut self, mode: Mode) -> &mut Self {
        self.data_mut().mode = mode;
        self
    }

    // Active Layer

    pub fn active_layer(&self) -> Layer {
        self.data().active_layer
    }

    pub fn set_active_layer(&mut self, layer: Layer) -> &mut Self {
        self.data_mut().active_layer = layer;
        self
    }
}

impl std::fmt::Debug for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Report")
            .field(
                "zeroth_byte",
                &format_args!("0x{:02X}", self.data().zeroth_byte),
            )
            .field(
                "set_get_mode",
                &format_args!("0x{:02X}", self.data().mode as u8),
            )
            .field(
                "active_layer",
                &format_args!("0x{:02X}", self.data().active_layer as u8),
            )
            .field("key_one", &self.data().key_one)
            .field("key_two", &self.data().key_two)
            .field("key_three", &self.data().key_three)
            .field("key_four", &self.data().key_four)
            .field("key_five", &self.data().key_five)
            .field("key_six", &self.data().key_six)
            .field("key_seven", &self.data().key_seven)
            .field("key_eight", &self.data().key_eight)
            .field(
                "key_one_rgb",
                &format_args!(
                    "({:02X}, {:02X}, {:02X})",
                    self.data().key_one_red,
                    self.data().key_one_green,
                    self.data().key_one_blue,
                ),
            )
            .field(
                "key_two_rgb",
                &format_args!(
                    "({:02X}, {:02X}, {:02X})",
                    self.data().key_two_red,
                    self.data().key_two_green,
                    self.data().key_two_blue,
                ),
            )
            .field(
                "key_three_rgb",
                &format_args!(
                    "({:02X}, {:02X}, {:02X})",
                    self.data().key_three_red,
                    self.data().key_three_green,
                    self.data().key_three_blue,
                ),
            )
            .field(
                "key_four_rgb",
                &format_args!(
                    "({:02X}, {:02X}, {:02X})",
                    self.data().key_four_red,
                    self.data().key_four_green,
                    self.data().key_four_blue,
                ),
            )
            .field(
                "key_five_rgb",
                &format_args!(
                    "({:02X}, {:02X}, {:02X})",
                    self.data().key_five_red,
                    self.data().key_five_green,
                    self.data().key_five_blue,
                ),
            )
            .field(
                "key_six_rgb",
                &format_args!(
                    "({:02X}, {:02X}, {:02X})",
                    self.data().key_six_red,
                    self.data().key_six_green,
                    self.data().key_six_blue,
                ),
            )
            .field(
                "key_seven_rgb",
                &format_args!(
                    "({:02X}, {:02X}, {:02X})",
                    self.data().key_seven_red,
                    self.data().key_seven_green,
                    self.data().key_seven_blue,
                ),
            )
            .field(
                "key_eight_rgb",
                &format_args!(
                    "({:02X}, {:02X}, {:02X})",
                    self.data().key_eight_red,
                    self.data().key_eight_green,
                    self.data().key_eight_blue,
                ),
            )
            .field("led_mode", &self.data().led_mode)
            .field("brightness", &self.data().brightness)
            .field("flow", &self.data().flow)
            .field(
                "global_rgb",
                &format_args!(
                    "({:02X}, {:02X}, {:02X})",
                    self.data().led_red,
                    self.data().led_green,
                    self.data().led_blue,
                ),
            )
            .finish()
    }
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data())
    }
}

impl std::fmt::Display for ReportData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ReportData {{ zeroth_byte: 0x{:02X}, set_get_mode: 0x{:02X}, active_layer: 0x{:02X}, ",
            self.zeroth_byte, self.mode as u8, self.active_layer as u8,
        )?;

        write!(
            f,
            "key_one: {}, key_two: {}, key_three: {}, key_four: {}, key_five: {}, key_six: {}, key_seven: {}, key_eight: {}, ",
            self.key_one,
            self.key_two,
            self.key_three,
            self.key_four,
            self.key_five,
            self.key_six,
            self.key_seven,
            self.key_eight,
        )?;

        write!(
            f,
            "key_one_rgb: ({:02X}, {:02X}, {:02X}), key_two_rgb: ({:02X}, {:02X}, {:02X}), ",
            self.key_one_red,
            self.key_one_green,
            self.key_one_blue,
            self.key_two_red,
            self.key_two_green,
            self.key_two_blue,
        )?;

        write!(
            f,
            "key_three_rgb: ({:02X}, {:02X}, {:02X}), key_four_rgb: ({:02X}, {:02X}, {:02X}), ",
            self.key_three_red,
            self.key_three_green,
            self.key_three_blue,
            self.key_four_red,
            self.key_four_green,
            self.key_four_blue,
        )?;

        write!(
            f,
            "key_five_rgb: ({:02X}, {:02X}, {:02X}), key_six_rgb: ({:02X}, {:02X}, {:02X}), ",
            self.key_five_red,
            self.key_five_green,
            self.key_five_blue,
            self.key_six_red,
            self.key_six_green,
            self.key_six_blue,
        )?;

        write!(
            f,
            "key_seven_rgb: ({:02X}, {:02X}, {:02X}), key_eight_rgb: ({:02X}, {:02X}, {:02X}), ",
            self.key_seven_red,
            self.key_seven_green,
            self.key_seven_blue,
            self.key_eight_red,
            self.key_eight_green,
            self.key_eight_blue,
        )?;

        write!(
            f,
            "led_mode: {:?} ({:02X}), brightness: {:?} ({:02X}), flow: {:?} ({:02X}), ",
            self.led_mode,
            self.led_mode as u8,
            self.brightness,
            self.brightness as u8,
            self.flow,
            self.flow as u8,
        )?;

        write!(
            f,
            "global_rgb: ({:02X}, {:02X}, {:02X}) }}",
            self.led_red, self.led_green, self.led_blue,
        )?;

        Ok(())
    }
}

impl Default for Report {
    fn default() -> Self {
        Self {
            bytes: DEFAULT_REPORT,
        }
    }
}

impl std::ops::Deref for Report {
    type Target = [u8; 264];

    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}

impl std::ops::DerefMut for Report {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_bytes_mut()
    }
}

impl std::convert::AsRef<[u8]> for Report {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl std::convert::AsMut<[u8]> for Report {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}

impl std::convert::From<[u8; 264]> for Report {
    fn from(bytes: [u8; 264]) -> Self {
        Self { bytes }
    }
}

impl std::convert::From<&[u8]> for Report {
    fn from(bytes: &[u8]) -> Self {
        let mut report = Report::new();
        report.as_bytes_mut().copy_from_slice(bytes);
        report
    }
}

impl std::convert::From<&mut [u8]> for Report {
    fn from(bytes: &mut [u8]) -> Self {
        let mut report = Report::new();
        report.as_bytes_mut().copy_from_slice(bytes);
        report
    }
}

impl std::convert::From<Vec<u8>> for Report {
    fn from(bytes: Vec<u8>) -> Self {
        let mut report = Report::new();
        report.as_bytes_mut().copy_from_slice(&bytes);
        report
    }
}

impl std::ops::Index<usize> for Report {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        self.as_bytes().index(index)
    }
}

impl std::ops::IndexMut<usize> for Report {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.as_bytes_mut().index_mut(index)
    }
}

impl std::ops::Index<std::ops::RangeFrom<usize>> for Report {
    type Output = [u8];

    fn index(&self, index: std::ops::RangeFrom<usize>) -> &Self::Output {
        self.as_bytes().index(index)
    }
}

impl std::ops::IndexMut<std::ops::RangeFrom<usize>> for Report {
    fn index_mut(&mut self, index: std::ops::RangeFrom<usize>) -> &mut Self::Output {
        self.as_bytes_mut().index_mut(index)
    }
}
