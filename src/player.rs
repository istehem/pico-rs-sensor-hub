use alloc::string::{String, ToString};
use alloc::vec::Vec;
use defmt::info;
use embedded_graphics::pixelcolor::BinaryColor;

use game_logic::two_four_eighteen::{Game, NumberOfDice};
use pico_display::aliases::Display as DisplayTrait;

use crate::DrawError;

#[derive(PartialEq)]
pub enum GameResult {
    Won,
    Fish,
    GameOver(i8),
    Playing,
}

impl GameResult {
    pub fn is_final_result(&self) -> bool {
        matches!(
            self,
            GameResult::GameOver(_) | GameResult::Won | GameResult::Fish
        )
    }
}

pub fn play_and_draw<T>(display: &mut T, game: &mut Game) -> Result<GameResult, DrawError<T::Error>>
where
    T: DisplayTrait,
{
    display.clear(BinaryColor::Off)?;
    if game.dice_left > NumberOfDice::Zero {
        game.roll();
        game.rolled.draw(display)?;
        info!("current score: {}", game.score());
        Ok(GameResult::Playing)
    } else {
        let mut picked: Vec<String> = game
            .picked
            .dice
            .iter()
            .map(|die| die.value.as_u8().to_string())
            .collect();
        picked.sort();
        info!("picked: {}", picked.join(",").as_str());
        let score = game.score();
        info!("final score: {}", score);
        game.picked.draw(display)?;
        if game.has_fish() {
            game.reset();
            Ok(GameResult::Fish)
        } else if game.has_won() {
            game.reset();
            Ok(GameResult::Won)
        } else {
            game.reset();
            Ok(GameResult::GameOver(score))
        }
    }
}
