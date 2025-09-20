#[cfg(test)]
mod tests {
    use rstest::rstest;
    use tracing::info;

    use core::convert::Infallible;
    use embedded_graphics::{
        pixelcolor::Rgb888,
        prelude::*,
        primitives::{PrimitiveStyle, PrimitiveStyleBuilder},
    };
    use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
    use pico_display::dice::draw_shapes;

    #[rstest]
    #[test_log::test]
    fn test_hello_word() -> () {
        info!("Hello World!");
    }

    #[rstest]
    #[test_log::test]
    fn test_draw_dice() -> Result<(), Infallible> {
        let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(325, 220));

        let stroke = PrimitiveStyle::with_stroke(Rgb888::MAGENTA, 1);

        let stroke_off_fill_off = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::RED)
            .stroke_width(1)
            .fill_color(Rgb888::GREEN)
            .build();

        let stroke_off_fill_on = PrimitiveStyle::with_fill(Rgb888::YELLOW);

        draw_shapes(&mut display.translated(Point::new(8, 8)), stroke)?;
        draw_shapes(
            &mut display.translated(Point::new(24, 24)),
            stroke_off_fill_on,
        )?;
        draw_shapes(
            &mut display.translated(Point::new(40, 40)),
            stroke_off_fill_off,
        )?;

        let output_settings = OutputSettingsBuilder::new().scale(1).build();
        Window::new("Filled primitives", &output_settings).show_static(&display);

        Ok(())
    }
}
