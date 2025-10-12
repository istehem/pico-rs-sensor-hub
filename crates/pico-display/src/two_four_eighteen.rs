use crate::dice::Dice;
use crate::die::{Die, FaceValue};
use rand::rngs::SmallRng;
use rand::Rng;

extern crate alloc;
use alloc::vec::Vec;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum NumberOfDice {
    //One,
    //Two,
    Three,
    Four,
    Five,
}

pub struct Game {
    dice_left: NumberOfDice,
    small_rng: SmallRng,
    rolled: Vec<Die>,
}

impl Game {
    pub fn new(small_rng: SmallRng) -> Self {
        Self {
            dice_left: NumberOfDice::Five,
            small_rng,
            rolled: Vec::new(),
        }
    }

    pub fn play(&mut self) {
        if self.dice_left == NumberOfDice::Five {
            let face_value = || self.small_rng.random();
            let dice = Dice::roll(face_value, 5);
            if dice.sum() < 17 {
                self.dice_left = NumberOfDice::Four;
            } else {
                self.dice_left = NumberOfDice::Three;
            }
            let mut picks = Vec::new();
            if !self.has_four() && dice.has(FaceValue::Four) {
                picks.push(Die::new(FaceValue::Four));
            }
            if !self.has_two() && dice.has(FaceValue::Two) {
                picks.push(Die::new(FaceValue::Two));
            }
            self.rolled = picks;
        }
    }

    pub fn has_four(&self) -> bool {
        self.has(FaceValue::Four)
    }

    pub fn has_two(&self) -> bool {
        self.has(FaceValue::Two)
    }

    pub fn score(&self) -> u8 {
        if self.has_fish() {
            return 0;
        }
        self.rolled
            .iter()
            .fold(0, |acc, &die| acc + die.value.as_u8())
    }

    fn has_fish(&self) -> bool {
        !(self.has_four() && self.has_two())
    }

    fn has(&self, face_value: FaceValue) -> bool {
        self.rolled.iter().any(|&die| die.value == face_value)
    }
}
