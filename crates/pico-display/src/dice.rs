use embedded_graphics::geometry::Size;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, primitives::rectangle::Rectangle};

use crate::die::{Die, FaceValue};

pub fn four_sixes<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let die_size = side_length / 2;
    let size = Size::new(die_size, die_size);

    let first_die_area = Rectangle::new(Point::new(0, 0), size);
    let second_die_area = Rectangle::new(Point::new(die_size as i32, 0), size);
    let third_die_area = Rectangle::new(Point::new(0, die_size as i32), size);
    let fourth_die_area = Rectangle::new(Point::new(die_size as i32, die_size as i32), size);

    let mut die = Die::new(FaceValue::Six, die_size);
    die.draw(&mut target.cropped(&first_die_area))?;
    die.draw(&mut target.cropped(&second_die_area))?;
    die.draw(&mut target.cropped(&third_die_area))?;
    die.draw(&mut target.cropped(&fourth_die_area))
}
