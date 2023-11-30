use num_traits::FromPrimitive;
use rusb::{Result, UsbContext};

use crate::{Falcon8, Key, Mode, Report};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum LEDMode {
    Static,
    Breathing,
    FadeIn,
    FadeOut,
    LastKeystroke,
    RGBWave,
    RGBRandom,
    Custom,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Brightness {
    Off,
    Low,
    Medium,
    High,
    Max,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Flow {
    RightToLeft,
    LeftToRight,
    TopToBottom,
    Spinning,
    Constant,
}

#[derive(Debug)]
pub struct LEDControls {
    pub mode: Option<LEDMode>,
    pub brightness: Option<Brightness>,
    pub flow: Option<Flow>,
    pub color: [u8; 3],       // only for custom mode
    pub key_colors: [u8; 24], // only for custom mode
}

impl LEDControls {
    pub fn new() -> Self {
        Self {
            mode: None,
            brightness: None,
            flow: None,
            color: [0; 3],
            key_colors: [0; 24],
        }
    }

    pub fn set_mode(&mut self, mode: LEDMode) -> &mut Self {
        self.mode = Some(mode);
        self
    }

    pub fn set_brightness(&mut self, brightness: Brightness) -> &mut Self {
        self.brightness = Some(brightness);
        self
    }

    pub fn set_flow(&mut self, flow: Flow) -> &mut Self {
        self.flow = Some(flow);
        self
    }

    /// Set the overall color of the LEDs, works with [`LEDMode::Static`], [`LEDMode::Breathing`], [`LEDMode::FadeIn`], [`LEDMode::FadeOut`], and [`LEDMode::LastKeystroke`].
    pub fn set_color(&mut self, color: (u8, u8, u8)) -> &mut Self {
        self.color = [color.0, color.1, color.2];
        self
    }

    /// Set the color of a specific key, works with [`LEDMode::Custom`].
    pub fn set_key_color(&mut self, key: Key, color: (u8, u8, u8)) -> &mut Self {
        let idx = key as usize * 3;
        self.key_colors[idx] = color.0;
        self.key_colors[idx + 1] = color.1;
        self.key_colors[idx + 2] = color.2;
        self
    }

    // Disables the color of a specific key, works with [`LEDMode::Custom`].
    pub fn disable_key_color(&mut self, key: Key) -> &mut Self {
        let idx = key as usize * 3;
        self.key_colors[idx] = 0;
        self.key_colors[idx + 1] = 0;
        self.key_colors[idx + 2] = 0;
        self
    }
}

impl Default for LEDControls {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: UsbContext> Falcon8<T> {
    fn set_leds_in_report(&mut self, report: &mut Report) -> Result<()> {
        // sanity checks
        // if self.color isnt zeroes and self.mode isnt custom, error
        // TODO: custom err
        if self.led_controls.color != [0; 3] && self.led_controls.mode != Some(LEDMode::Custom) {
            return Err(rusb::Error::InvalidParam);
        }

        // if self.key_colors isnt zeroes and self.mode isnt custom, error
        if self.led_controls.key_colors != [0; 24]
            && self.led_controls.mode != Some(LEDMode::Custom)
        {
            return Err(rusb::Error::InvalidParam);
        }

        if let Some(mode) = self.led_controls.mode {
            report.data_mut().led_mode = mode;
        }

        if let Some(brightness) = self.led_controls.brightness {
            report.data_mut().brightness = brightness;
        }

        if let Some(flow) = self.led_controls.flow {
            report.data_mut().flow = flow;
        }

        if self.led_controls.color != [0; 3] {
            report.data_mut().led_red = self.led_controls.color[0];
            report.data_mut().led_green = self.led_controls.color[1];
            report.data_mut().led_blue = self.led_controls.color[2];
        }

        if self.led_controls.key_colors != [0; 24] {
            for (i, rgb) in self.led_controls.key_colors.chunks_exact(3).enumerate() {
                if rgb != [0; 3] {
                    report.set_key_color(Key::from_u8(i as u8).unwrap(), (rgb[0], rgb[1], rgb[2]));
                }
            }
        }

        Ok(())
    }

    pub fn update_leds(&mut self) -> Result<()> {
        let mut report = Report::new();
        report.data_mut().zeroth_byte = 0x07;
        report.data_mut().mode = Mode::KeyRead;
        report.data_mut().active_layer = self.active_layer;

        self.set_report(&report)?;

        self.get_report(&mut report)?;

        report.set_mode(Mode::KeyWrite).clear_end();
        self.set_leds_in_report(&mut report)?;
        self.set_report(&report)?;

        Ok(())
    }
}
