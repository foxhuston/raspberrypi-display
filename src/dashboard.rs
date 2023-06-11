use std::{error::Error, fmt::Debug, thread, time::Duration};

use local_ip_address::local_ip;

use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt, CpuExt};

use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X10, ascii::FONT_7X13},
    pixelcolor::BinaryColor,
    primitives::*,
    draw_target::DrawTarget,
    prelude::*,
    text::{Baseline, Text, LineHeight, TextStyleBuilder},
};



#[derive(Debug)]
pub struct Dashboard {
    sys: System,
}


impl Dashboard {
    pub fn new() -> Dashboard {
        Dashboard {
            sys: System::new(),
        }
    }

    pub fn draw<D>(&mut self, display: &mut D) -> Result<(), D::Error>
        where D: DrawTarget<Color = BinaryColor>, <D as DrawTarget>::Error : Debug
    {
        self.sys.refresh_cpu();

        let cs_large = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);
        let cs_small = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

        let local_ip = local_ip().unwrap();

        let text_style = TextStyleBuilder::new()
            .baseline(Baseline::Top)
            .build();

        let host_line = format!("{}", self.sys.host_name().unwrap());
        let ip_line = format!("{:?}", local_ip);

        let cpu: f32 =
            (self.sys.cpus().into_iter()
                           .map(|cpu| cpu.cpu_usage())
                           .sum::<f32>()) / self.sys.cpus().len() as f32 / 100.0;

        Text::with_text_style(host_line.as_str(), Point::zero(), cs_large, text_style)
            .draw(display)?;

        Line::new(Point::new(0, 12), Point::new(128, 12))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(display)?;

        Text::with_text_style(ip_line.as_str(), Point::new(10, 16), cs_small, text_style)
            .draw(display)?;

        let left_margin = 4;

        Text::with_text_style("CPU", Point::new(left_margin, 32), cs_small, text_style)
            .draw(display)?;

        self.draw_bar(Point::new(25 + left_margin, 33),
                      Size::new(90, 7),
                      cpu,
                      display)?;

        Text::with_text_style("Mem", Point::new(left_margin, 42), cs_small, text_style)
            .draw(display)?;

        self.draw_bar(Point::new(25 + left_margin, 43),
                      Size::new(90, 7),
                      (self.sys.used_memory() as f32) / (self.sys.total_memory() as f32),
                      display)?;

        Ok(())
    }

    pub fn draw_bar<D>(&mut self, top_left: Point, size: Size, percent: f32, display: &mut D) -> Result<(), D::Error>
        where D: DrawTarget<Color = BinaryColor>, <D as DrawTarget>::Error : Debug {

        let outline_style = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .stroke_width(1)
            .build();

        let fill_style = PrimitiveStyleBuilder::new()
            .fill_color(BinaryColor::On)
            .stroke_width(1)
            .build();

        // Draw filled portion
        Rectangle::new(top_left, Size::new((size.width as f32 * percent) as u32, size.height))
            .into_styled(fill_style)
            .draw(display)?;

        // Outline over the top
        Rectangle::new(top_left, size)
            .into_styled(outline_style)
            .draw(display)?;

        Ok(())
    }
}
