use std::{error::Error, fmt::Debug, thread, time::Duration};

use local_ip_address::local_ip;

use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle, ascii::FONT_6X10, ascii::FONT_7X13},
    pixelcolor::BinaryColor,
    primitives::{Line, PrimitiveStyle},
    draw_target::DrawTarget,
    prelude::*,
    text::{Baseline, Text, LineHeight, TextStyleBuilder},
};



#[derive(Debug)]
pub struct Dashboard {

}



impl Dashboard {
    pub fn new() -> Dashboard {
        Dashboard { }
    }

    pub fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
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
}
