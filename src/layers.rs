use num_derive::FromPrimitive;
use rusb::{Result, UsbContext};

use crate::{Falcon8, Mode, Report};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
pub enum Layer {
    One = 1,
    Two,
    Three,
    Four,
    Five,
}

impl<T: UsbContext> Falcon8<T> {
    pub fn update_layer(&mut self, layer: Layer) -> Result<()> {
        let mut report = Report::new();
        report.data_mut().zeroth_byte = 0x07;
        report.data_mut().mode = Mode::LayerWrite;
        report.data_mut().active_layer = layer;
        self.active_layer = layer;
        self.set_report(&report)
    }
}
