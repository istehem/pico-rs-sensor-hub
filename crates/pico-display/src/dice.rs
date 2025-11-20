use core::cmp::min;
use embedded_graphics::geometry::Size;
use embedded_graphics::{prelude::*, primitives::rectangle::Rectangle};

extern crate alloc;
use alloc::vec::Vec;

use crate::aliases::Display;
use crate::die::{Die, FaceValue};

pub struct Dice {
    pub dice: Vec<Die>,
}

impl Dice {
    pub fn empty() -> Self {
        Self { dice: Vec::new() }
    }

    fn from(dice: Vec<Die>) -> Self {
        Self { dice }
    }

    pub fn roll<F>(mut face_value: F, number_of_dice: u32) -> Self
    where
        F: FnMut() -> FaceValue,
    {
        let mut dice = Vec::new();
        for _ in 0..number_of_dice {
            let die = Die::new(face_value());
            dice.push(die);
        }
        Dice { dice }
    }

    pub fn pick<F>(&self, decide: F, max_hits: Option<usize>) -> Self
    where
        F: Fn(FaceValue) -> bool,
    {
        let max_hits = max_hits.unwrap_or(self.dice.len());
        self.dice
            .iter()
            .filter(|&die| decide(die.value))
            .take(max_hits)
            .cloned()
            .collect::<Vec<_>>()
            .into()
    }

    pub fn max(&self) -> Option<Die> {
        self.dice.iter().max().copied()
    }

    pub fn draw<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: Display,
    {
        let number_of_dice = self.dice.len() as u32;
        let size = target.size();

        let (colums, rows, sub_target_length) =
            find_best_grid(number_of_dice, size.width, size.height);
        let rows_excess_space = (size.height - sub_target_length * rows) / 2;
        let columns_excess_space = (size.width - sub_target_length * colums) / 2;

        for (counter, (i, j)) in (0..colums)
            .flat_map(|i| (0..rows).map(move |j| (i, j)))
            .enumerate()
        {
            if (counter as u32) >= number_of_dice {
                break;
            }

            let x = sub_target_length * i + columns_excess_space;
            let y = sub_target_length * j + rows_excess_space;
            let size = Size::new(sub_target_length, sub_target_length);

            let area = Rectangle::new(Point::new(x as i32, y as i32), size);

            let mut die = self.dice[counter];
            die.draw(&mut target.cropped(&area))?;
        }
        Ok(())
    }

    pub fn append(&mut self, dice: &mut Dice) {
        self.dice.append(&mut dice.dice);
    }

    pub fn len(&self) -> usize {
        self.dice.len()
    }

    pub fn is_empty(&self) -> bool {
        self.dice.is_empty()
    }

    pub fn push(&mut self, die: Die) {
        self.dice.push(die);
    }
}

fn find_best_grid(number_of_entries: u32, width: u32, height: u32) -> (u32, u32, u32) {
    let mut best_size = 0;
    let mut best_colums = 1;
    let mut best_rows = number_of_entries;

    for colums in 1..number_of_entries + 1 {
        let rows = number_of_entries.div_ceil(colums); //  (number_of_entries + colums - 1) / colums;
        let max_width_size = width / colums;
        let max_height_size = height / rows;
        let size = min(max_width_size, max_height_size);

        if size > best_size {
            best_size = size;
            best_colums = colums;
            best_rows = rows;
        }
    }

    (best_colums, best_rows, best_size)
}

impl From<Vec<Die>> for Dice {
    fn from(value: Vec<Die>) -> Self {
        Dice::from(value)
    }
}
