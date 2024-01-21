use display_interface_parallel_gpio::{Generic8BitBus, PGPIO8BitInterface};
use esp_idf_svc::hal::prelude::*;
use hal::{
    delay,
    gpio::PinDriver,
    ledc::{config, LedcDriver, LedcTimerDriver, LEDC},
};
use mipidsi::{ColorInversion, ModelOptions, Orientation};

use crate::Pins;

pub type Display = mipidsi::Display<
    PGPIO8BitInterface<
        Generic8BitBus<
            PinDriver<'static, hal::gpio::Gpio39, hal::gpio::Output>,
            PinDriver<'static, hal::gpio::Gpio40, hal::gpio::Output>,
            PinDriver<'static, hal::gpio::Gpio41, hal::gpio::Output>,
            PinDriver<'static, hal::gpio::Gpio42, hal::gpio::Output>,
            PinDriver<'static, hal::gpio::Gpio45, hal::gpio::Output>,
            PinDriver<'static, hal::gpio::Gpio46, hal::gpio::Output>,
            PinDriver<'static, hal::gpio::Gpio47, hal::gpio::Output>,
            PinDriver<'static, hal::gpio::Gpio48, hal::gpio::Output>,
        >,
        PinDriver<'static, hal::gpio::Gpio7, hal::gpio::Output>,
        PinDriver<'static, hal::gpio::Gpio8, hal::gpio::Output>,
    >,
    mipidsi::models::ST7789,
    PinDriver<'static, hal::gpio::Gpio5, hal::gpio::Output>,
>;

pub struct LilyGoDisplay {
    display: Display,
    // we keep those in the struct as HAL disables the pins in the Drop function and we need them set to specific levels
    _cs: PinDriver<'static, hal::gpio::Gpio6, hal::gpio::Output>,
    _rd: PinDriver<'static, hal::gpio::Gpio9, hal::gpio::Output>,
    // enables lcd power on battery
    lcd_power: PinDriver<'static, hal::gpio::Gpio15, hal::gpio::Output>,
    backlight: LedcDriver<'static>,
}

impl std::ops::DerefMut for LilyGoDisplay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.display
    }
}

impl std::ops::Deref for LilyGoDisplay {
    type Target = Display;

    fn deref(&self) -> &Self::Target {
        &self.display
    }
}
const WIDTH: u16 = 170;
const HEIGHT: u16 = 320;

// LilyGo display has some offset
pub fn offset_handler(options: &ModelOptions) -> (u16, u16) {
    match options.orientation() {
        Orientation::Portrait(false) => (35, 0),
        Orientation::Portrait(true) => (35, 0),
        Orientation::Landscape(false) => (0, 35),
        Orientation::Landscape(true) => (0, 35),
        Orientation::PortraitInverted(false) => (35, 0),
        Orientation::PortraitInverted(true) => (35, 0),
        Orientation::LandscapeInverted(false) => (0, 35),
        Orientation::LandscapeInverted(true) => (0, 35),
    }
}

impl LilyGoDisplay {
    pub fn new(mut pins: Pins, ledc: LEDC) -> Self {
        let dc = PinDriver::output(pins.gp7.take().unwrap()).unwrap();
        let mut cs = PinDriver::output(pins.gp6.take().unwrap()).unwrap();
        let rst = PinDriver::output(pins.gp5.take().unwrap()).unwrap();
        let wr = PinDriver::output(pins.gp8.take().unwrap()).unwrap();
        let mut rd = PinDriver::output(pins.gp9.take().unwrap()).unwrap();
        let lcd_power = PinDriver::output(pins.gp15.take().unwrap()).unwrap();
        let backlight = LedcDriver::new(
            ledc.channel0,
            LedcTimerDriver::new(
                ledc.timer0,
                &config::TimerConfig::new().frequency(25.kHz().into()),
            )
            .unwrap(),
            pins.gp38.take().unwrap(),
        )
        .unwrap();

        // IMPORTANT! these must be set BEFORE constructing the display struct!
        // R.I.P. 3 hours of my life :C
        // set to low to enable display
        cs.set_low().unwrap();
        // set to high when not in use
        rd.set_high().unwrap();

        let d0 = PinDriver::output(pins.gp39.take().unwrap()).unwrap();
        let d1 = PinDriver::output(pins.gp40.take().unwrap()).unwrap();
        let d2 = PinDriver::output(pins.gp41.take().unwrap()).unwrap();
        let d3 = PinDriver::output(pins.gp42.take().unwrap()).unwrap();
        let d4 = PinDriver::output(pins.gp45.take().unwrap()).unwrap();
        let d5 = PinDriver::output(pins.gp46.take().unwrap()).unwrap();
        let d6 = PinDriver::output(pins.gp47.take().unwrap()).unwrap();
        let d7 = PinDriver::output(pins.gp48.take().unwrap()).unwrap();

        let bus = Generic8BitBus::new((d0, d1, d2, d3, d4, d5, d6, d7)).unwrap();

        let di = PGPIO8BitInterface::new(bus, dc, wr);
        let display = mipidsi::Builder::st7789(di)
            .with_display_size(WIDTH, HEIGHT)
            .with_framebuffer_size(WIDTH, HEIGHT)
            .with_window_offset_handler(offset_handler)
            .with_orientation(Orientation::PortraitInverted(false))
            .with_invert_colors(ColorInversion::Inverted)
            .init(&mut delay::FreeRtos, Some(rst))
            .unwrap();

        Self {
            display,
            _cs: cs,
            _rd: rd,
            lcd_power,
            backlight,
        }
    }

    pub fn set_brightness(&mut self, value: u8) {
        self.backlight.set_duty(value as u32).unwrap();
        self.lcd_power.set_level((value > 0).into()).unwrap();
    }
}
