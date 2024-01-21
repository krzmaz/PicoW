use eg_seven_segment::{SevenSegmentStyle, SevenSegmentStyleBuilder};
use embedded_graphics::{
    primitives::{Primitive, PrimitiveStyle, PrimitiveStyleBuilder, StyledDrawable},
    text::{Alignment, Baseline, Text, TextStyle, TextStyleBuilder},
};
use embedded_graphics_core::{
    geometry::{Dimensions, Point, Size},
    pixelcolor::{Rgb565, RgbColor},
    primitives::Rectangle,
    Drawable,
};

fn text_style() -> TextStyle {
    TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .baseline(Baseline::Middle)
        .build()
}

fn char_style() -> SevenSegmentStyle<Rgb565> {
    SevenSegmentStyleBuilder::new()
        .digit_size(Size::new(60, 120))
        .digit_spacing(10)
        .segment_width(16)
        .segment_color(RgbColor::WHITE)
        .build()
}

pub fn draw_blue_score(score: u8, display: &mut crate::display::Display) {
    let width = display.bounding_box().size.width;
    let score_str = format!("{:02}", score);

    Rectangle::new(Point::new(0, 20), Size::new(width, 150))
        .draw_styled(&PrimitiveStyle::with_fill(RgbColor::BLUE), display)
        .unwrap();

    Text::with_text_style(
        &score_str,
        Point::new(width as i32 / 2, 95),
        char_style(),
        text_style(),
    )
    .draw(display)
    .unwrap();
}

pub fn draw_red_score(score: u8, display: &mut crate::display::Display) {
    let width = display.bounding_box().size.width;
    let score_str = format!("{:02}", score);

    Rectangle::new(Point::new(0, 170), Size::new(width, 150))
        .into_styled(PrimitiveStyle::with_fill(RgbColor::RED))
        .draw(&mut *display)
        .unwrap();
    Text::with_text_style(
        &score_str,
        Point::new(width as i32 / 2, 95 + 150),
        char_style(),
        text_style(),
    )
    .draw(&mut *display)
    .unwrap();
}

pub fn draw_initial_state(display: &mut crate::display::Display) {
    let bounding_box = display.bounding_box();
    let width = bounding_box.size.width;
    // Draw "button labels"
    Rectangle::new(bounding_box.top_left, Size::new(width / 2, 20))
        .draw_styled(
            &PrimitiveStyleBuilder::new()
                .stroke_width(2)
                .stroke_alignment(embedded_graphics::primitives::StrokeAlignment::Inside)
                .stroke_color(RgbColor::BLACK)
                .fill_color(RgbColor::BLUE)
                .build(),
            &mut *display,
        )
        .unwrap();
    Rectangle::new(Point::new(width as i32 / 2, 0), Size::new(width / 2, 20))
        .draw_styled(
            &PrimitiveStyleBuilder::new()
                .stroke_width(2)
                .stroke_alignment(embedded_graphics::primitives::StrokeAlignment::Inside)
                .stroke_color(RgbColor::BLACK)
                .fill_color(RgbColor::RED)
                .build(),
            &mut *display,
        )
        .unwrap();

    draw_blue_score(0, display);
    draw_red_score(0, display);
}
