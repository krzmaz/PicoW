use std::{thread::sleep, time::Duration};

use esp_idf_svc::hal::{gpio::PinDriver, peripherals::Peripherals};
use foosball_score_display::{
    display::LilyGoDisplay,
    graphics::{draw_blue_score, draw_initial_state, draw_red_score},
    Pins,
};

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    let mut pins = Pins::from_peripheral(peripherals.pins);

    // left looking in the orientation of the "LILYGO" text above the screen.
    let left_button = PinDriver::input(pins.gp0.take().unwrap())?;
    let right_button = PinDriver::input(pins.gp14.take().unwrap())?;

    let mut display = LilyGoDisplay::new(pins, peripherals.ledc);
    display.set_brightness(40);
    draw_initial_state(&mut display);
    let mut blue_score = 0;
    let mut red_score = 0;

    // the time it takes to draw the scores acts as the button debouncing delay
    loop {
        if left_button.is_low() {
            log::info!("Left pressed!");
            red_score += 1;
            draw_red_score(red_score, &mut display);
        }
        if right_button.is_low() {
            log::info!("Right pressed!");
            blue_score += 1;
            draw_blue_score(blue_score, &mut display);
        }
        sleep(Duration::from_millis(10));
    }
    // Ok(())
}
