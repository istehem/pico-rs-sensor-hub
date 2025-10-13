use crate::dice::Dice;
use crate::die::{Die, FaceValue};
use core::ops::Sub;
use rand::rngs::SmallRng;
use rand::Rng;

extern crate alloc;
use alloc::vec::Vec;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum NumberOfDice {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

impl NumberOfDice {
    fn as_u8(&self) -> u8 {
        match self {
            NumberOfDice::Zero => 0,
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
            5 => NumberOfDice::Five,
            _ => NumberOfDice::Zero,
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
    picked: Vec<Die>,
    rolled: Option<Dice>,
}

impl Game {
    pub fn new(small_rng: SmallRng) -> Self {
        Self {
            dice_left: NumberOfDice::Five,
            small_rng,
            picked: Vec::new(),
            rolled: None,
        }
    }

    pub fn play(&mut self) {
        while self.dice_left > NumberOfDice::Zero {
            self.roll();
        }
    }

    fn roll(&mut self) {
        if self.dice_left == NumberOfDice::Zero {
            return;
        }
        let face_value = || self.small_rng.random();
        let dice = Dice::roll(face_value, self.dice_left.as_u8() as u32);
        let mut picked = Vec::new();
        if !self.has_four() {
            picked.append(&mut dice.pick(FaceValue::Four, Some(1)));
        }
        if !self.has_two() {
            picked.append(&mut dice.pick(FaceValue::Two, Some(1)));
        }
        if has(&picked, FaceValue::Four) && has(&picked, FaceValue::Two) {
            picked.append(&mut dice.pick(FaceValue::Six, None));
        }
        if picked.is_empty() {
            // there must be a max value since dice were rolled
            picked.push(dice.max().unwrap());
        }

        self.rolled = Some(dice);
        self.picked = picked;

        self.dice_left = self.dice_left - self.picked.len() as u8;
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
        self.picked
            .iter()
            .fold(0, |acc, &die| acc + die.value.as_u8())
    }

    fn has_fish(&self) -> bool {
        !(self.has_four() && self.has_two())
    }

    fn has(&self, face_value: FaceValue) -> bool {
        has(&self.picked, face_value)
    }
}

fn has(dice: &[Die], face_value: FaceValue) -> bool {
    dice.iter().any(|&die| die.value == face_value)
}
