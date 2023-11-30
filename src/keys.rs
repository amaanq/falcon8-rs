use num_derive::FromPrimitive;
use rusb::{Result, UsbContext};

use crate::{keycode::KeyCode, Falcon8, Mode, Report};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
pub enum Key {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

#[derive(Debug)]
pub struct KeyControl {
    pub key: Key,
    pub key_code: KeyCode,
}

#[derive(Debug)]
pub struct KeyControls {
    pub keys: [KeyControl; 8],
}

impl Key {
    pub fn to_index(&self) -> usize {
        match self {
            Key::One => 0x08,
            Key::Five => 0x09,
            Key::Two => 0x0D,
            Key::Six => 0x0E,
            Key::Three => 0x12,
            Key::Seven => 0x13,
            Key::Four => 0x17,
            Key::Eight => 0x18,
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0x08 => Some(Key::One),
            0x09 => Some(Key::Five),
            0x0D => Some(Key::Two),
            0x0E => Some(Key::Six),
            0x12 => Some(Key::Three),
            0x13 => Some(Key::Seven),
            0x17 => Some(Key::Four),
            0x18 => Some(Key::Eight),
            _ => None,
        }
    }

    pub fn to_color_indices(&self) -> (usize, usize, usize) {
        match self {
            Key::One => (0x3A, 0x53, 0x6C),
            Key::Two => (0x3B, 0x54, 0x6D),
            Key::Three => (0x40, 0x59, 0x72),
            Key::Four => (0x3C, 0x55, 0x6E),
            Key::Five => (0x3F, 0x58, 0x71),
            Key::Six => (0x45, 0x5E, 0x77),
            Key::Seven => (0x4A, 0x63, 0x7C),
            Key::Eight => (0x41, 0x5A, 0x73),
        }
    }

    pub fn to_macro_index(&self) -> usize {
        match self {
            Key::One => 0x00,
            Key::Five => 0x01,
            Key::Two => 0x05,
            Key::Six => 0x06,
            Key::Three => 0x0A,
            Key::Seven => 0x0B,
            Key::Four => 0x0F,
            Key::Eight => 0x10,
        }
    }
}

impl KeyControls {
    pub fn new() -> Self {
        Self {
            keys: [
                KeyControl {
                    key: Key::One,
                    key_code: KeyCode::Mute,
                },
                KeyControl {
                    key: Key::Two,
                    key_code: KeyCode::PlayPause,
                },
                KeyControl {
                    key: Key::Three,
                    key_code: KeyCode::MediaStop,
                },
                KeyControl {
                    key: Key::Four,
                    key_code: KeyCode::VolumeUp,
                },
                KeyControl {
                    key: Key::Five,
                    key_code: KeyCode::MediaLaunch,
                },
                KeyControl {
                    key: Key::Six,
                    key_code: KeyCode::MediaPrevious,
                },
                KeyControl {
                    key: Key::Seven,
                    key_code: KeyCode::MediaNext,
                },
                KeyControl {
                    key: Key::Eight,
                    key_code: KeyCode::VolumeDown,
                },
            ],
        }
    }

    pub fn set_key(&mut self, key: Key, key_code: KeyCode) {
        self.keys[key as usize].key_code = key_code;
    }
}

impl Default for KeyControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Report {
    pub fn set_key(&mut self, key: Key, key_code: KeyCode) {
        match key {
            Key::One => self.data_mut().key_one = key_code,
            Key::Two => self.data_mut().key_two = key_code,
            Key::Three => self.data_mut().key_three = key_code,
            Key::Four => self.data_mut().key_four = key_code,
            Key::Five => self.data_mut().key_five = key_code,
            Key::Six => self.data_mut().key_six = key_code,
            Key::Seven => self.data_mut().key_seven = key_code,
            Key::Eight => self.data_mut().key_eight = key_code,
        }
    }

    pub fn set_key_color(&mut self, key: Key, color: (u8, u8, u8)) {
        match key {
            Key::One => {
                self.data_mut().key_one_red = color.0;
                self.data_mut().key_one_green = color.1;
                self.data_mut().key_one_blue = color.2;
            }
            Key::Two => {
                self.data_mut().key_two_red = color.0;
                self.data_mut().key_two_green = color.1;
                self.data_mut().key_two_blue = color.2;
            }
            Key::Three => {
                self.data_mut().key_three_red = color.0;
                self.data_mut().key_three_green = color.1;
                self.data_mut().key_three_blue = color.2;
            }
            Key::Four => {
                self.data_mut().key_four_red = color.0;
                self.data_mut().key_four_green = color.1;
                self.data_mut().key_four_blue = color.2;
            }
            Key::Five => {
                self.data_mut().key_five_red = color.0;
                self.data_mut().key_five_green = color.1;
                self.data_mut().key_five_blue = color.2;
            }
            Key::Six => {
                self.data_mut().key_six_red = color.0;
                self.data_mut().key_six_green = color.1;
                self.data_mut().key_six_blue = color.2;
            }
            Key::Seven => {
                self.data_mut().key_seven_red = color.0;
                self.data_mut().key_seven_green = color.1;
                self.data_mut().key_seven_blue = color.2;
            }
            Key::Eight => {
                self.data_mut().key_eight_red = color.0;
                self.data_mut().key_eight_green = color.1;
                self.data_mut().key_eight_blue = color.2;
            }
        }
    }

    // pub fn set_macro(report: &mut Report, key: Key, macro_index: u8) {
    //     let index = key.to_macro_index();
    //     report[index] = macro_index;
    // }
}

impl<T: UsbContext> Falcon8<T> {
    pub fn get_keys(&mut self) -> Result<Report> {
        let mut report = Report::new();
        report.data_mut().zeroth_byte = 0x07;
        report.data_mut().mode = Mode::KeyRead;
        report.data_mut().active_layer = self.active_layer;

        self.set_report(&report)?;
        self.get_report(&mut report)?;

        Ok(report)
    }

    pub fn update_keys(&mut self) -> Result<()> {
        let mut report = Report::new();
        report.data_mut().zeroth_byte = 0x07;
        report.data_mut().mode = Mode::KeyRead;
        report.data_mut().active_layer = self.active_layer;

        self.set_report(&report)?;

        self.get_report(&mut report)?;

        report.set_mode(Mode::KeyWrite).clear_end();
        for key_control in self.key_controls.keys.iter() {
            if key_control.key_code == KeyCode::Disable {
                continue; // TODO: do we want to allow disabling?
            }
            report.set_key(key_control.key, key_control.key_code);
        }

        self.set_report(&report)?;

        report.fill(0);
        report.data_mut().zeroth_byte = 0x07;
        report.data_mut().mode = Mode::Finalize;
        self.set_report(&report)?;

        Ok(())
    }
}
