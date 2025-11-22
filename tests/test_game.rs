#[cfg(test)]
mod tests {
    use rand;
    use rstest::{fixture, rstest};

    use game_logic::two_four_eighteen::Game;
    use game_logic::two_four_eighteen::NumberOfDice;
    use pico_display::dice::Dice;
    use pico_display::die::{Die, FaceValue};
    use pico_display::messages;

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
    #[ignore]
    fn play_game(
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
            thread::sleep(Duration::from_secs(1));
            display.clear(BinaryColor::Off)?;
        }
        let mut picked: Vec<String> = game
            .picked
            .dice
            .iter()
            .map(|die| die.value.as_u8().to_string())
            .collect();
        picked.sort();
        info!("picked: {}", picked.join(","));
        let score = game.score();
        info!("final score: {}", score);
        display.clear(BinaryColor::Off)?;
        if game.has_fish() {
            messages::big_centered_message("Fish!", &mut display).unwrap();
        } else if game.has_won() {
            messages::big_centered_message("18! You Win!", &mut display).unwrap();
        } else {
            messages::big_centered_message(score.to_string().as_str(), &mut display).unwrap();
        }
        window.update(&display);
        thread::sleep(Duration::from_secs(1));
        Ok(())
    }

    #[rstest]
    #[test_log::test]
    fn start_game_with_no_fish(
        #[from(init_display)] mut display: Display,
        #[from(gen_small_rng)] small_rng: SmallRng,
    ) -> Result<(), Infallible> {
        let output_settings = OutputSettingsBuilder::new().scale(1).build();
        let mut window = Window::new("Two Four Eighteen (No Fish)", &output_settings);

        let picked = vec![Die::new(FaceValue::Two), Die::new(FaceValue::Four)];
        let mut game = Game {
            dice_left: NumberOfDice::Three,
            small_rng,
            picked: Dice::from(picked),
            rolled: Dice::empty(),
        };
        game.roll();

        game.rolled.draw(&mut display)?;
        window.update(&display);
        thread::sleep(Duration::from_secs(2));

        display.clear(BinaryColor::Off)?;
        game.picked.draw(&mut display)?;
        window.update(&display);
        thread::sleep(Duration::from_secs(2));

        Ok(())
    }
}
