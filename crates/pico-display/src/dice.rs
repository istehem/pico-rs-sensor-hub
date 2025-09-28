use core::cmp::min;
use embedded_graphics::geometry::Size;
use embedded_graphics::{prelude::*, primitives::rectangle::Rectangle};

use crate::aliases::Display;
use crate::die::{Die, FaceValue};

pub fn draw_dice<T, F>(
    target: &mut T,
    number_of_dice: u32,
    mut face_value: F,
) -> Result<(), T::Error>
where
    T: Display,
    F: FnMut() -> FaceValue,
{
    let size = target.size();

    let (colums, rows, sub_target_length) = find_best_grid(number_of_dice, size.width, size.height);

    let mut counter = 0;

    for i in 0..colums {
        if counter >= number_of_dice {
            break;
        }
        for j in 0..rows {
            if counter >= number_of_dice {
                break;
            }

            let x = sub_target_length * i;
            let y = sub_target_length * j;
            let size = Size::new(sub_target_length, sub_target_length);

            let area = Rectangle::new(Point::new(x as i32, y as i32), size);

            let mut die = Die::new(face_value());
            die.draw(&mut target.cropped(&area))?;

            counter += 1;
        }
    }

    Ok(())
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
