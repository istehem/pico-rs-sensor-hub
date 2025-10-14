#[cfg(test)]
mod tests {
    use rand;
    use rstest::{fixture, rstest};

    use game_logic::two_four_eighteen::Game;
    use game_logic::two_four_eighteen::NumberOfDice;

    use core::convert::Infallible;
    use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
    use embedded_graphics_simulator::{
        OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
    };
    use tracing::info;

    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    use std::thread;
    use std::time::Duration;

    const SCALE: u32 = 5;
    const SCREEN_WIDTH: u32 = SCALE * 128;
    const SCREEN_HEIGHT: u32 = SCALE * 64;

    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }

    type Display = SimulatorDisplay<BinaryColor>;

    #[fixture]
    fn init_display() -> Display {
        SimulatorDisplay::new(Size::new(SCREEN_WIDTH, SCREEN_HEIGHT))
    }

    #[fixture]
    fn gen_small_rng() -> SmallRng {
        let seed: u64 = rand::random();
        SmallRng::seed_from_u64(seed)
    }

    #[rstest]
    #[test_log::test]
    fn test_play_game(
        #[from(init_display)] mut display: Display,
        #[from(gen_small_rng)] small_rng: SmallRng,
    ) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();

        let output_settings = OutputSettingsBuilder::new().scale(1).build();
        let mut window = Window::new("Two Four Eighteen", &output_settings);

        let mut game = Game::new(small_rng);

        'running: while game.dice_left > NumberOfDice::Zero {
            display.clear(BinaryColor::Off)?;
            game.roll();
            game.rolled.draw(&mut display)?;
            window.update(&display);
            info!("current score: {}", game.score());

            if window.events().any(|e| e == SimulatorEvent::Quit) {
                break 'running;
            }
            thread::sleep(Duration::from_secs(5));
        }
        let mut picked: Vec<String> = game
            .picked
            .iter()
            .map(|die| die.value.as_u8().to_string())
            .collect();
        picked.sort();
        info!("picked: {}", picked.join(","));
        info!("final score: {}", game.score());
        Ok(())
    }
}
