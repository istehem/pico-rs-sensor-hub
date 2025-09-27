use embedded_graphics::{geometry::OriginDimensions, pixelcolor::BinaryColor, prelude::DrawTarget};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use trait_set::trait_set;

use crate::dice;

trait_set! {
    pub trait Display = DrawTarget<Color = BinaryColor> + OriginDimensions;
}

pub fn roll_die<T>(target: &mut T, seed: u64) -> Result<(), T::Error>
where
    T: Display,
{
    let mut small_rng = SmallRng::seed_from_u64(seed);
    let face_value = || small_rng.gen();
    dice::draw_dice(target, 1, face_value)
}

pub fn roll_two_dice<T>(target: &mut T, seed: u64) -> Result<(), T::Error>
where
    T: Display,
{
    let mut small_rng = SmallRng::seed_from_u64(seed);
    let face_value = || small_rng.gen();
    dice::draw_dice(target, 2, face_value)
}

pub fn roll_three_dice<T>(target: &mut T, seed: u64) -> Result<(), T::Error>
where
    T: Display,
{
    let mut small_rng = SmallRng::seed_from_u64(seed);
    let face_value = || small_rng.gen();
    dice::draw_dice(target, 3, face_value)
}

pub fn roll_four_dice<T>(target: &mut T, seed: u64) -> Result<(), T::Error>
where
    T: Display,
{
    let mut small_rng = SmallRng::seed_from_u64(seed);
    let face_value = || small_rng.gen();
    dice::draw_dice(target, 4, face_value)
}

pub fn roll_five_dice<T>(target: &mut T, seed: u64) -> Result<(), T::Error>
where
    T: Display,
{
    let mut small_rng = SmallRng::seed_from_u64(seed);
    let face_value = || small_rng.gen();
    dice::draw_dice(target, 5, face_value)
}
