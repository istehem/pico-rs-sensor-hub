use crate::dice::Dice;
use crate::die::{Die, FaceValue};
use core::ops::Sub;
use rand::rngs::SmallRng;
use rand::Rng;

extern crate alloc;
use alloc::vec::Vec;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum NumberOfDice {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl NumberOfDice {
    fn as_u8(&self) -> u8 {
        match self {
            NumberOfDice::One => 1,
            NumberOfDice::Two => 2,
            NumberOfDice::Three => 3,
            NumberOfDice::Four => 4,
            NumberOfDice::Five => 5,
        }
    }

    fn from_u8(number: u8) -> Self {
        match number {
            1 => NumberOfDice::One,
            2 => NumberOfDice::Two,
            3 => NumberOfDice::Three,
            4 => NumberOfDice::Four,
            _ => NumberOfDice::Five,
        }
    }
}

impl Sub<u8> for NumberOfDice {
    type Output = Self;

    fn sub(self, number: u8) -> Self::Output {
        NumberOfDice::from_u8(self.as_u8() - number)
    }
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
            let mut picks = Vec::new();
            if !self.has_four() && dice.has(FaceValue::Four) {
                picks.push(Die::new(FaceValue::Four));
            }
            if !self.has_two() && dice.has(FaceValue::Two) {
                picks.push(Die::new(FaceValue::Two));
            }
            if has(&picks, FaceValue::Four)
                && has(&picks, FaceValue::Two)
                && dice.has(FaceValue::Six)
            {
                picks.push(Die::new(FaceValue::Six));
            }

            self.rolled = picks;

            self.dice_left = self.dice_left - self.rolled.len() as u8;
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
        has(&self.rolled, face_value)
    }
}

fn has(dice: &[Die], face_value: FaceValue) -> bool {
    dice.iter().any(|&die| die.value == face_value)
}
