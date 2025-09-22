#[cfg(test)]
mod tests {
    use rand;
    use rstest::{fixture, rstest};

    use pico_display::player;

    use core::convert::Infallible;
    use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
    use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

    const PADDING: i32 = 8;
    const SCREEN_WIDTH: u32 = 255;
    const SCREEN_HEIGHT: u32 = SCREEN_WIDTH;
    const FACE_SIDE_LENGTH: u32 = SCREEN_WIDTH - 2 * PADDING as u32;

    type Display = SimulatorDisplay<BinaryColor>;

    fn draw_in_window(display: &Display) -> Result<(), Infallible> {
        let output_settings = OutputSettingsBuilder::new().scale(1).build();
        Window::new("a die", &output_settings).show_static(&display);

        Ok(())
    }

    #[fixture]
    fn init_display() -> Display {
        SimulatorDisplay::new(Size::new(SCREEN_WIDTH, SCREEN_HEIGHT))
    }

    #[rstest]
    #[test_log::test]
    fn test_draw_face_one(#[from(init_display)] mut display: Display) -> Result<(), Infallible> {
        player::roll_die(
            &mut display.translated(Point::new(PADDING, PADDING)),
            FACE_SIDE_LENGTH,
            rand::random(),
        )?;

        draw_in_window(&display)
    }
}
