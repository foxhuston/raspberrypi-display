use std::{error::Error, fmt::Debug, thread, time::Duration};

use local_ip_address::local_ip;

use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt, CpuExt};

use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X10, ascii::FONT_5X7},
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
            sys: System::new_all(),
        }
    }

    pub fn draw<D>(&mut self, display: &mut D) -> Result<(), D::Error>
        where D: DrawTarget<Color = BinaryColor>, <D as DrawTarget>::Error : Debug
    {
        self.sys.refresh_all();

        let cs_large = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let cs_small = MonoTextStyle::new(&FONT_5X7, BinaryColor::On);

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

        let rows = [9, 12, 22, 42];

        Text::with_text_style(host_line.as_str(), Point::zero(), cs_large, text_style)
            .draw(display)?;

        Line::new(Point::new(0, rows[0]), Point::new(128, rows[0]))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(display)?;

        Text::with_text_style(ip_line.as_str(), Point::new(10, rows[1]), cs_small, text_style)
            .draw(display)?;

        let left_margin = 4;
        let label_margin = 20;
        let bar_size = Size::new(35, 7);

        Text::with_text_style("CPU", Point::new(left_margin, rows[2]), cs_small, text_style)
            .draw(display)?;

        self.draw_bar(Point::new(label_margin + left_margin, rows[2]),
                      bar_size,
                      cpu,
                      display)?;

        let second_column = 63;

        Text::with_text_style("Mem", Point::new(second_column + left_margin, rows[2]), cs_small, text_style)
            .draw(display)?;

        self.draw_bar(Point::new(second_column + label_margin + left_margin, rows[2]),
                      bar_size,
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
