use embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::dice;

pub fn roll_die<T>(target: &mut T, side_length: u32, seed: u64) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let mut small_rng = SmallRng::seed_from_u64(seed);
    let face_value = || small_rng.gen();
    dice::draw_dice(target, side_length, 1, face_value)
}

pub fn roll_two_dice<T>(target: &mut T, side_length: u32, seed: u64) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let mut small_rng = SmallRng::seed_from_u64(seed);
    let face_value = || small_rng.gen();
    dice::draw_dice(target, side_length, 2, face_value)
}
