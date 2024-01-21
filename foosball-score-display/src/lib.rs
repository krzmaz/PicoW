pub mod display;
pub mod graphics;

use hal::gpio::{
    Gpio0, Gpio14, Gpio15, Gpio38, Gpio39, Gpio40, Gpio41, Gpio42, Gpio45, Gpio46, Gpio47, Gpio48,
    Gpio5, Gpio6, Gpio7, Gpio8, Gpio9,
};

// convenience struct to be able to pass many pins to functions easily
pub struct Pins {
    pub gp0: Option<Gpio0>,
    pub gp5: Option<Gpio5>,
    pub gp6: Option<Gpio6>,
    pub gp7: Option<Gpio7>,
    pub gp8: Option<Gpio8>,
    pub gp9: Option<Gpio9>,
    pub gp14: Option<Gpio14>,
    pub gp15: Option<Gpio15>,
    pub gp38: Option<Gpio38>,
    pub gp39: Option<Gpio39>,
    pub gp40: Option<Gpio40>,
    pub gp41: Option<Gpio41>,
    pub gp42: Option<Gpio42>,
    pub gp45: Option<Gpio45>,
    pub gp46: Option<Gpio46>,
    pub gp47: Option<Gpio47>,
    pub gp48: Option<Gpio48>,
}

impl Pins {
    pub fn from_peripheral(pins: hal::gpio::Pins) -> Self {
        Self {
            gp0: Some(pins.gpio0),
            gp5: Some(pins.gpio5),
            gp6: Some(pins.gpio6),
            gp7: Some(pins.gpio7),
            gp8: Some(pins.gpio8),
            gp9: Some(pins.gpio9),
            gp14: Some(pins.gpio14),
            gp15: Some(pins.gpio15),
            gp38: Some(pins.gpio38),
            gp39: Some(pins.gpio39),
            gp40: Some(pins.gpio40),
            gp41: Some(pins.gpio41),
            gp42: Some(pins.gpio42),
            gp45: Some(pins.gpio45),
            gp46: Some(pins.gpio46),
            gp47: Some(pins.gpio47),
            gp48: Some(pins.gpio48),
        }
    }
}
