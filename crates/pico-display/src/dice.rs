use core::cmp::min;
use embedded_graphics::geometry::Size;
use embedded_graphics::{prelude::*, primitives::rectangle::Rectangle};

use crate::aliases::Display;
use crate::die::{Die, FaceValue};

fn number_of_rows(number_of_dice: u32) -> u32 {
    let log = number_of_dice.ilog2(); // log = floor(log2(n))
    if log * log < number_of_dice {
        log + 1
    } else {
        log
    }
}

pub fn draw_dice<T, F>(
    target: &mut T,
    number_of_dice: u32,
    mut face_value: F,
) -> Result<(), T::Error>
where
    T: Display,
    F: FnMut() -> FaceValue,
{
    let sub_rows = number_of_rows(number_of_dice);
    let size = target.size();
    let sub_target_length = min(size.width, size.height) / sub_rows;

    for (counter, (i, j)) in (0..sub_rows)
        .flat_map(|i| (0..sub_rows).map(move |j| (i, j)))
        .enumerate()
    {
        if (counter as u32) >= number_of_dice {
            break;
        }
        let x = sub_target_length * i;
        let y = sub_target_length * j;
        let size = Size::new(sub_target_length, sub_target_length);

        let area = Rectangle::new(Point::new(x as i32, y as i32), size);

        let mut die = Die::new(face_value());
        die.draw(&mut target.cropped(&area))?;
    }
    Ok(())
}
