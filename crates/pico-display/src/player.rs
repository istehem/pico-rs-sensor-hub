use embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::die;

pub fn roll_die<T>(target: &mut T, side_length: u32, seed: u64) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let mut small_rng = SmallRng::seed_from_u64(seed);
    let roll: u8 = small_rng.gen_range(1..7);

    match roll {
        1 => die::draw_one(target, side_length),
        2 => die::draw_two(target, side_length),
        3 => die::draw_three(target, side_length),
        4 => die::draw_four(target, side_length),
        5 => die::draw_five(target, side_length),
        6 => die::draw_six(target, side_length),
        _ => panic!("this face value should never be generated"),
    }
}
