use num_derive::FromPrimitive;
use rusb::{Result, UsbContext};

use crate::{Falcon8, KeyCode, Mode, Report};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
#[repr(u16)]
pub enum Repetition {
    UntilNextKeyPressed = 0x0000,
    WhilePressed = 0xFFFF,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
#[repr(u16)]
pub enum KeyPress {
    Up = 0x0000,
    Down = 0x8000,
}

impl Repetition {
    pub fn as_bytes(&self) -> [u8; 2] {
        [(*self as u16 >> 8) as u8, (*self as u16 & 0xff) as u8]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MacroData {
    pub key_press: KeyPress,
    pub delay: u16,
    pub key_code: KeyCode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Macro {
    pub repetition: Repetition,
    pub data: Vec<MacroData>,
}

impl MacroData {
    pub fn as_bytes(&self) -> [u8; 3] {
        //  0 0 0 0 0 0 0 0  0 0 0 0 0 0 0 0  0 0 0 0 0 0 0 0 0 <- HID Key Code
        //  1 1 1 1 1 1 1 1  1 1 1 1 1 1 1 1
        //  _ _ _ _ _ _ _ _  _ _ _ _ _ _ _ _  Last 15 bits are Delay as a factor of 10ms in big endian format (weird)
        //  🡅
        //  KeyPress bit 1 = DOWN 0 = UP

        [
            (self.delay >> 8) as u8 | (self.key_press as u16 >> 8) as u8,
            self.delay as u8,
            self.key_code as u8,
        ]
    }
}

impl Macro {
    /// Max limit of 240 inputs for any macro
    pub fn add_macro_data(&mut self, macro_data: MacroData) -> Result<()> {
        if self.data.len() >= 240 {
            return Err(rusb::Error::InvalidParam);
        }

        self.data.push(macro_data);

        Ok(())
    }

    /// Take each input in the macro and convert it to its byte array, first 2 bytes are the repetition then each input represented as 3 bytes.
    //
    /// The falcon 8 has 3 packets sending the macro, each packet is 264 bytes but has an 8 byte header so there are 256 bytes left effectively.
    /// However, the first packet also has the repetition packed in 2 bytes, so we need to subtract 2 bytes from the total (first packet has 254 usable bytes..)
    /// So, this gives us 766 bytes to use, which if divided into 3 for each input gives us 255 inputs, but the Falcon-8 software only permits a max of 240 so for safety we'll use 240.
    /// Note: returns an array of byte arrays of length 3 (3 packets)
    pub fn to_bytes(&self) -> [Vec<u8>; 3] {
        let mut b = [Vec::new(), Vec::new(), Vec::new()];

        let mut index = 0;

        b[index].extend_from_slice(&self.repetition.as_bytes());

        for macro_data in &self.data {
            let bytes = macro_data.as_bytes();

            if b[index].len() >= 256 {
                index += 1;
            }
            b[index].push(bytes[0]);

            if b[index].len() >= 256 {
                index += 1;
            }
            b[index].push(bytes[1]);

            if b[index].len() >= 256 {
                index += 1;
            }
            b[index].push(bytes[2]);
        }

        b
    }
}

impl<T: UsbContext> Falcon8<T> {
    pub fn update_macros(&mut self) -> Result<()> {
        let mut report = Report::from_falcon(self);

        self.set_report(&report)?;

        self.get_report(&mut report)?;

        report.set_mode(Mode::KeyWrite).clear_end();
        self.set_report(&report)?;

        // send the 4 macro frames

        Ok(())
    }
}
