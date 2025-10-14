use crate::dice::Dice;
use crate::die::{Die, FaceValue};
use core::fmt;
use core::ops::Sub;
use rand::rngs::SmallRng;
use rand::Rng;

extern crate alloc;
use alloc::vec::Vec;

pub trait Delay {
    fn delay_ms(&mut self, ms: u32);
}

impl Delay for cortex_m::delay::Delay {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_ms(ms);
    }
}

impl fmt::Display for NumberOfDice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            NumberOfDice::Zero => "Zero",
            NumberOfDice::One => "One",
            NumberOfDice::Two => "Two",
            NumberOfDice::Three => "Three",
            NumberOfDice::Four => "Four",
            NumberOfDice::Five => "Five",
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum NumberOfDice {
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
    pub dice_left: NumberOfDice,
    pub small_rng: SmallRng,
    pub picked: Vec<Die>,
    pub rolled: Dice,
}

impl Game {
    pub fn new(small_rng: SmallRng) -> Self {
        Self {
            dice_left: NumberOfDice::Five,
            small_rng,
            picked: Vec::new(),
            rolled: Dice::empty(),
        }
    }

    pub fn roll(&mut self) {
        if self.dice_left == NumberOfDice::Zero {
            return;
        }
        let face_value = || self.small_rng.random();
        let dice = Dice::roll(face_value, self.dice_left.as_u8() as u32);

        let mut picked = Vec::new();
        picked.append(&mut self.picked);

        if !has_four(&picked) {
            picked.append(&mut dice.pick(FaceValue::Four, Some(1)));
        }
        if !has_two(&picked) {
            picked.append(&mut dice.pick(FaceValue::Two, Some(1)));
        }
        if !has_fish(&picked) {
            picked.append(&mut dice.pick(FaceValue::Six, None));
        }
        // at least one die needs to be picked
        if self.dice_left == (NumberOfDice::Five - picked.len() as u8) {
            // there must be a max value since dice were rolled
            picked.push(dice.max().unwrap());
        }

        self.rolled = dice;
        self.picked = picked;

        self.dice_left = NumberOfDice::Five - self.picked.len() as u8;
    }

    pub fn score(&self) -> i8 {
        if has_fish(&self.picked) {
            return -1;
        }
        self.picked
            .iter()
            .fold(0, |acc, &die| acc + die.value.as_u8()) as i8
            - NumberOfDice::Four.as_u8() as i8
            - NumberOfDice::Two.as_u8() as i8
    }
}

fn has_fish(dice: &[Die]) -> bool {
    !(has_four(dice) && has_two(dice))
}

fn has_four(dice: &[Die]) -> bool {
    has(dice, FaceValue::Four)
}

fn has_two(dice: &[Die]) -> bool {
    has(dice, FaceValue::Two)
}

fn has(dice: &[Die], face_value: FaceValue) -> bool {
    dice.iter().any(|&die| die.value == face_value)
}
