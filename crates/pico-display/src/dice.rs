use embedded_graphics::geometry::Size;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, primitives::rectangle::Rectangle};

use crate::die::{Die, FaceValue};

fn number_of_rows(number_of_dice: u32) -> u32 {
    let log = number_of_dice.ilog2(); // log = floor(log2(n))
    if log * log < number_of_dice {
        log + 1
    } else {
        log
    }
}

pub fn draw_dice<T>(target: &mut T, side_length: u32, number_of_dice: u32) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let sub_rows = number_of_rows(number_of_dice);
    let sub_target_length = side_length / sub_rows;
    let mut counter = 0;

    for (i, j) in (0..sub_rows).flat_map(|i| (0..sub_rows).map(move |j| (i, j))) {
        if counter >= number_of_dice {
            break;
        }
        let x = sub_target_length * i;
        let y = sub_target_length * j;
        let size = Size::new(sub_target_length, sub_target_length);

        let area = Rectangle::new(Point::new(x as i32, y as i32), size);

        let mut die = Die::new(FaceValue::Six, sub_target_length);
        die.draw(&mut target.cropped(&area))?;
        counter += 1;
    }
    return Ok(());
}
