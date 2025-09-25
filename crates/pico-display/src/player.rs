use embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::dice;
use crate::die;

pub fn roll_die<T>(target: &mut T, side_length: u32, seed: u64) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let mut small_rng = SmallRng::seed_from_u64(seed);
    let value: die::FaceValue = small_rng.gen();
    die::Die::new(value, side_length).draw(target)
}

pub fn roll_two_dice<T>(target: &mut T, side_length: u32, _seed: u64) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    dice::double_sixes(target, side_length)
}
