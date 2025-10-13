use crate::aliases::Display;
use crate::dice::Dice;
use crate::die::{Die, FaceValue};
use core::fmt;
use core::ops::Sub;
use embedded_graphics::pixelcolor::BinaryColor;
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
    small_rng: SmallRng,
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

    pub fn play<DI, DE>(&mut self, mut delay: DE, display: &mut DI) -> Result<(), DI::Error>
    where
        DI: Display,
        DE: Delay,
    {
        while self.dice_left > NumberOfDice::Zero {
            display.clear(BinaryColor::Off)?;
            self.roll();
            self.rolled.draw(display)?;
            //display.flush().unwrap();
            delay.delay_ms(5000);
        }
        Ok(())
    }

    pub fn roll(&mut self) {
        if self.dice_left == NumberOfDice::Zero {
            return;
        }
        let face_value = || self.small_rng.random();
        let dice = Dice::roll(face_value, self.dice_left.as_u8() as u32);

        let mut picked = Vec::new();
        picked.append(&mut self.picked);

        if !self.has_four() {
            picked.append(&mut dice.pick(FaceValue::Four, Some(1)));
        }
        if !self.has_two() {
            picked.append(&mut dice.pick(FaceValue::Two, Some(1)));
        }
        if has(&picked, FaceValue::Four) && has(&picked, FaceValue::Two) {
            picked.append(&mut dice.pick(FaceValue::Six, None));
        }
        if self.dice_left == (NumberOfDice::Five - picked.len() as u8) {
            // there must be a max value since dice were rolled
            picked.push(dice.max().unwrap());
        }

        self.rolled = dice;
        self.picked = picked;

        self.dice_left = NumberOfDice::Five - self.picked.len() as u8;
    }

    pub fn has_four(&self) -> bool {
        self.has(FaceValue::Four)
    }

    pub fn has_two(&self) -> bool {
        self.has(FaceValue::Two)
    }

    pub fn score(&self) -> i8 {
        if self.has_fish() {
            return -1;
        }
        self.picked
            .iter()
            .fold(0, |acc, &die| acc + die.value.as_u8()) as i8
            - 6
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
