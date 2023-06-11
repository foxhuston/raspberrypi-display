use std::{error::Error, fmt::Debug, thread, time::Duration};

use rppal::i2c::I2c;
// use rppal::system::DeviceInfo;
use local_ip_address::local_ip;


use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle, ascii::FONT_6X10, ascii::FONT_7X13},
    pixelcolor::BinaryColor,
    primitives::{Line, PrimitiveStyle},
    draw_target::DrawTarget,
    prelude::*,
    text::{Baseline, Text, LineHeight, TextStyleBuilder},
};

#[cfg(feature = "embedded-graphics-simulator")]
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder, SimulatorEvent};

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn draw<D>(display: &mut D) -> Result<(), D::Error>
    where D: DrawTarget<Color = BinaryColor>, <D as DrawTarget>::Error : Debug
{
    let cs_large = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);
    let cs_small = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    let local_ip = local_ip().unwrap();
    let name = hostname::get().unwrap();

    let text_style = TextStyleBuilder::new()
        .baseline(Baseline::Top)
        // .line_height(LineHeight::Percent(100))
        .build();

    let host_line = format!("{}", name.to_string_lossy());
    let ip_line = format!("{:?}", local_ip);

    Text::with_text_style(host_line.as_str(), Point::zero(), cs_large, text_style)
        .draw(display)?;

    Line::new(Point::new(0, 12), Point::new(128, 12))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)?;

    Text::with_text_style(ip_line.as_str(), Point::new(10, 16), cs_small, text_style)
        .draw(display)?;

    Ok(())
}

#[cfg(feature = "embedded-graphics-simulator")]
fn run_display() -> Result<(), Box<dyn Error>> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));

    draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();

    let mut win = Window::new("Hello World", &output_settings);

    'running: loop {
        win.update(&display);
        if win.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }
        thread::sleep(Duration::from_millis(50));
    }
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
