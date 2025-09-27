#[cfg(test)]
mod tests {
    use rstest::fixture;
    use rstest::rstest;

    use core::convert::Infallible;
    use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
    use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
    use lazy_static::lazy_static;
    use pico_display::die::{Die, FaceValue};
    use std::sync::Mutex;

    lazy_static! {
        static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }

    const SCREEN_WIDTH: u32 = 255;
    const SCREEN_HEIGHT: u32 = SCREEN_WIDTH;

    type Display = SimulatorDisplay<BinaryColor>;

    #[fixture]
    fn init_display() -> Display {
        SimulatorDisplay::new(Size::new(SCREEN_WIDTH, SCREEN_HEIGHT))
    }

    fn draw_in_window(display: &Display) -> Result<(), Infallible> {
        let output_settings = OutputSettingsBuilder::new().scale(1).build();
        Window::new("a die", &output_settings).show_static(&display);

        Ok(())
    }

    #[rstest]
    #[test_log::test]
    fn test_draw_face_one(#[from(init_display)] mut display: Display) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();

        let mut die = Die::new(FaceValue::One);
        die.draw(&mut display)?;
        draw_in_window(&display)
    }

    #[rstest]
    #[test_log::test]
    fn test_draw_face_two(#[from(init_display)] mut display: Display) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();

        let mut die = Die::new(FaceValue::Two);
        die.draw(&mut display)?;
        draw_in_window(&display)
    }

    #[rstest]
    #[test_log::test]
    fn test_draw_face_three(#[from(init_display)] mut display: Display) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();

        let mut die = Die::new(FaceValue::Three);
        die.draw(&mut display)?;
        draw_in_window(&display)
    }

    #[rstest]
    #[test_log::test]
    fn test_draw_face_four(#[from(init_display)] mut display: Display) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();

        let mut die = Die::new(FaceValue::Four);
        die.draw(&mut display)?;
        draw_in_window(&display)
    }

    #[rstest]
    #[test_log::test]
    fn test_draw_face_five(#[from(init_display)] mut display: Display) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();

        let mut die = Die::new(FaceValue::Five);
        die.draw(&mut display)?;
        draw_in_window(&display)
    }

    #[rstest]
    #[test_log::test]
    fn test_draw_face_six(#[from(init_display)] mut display: Display) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();

        let mut die = Die::new(FaceValue::Six);
        die.draw(&mut display)?;
        draw_in_window(&display)
    }
}
