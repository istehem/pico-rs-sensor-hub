use crate::dice::Dice;
use rand::rngs::SmallRng;
use rand::Rng;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum NumberOfDice {
    One,
    Two,
    Three,
    Four,
    Five,
}

pub struct Game {
    has_fish: bool,
    dice_left: NumberOfDice,
    score: u8,
    small_rng: SmallRng,
}

impl Game {
    pub fn new(small_rng: SmallRng) -> Self {
        Self {
            has_fish: false,
            dice_left: NumberOfDice::Five,
            score: 0,
            small_rng,
        }
    }

    pub fn play(&mut self) -> () {
        if self.dice_left == NumberOfDice::Five {
            let face_value = || self.small_rng.random();
            let dice = Dice::roll(face_value, 5);
            if dice.sum() < 17 {
                self.dice_left = NumberOfDice::Four;
            } else {
                self.dice_left = NumberOfDice::Three;
            }
        }
    }
}
