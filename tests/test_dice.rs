#[cfg(test)]
mod tests {
    use rstest::rstest;
    use tracing::info;

    use core::convert::Infallible;
    use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
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
        let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(255, 255));

        draw_shapes(&mut display.translated(Point::new(8, 8)))?;

        let output_settings = OutputSettingsBuilder::new().scale(1).build();
        Window::new("a die", &output_settings).show_static(&display);

        Ok(())
    }
}
