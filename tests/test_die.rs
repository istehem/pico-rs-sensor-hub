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

    const PADDING: i32 = 8;
    const SCREEN_WIDTH: u32 = 255;
    const SCREEN_HEIGHT: u32 = SCREEN_WIDTH;
    const FACE_SIDE_LENGTH: u32 = SCREEN_WIDTH - 2 * PADDING as u32;

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

        let mut die = Die::new(FaceValue::One, FACE_SIDE_LENGTH);
        die.draw(&mut display.translated(Point::new(PADDING, PADDING)))?;
        draw_in_window(&display)
    }

    #[rstest]
    #[test_log::test]
    fn test_draw_face_two(#[from(init_display)] mut display: Display) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();

        let mut die = Die::new(FaceValue::Two, FACE_SIDE_LENGTH);
        die.draw(&mut display.translated(Point::new(PADDING, PADDING)))?;
        draw_in_window(&display)
    }

    #[rstest]
    #[test_log::test]
    fn test_draw_face_three(#[from(init_display)] mut display: Display) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();

        let mut die = Die::new(FaceValue::Three, FACE_SIDE_LENGTH);
        die.draw(&mut display.translated(Point::new(PADDING, PADDING)))?;
        draw_in_window(&display)
    }

    #[rstest]
    #[test_log::test]
    fn test_draw_face_four(#[from(init_display)] mut display: Display) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();

        let mut die = Die::new(FaceValue::Four, FACE_SIDE_LENGTH);
        die.draw(&mut display.translated(Point::new(PADDING, PADDING)))?;
        draw_in_window(&display)
    }

    #[rstest]
    #[test_log::test]
    fn test_draw_face_five(#[from(init_display)] mut display: Display) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();

        let mut die = Die::new(FaceValue::Five, FACE_SIDE_LENGTH);
        die.draw(&mut display.translated(Point::new(PADDING, PADDING)))?;
        draw_in_window(&display)
    }

    #[rstest]
    #[test_log::test]
    fn test_draw_face_six(#[from(init_display)] mut display: Display) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();

        let mut die = Die::new(FaceValue::Six, FACE_SIDE_LENGTH);
        die.draw(&mut display.translated(Point::new(PADDING, PADDING)))?;
        draw_in_window(&display)
    }
}
