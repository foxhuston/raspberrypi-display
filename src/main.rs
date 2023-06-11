use std::{error::Error, fmt::Debug};

use rppal::i2c::I2c;
// use rppal::system::DeviceInfo;


use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    draw_target::DrawTarget,
    prelude::*,
    text::{Baseline, Text, LineHeight, TextStyleBuilder},
};

#[cfg(feature = "embedded-graphics-simulator")]
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder};

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn draw<D>(display: &mut D) -> Result<(), D::Error>
    where D: DrawTarget<Color = BinaryColor>, <D as DrawTarget>::Error : Debug
{
    let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    let text_style = TextStyleBuilder::new()
        .line_height(LineHeight::Percent(100))
        .build();

    Text::with_text_style("Hello world!", Point::zero(), character_style, text_style)
        .draw(display)
        .unwrap();

    Text::with_text_style("Hello Rust!", Point::new(0, 16), character_style, text_style)
        .draw(display)
        .unwrap();

    Ok(())
}

#[cfg(feature = "embedded-graphics-simulator")]
fn run_display() -> Result<(), Box<dyn Error>> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));

    draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    Window::new("Hello World", &output_settings).show_static(&display);


    Ok(())
}

#[cfg(not(feature = "embedded-graphics-simulator"))]
fn run_display() -> Result<(), Box<dyn Error>> {
    let i2c = I2c::new()?;

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    draw(&mut display);

    display.flush().unwrap();

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run_display()?;

    Ok(())
}
