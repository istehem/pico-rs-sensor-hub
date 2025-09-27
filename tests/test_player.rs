#[cfg(test)]
mod tests {
    use rand;
    use rstest::{fixture, rstest};

    use pico_display::player;

    use core::convert::Infallible;
    use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
    use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

    const SCREEN_WIDTH: u32 = 255;
    const SCREEN_HEIGHT: u32 = SCREEN_WIDTH;

    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }

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
    fn test_roll_die(#[from(init_display)] mut display: Display) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();
        player::roll_die(&mut display, rand::random())?;

        draw_in_window(&display)
    }

    #[rstest]
    #[test_log::test]
    fn test_roll_two_dice(#[from(init_display)] mut display: Display) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();
        player::roll_five_dice(&mut display, rand::random())?;

        draw_in_window(&display)
    }
}
