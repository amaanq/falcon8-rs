use std::time::Duration;

use rusb::{Context, Device, DeviceHandle, Direction, Recipient, RequestType, Result, UsbContext};

mod consts;
mod keycode;
mod keys;
mod layers;
mod led;
mod r#macro;
mod mode;
mod report;
mod tracing;

pub use consts::*;
pub use keycode::KeyCode;
pub use keys::{Key, KeyControl, KeyControls};
pub use layers::Layer;
pub use led::{Brightness, Flow, LEDControls, LEDMode};
pub use mode::Mode;
pub use report::Report;
pub use tracing::debug_report;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
}

#[derive(Debug)]
pub struct Falcon8<T: UsbContext> {
    pub device: Device<T>,
    pub handle: DeviceHandle<T>,
    pub interfaces: Vec<u8>,

    pub active_layer: Layer,
    pub led_controls: LEDControls,
    pub key_controls: KeyControls,
}

impl Falcon8<Context> {
    pub fn new() -> Result<Vec<Self>> {
        let mut context = Context::new()?;
        let devices = Self::open_devices(&mut context, VID, PID)?;

        if devices.is_empty() {
            return Err(rusb::Error::NotFound);
        }

        Ok(devices)
    }
}

impl<T: UsbContext> Falcon8<T> {
    fn open_devices(context: &mut T, vid: u16, pid: u16) -> Result<Vec<Falcon8<T>>> {
        let devices = context.devices()?;
        let mut result = Vec::new();

        for device in devices.iter() {
            let Ok(device_desc) = device.device_descriptor() else {
                continue;
            };

            if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
                if let Ok(mut handle) = device.open() {
                    handle.set_auto_detach_kernel_driver(true)?;

                    let mut falcon8 = Falcon8 {
                        device,
                        handle,
                        interfaces: Vec::new(),

                        active_layer: Layer::One,
                        led_controls: LEDControls::default(),
                        key_controls: KeyControls::default(),
                    };

                    falcon8.claim_interfaces()?;

                    result.push(falcon8);
                }
            }
        }

        Ok(result)
    }

    pub fn claim_interfaces(&mut self) -> Result<()> {
        let config_desc = self.device.config_descriptor(0)?;
        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    if endpoint_desc.direction() == Direction::In {
                        self.handle
                            .claim_interface(interface_desc.interface_number())?;
                        self.interfaces.push(interface_desc.interface_number());
                    }
                }
            }
        }

        Ok(())
    }

    pub fn print_device_info(&self) -> Result<()> {
        let device_desc = self.handle.device().device_descriptor()?;
        let timeout = std::time::Duration::from_secs(1);
        let languages = self.handle.read_languages(timeout)?;

        println!(
            "Active configuration: {}",
            self.handle.active_configuration()?
        );

        if !languages.is_empty() {
            let language = languages[0];

            println!(
                "Manufacturer: {}",
                self.handle
                    .read_manufacturer_string(language, &device_desc, timeout)
                    .unwrap_or_else(|_| "Not Found".to_string())
            );
            println!(
                "Product: {}",
                self.handle
                    .read_product_string(language, &device_desc, timeout)
                    .unwrap_or_else(|_| "Not Found".to_string())
            );
        }
        Ok(())
    }

    pub fn find_readable_endpoints(&self) -> Result<Vec<Endpoint>> {
        let config_desc = self.device.config_descriptor(0)?;
        let mut endpoints = vec![];

        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    endpoints.push(Endpoint {
                        config: config_desc.number(),
                        iface: interface_desc.interface_number(),
                        setting: interface_desc.setting_number(),
                        address: endpoint_desc.address(),
                    });
                }
            }
        }

        Ok(endpoints)
    }

    pub fn get_report(&mut self, report: &mut Report) -> Result<()> {
        self.claim_interfaces()?;

        let size = self.handle.read_control(
            rusb::request_type(Direction::In, RequestType::Class, Recipient::Interface),
            0x01,
            0x0307,
            0x0002,
            report.as_mut(),
            Duration::from_secs(1),
        )?;
        assert_eq!(size, 264);
        std::thread::sleep(Duration::from_millis(50));

        #[cfg(feature = "tracing")]
        debug_report(report, false);

        Ok(())
    }

    pub fn set_report(&mut self, report: &Report) -> Result<()> {
        self.claim_interfaces()?;

        let size = self.handle.write_control(
            rusb::request_type(Direction::Out, RequestType::Class, Recipient::Interface),
            0x09,
            0x0307,
            0x0002,
            report.as_ref(),
            Duration::from_secs(1),
        )?;
        assert_eq!(size, 264);
        std::thread::sleep(Duration::from_millis(50));

        #[cfg(feature = "tracing")]
        debug_report(report, true);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use num_traits::FromPrimitive;

    #[cfg(test)]
    #[ctor::ctor]
    fn init() {
        std::env::set_var("RUST_LOG", "debug");
    }

    #[test_log::test]
    fn test_get_report() -> Result<()> {
        let mut falcon = Falcon8::new().unwrap().remove(0);
        falcon.get_keys().map(|_| ())
    }

    #[test_log::test]
    fn test_update_layer() -> Result<()> {
        let mut falcon = Falcon8::new().unwrap().remove(0);

        for layer in 1..=5 {
            falcon.update_layer(Layer::from_u8(layer).unwrap())?;
            std::thread::sleep(Duration::from_millis(500));
        }

        falcon.update_layer(Layer::One)
    }

    #[test_log::test]
    fn test_update_leds() -> Result<()> {
        let mut falcon = Falcon8::new().unwrap().remove(0);
        falcon
            .led_controls
            .set_mode(LEDMode::Static)
            .set_brightness(Brightness::Max);
        // falcon
        //     .led_controls
        //     .set_mode(LEDMode::Custom)
        //     .set_brightness(Brightness::Max)
        //     .set_key_color(Key::One, (0, 0, 255))
        //     .set_key_color(Key::Two, (0, 255, 0))
        //     .set_key_color(Key::Three, (255, 0, 0))
        //     .set_key_color(Key::Four, (255, 255, 0))
        //     .set_key_color(Key::Five, (255, 0, 255))
        //     .set_key_color(Key::Six, (0, 255, 255))
        //     .set_key_color(Key::Seven, (255, 255, 255))
        //     .set_key_color(Key::Eight, (0, 0, 0));
        falcon.update_leds()
    }

    #[test_log::test]
    fn test_update_keys() -> Result<()> {
        let mut falcon = Falcon8::new().unwrap().remove(0);
        let mut report = Report::new();
        report.data_mut().zeroth_byte = 0x07;
        report.data_mut().mode = Mode::KeyRead;
        report.data_mut().active_layer = falcon.active_layer;

        falcon
            .led_controls
            .set_mode(LEDMode::Breathing)
            .set_brightness(Brightness::Max);
        falcon.update_leds()?;

        falcon.key_controls.set_key(Key::One, KeyCode::KPAsterisk);
        falcon.key_controls.set_key(Key::Five, KeyCode::KPSlash);
        falcon
            .key_controls
            .set_key(Key::Seven, KeyCode::MediaPrevious);
        falcon.key_controls.set_key(Key::Six, KeyCode::MediaNext);
        falcon.update_keys()
    }
}
