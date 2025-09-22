use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};

use crate::die;

pub fn roll_die<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    die::draw_six(target, side_length)
}
