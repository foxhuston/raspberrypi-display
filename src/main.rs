use std::{error::Error, fmt::Debug, thread, time::Duration};

#[cfg(not(feature = "embedded-graphics-simulator"))]
use rppal::i2c::I2c;

#[cfg(feature = "embedded-graphics-simulator")]
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder, SimulatorEvent};

#[cfg(not(feature = "embedded-graphics-simulator"))]
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use embedded_graphics::{
    prelude::*,
    pixelcolor::BinaryColor
};

pub mod dashboard;

#[cfg(feature = "embedded-graphics-simulator")]
fn run_display() -> Result<(), Box<dyn Error>> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 32));
    let mut dashboard = dashboard::Dashboard::new();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();

    let mut win = Window::new("Hello World", &output_settings);

    'running: loop {
        display.clear(BinaryColor::Off)?;
        dashboard.draw(&mut display)?;
        win.update(&display);

        if win.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }

        thread::sleep(Duration::from_millis(1000));
    }
}

#[cfg(not(feature = "embedded-graphics-simulator"))]
fn run_display() -> Result<(), Box<dyn Error>> {
    let i2c = I2c::new()?;

    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    let mut dashboard = dashboard::Dashboard::new();

    display.init().unwrap();

    // ctrlc::set_handler(move || break 'running Ok(()))
    //     .expect("Error setting ctrlc handler.");

    'running: loop {
        dashboard.draw(&mut display).unwrap();
        display.flush().unwrap();
        thread::sleep(Duration::from_millis(1000));
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run_display()?;

    Ok(())
}
