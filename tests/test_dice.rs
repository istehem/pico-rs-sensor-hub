#[cfg(test)]
mod tests {
    use rstest::fixture;
    use rstest::rstest;

    use core::convert::Infallible;
    use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
    use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
    use pico_display::dice;

    type Display = SimulatorDisplay<Rgb888>;

    #[fixture]
    #[once]
    fn shared_display() -> Display {
        SimulatorDisplay::new(Size::new(255, 255))
    }

    #[fixture]
    fn shared_display_clone(#[from(shared_display)] display: &Display) -> Display {
        display.clone()
    }

    /*
    #[rstest]
    #[test_log::test]
    fn test_draw_dice_one(
        #[from(shared_display_clone)] mut display: Display,
    ) -> Result<(), Infallible> {
        dice::draw_one(&mut display.translated(Point::new(8, 8)), 255 - 8 - 8)?;

        let output_settings = OutputSettingsBuilder::new().scale(1).build();
        Window::new("a die", &output_settings).show_static(&display);

        Ok(())
    }
    */

    #[rstest]
    #[test_log::test]
    fn test_draw_dice_two(
        #[from(shared_display_clone)] mut display: Display,
    ) -> Result<(), Infallible> {
        dice::draw_two(&mut display.translated(Point::new(8, 8)), 255 - 8 - 8)?;

        let output_settings = OutputSettingsBuilder::new().scale(1).build();
        Window::new("a die", &output_settings).show_static(&display);

        Ok(())
    }
}
