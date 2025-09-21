use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, CornerRadii, PrimitiveStyle, Rectangle, RoundedRectangle},
};

use num_traits::float::FloatCore;

pub fn draw_one<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let stroke_off_fill_on = PrimitiveStyle::with_fill(BinaryColor::On);

    let middle = (side_length - 1) / 2;
    let pip_size = percent_of_to_nearest_odd(side_length, 13);

    let pip_starts_at = (middle - (pip_size - 1) / 2) as i32;

    Circle::new(Point::new(pip_starts_at, pip_starts_at), pip_size)
        .into_styled(stroke_off_fill_on)
        .draw(target)?;

    RoundedRectangle::new(
        Rectangle::new(Point::new(0, 0), Size::new(side_length, side_length)),
        CornerRadii::new(Size::new(16, 16)),
    )
    .translate(Point::new(0, 0))
    .into_styled(stroke)
    .draw(target)
}

pub fn draw_two<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let stroke_off_fill_on = PrimitiveStyle::with_fill(BinaryColor::On);

    let middle = (side_length - 1) / 2;
    let pip_size = percent_of_to_nearest_odd(side_length, 13);

    let middle_off_set = (middle - 1) / 2;

    let first_pip_starts_at = (middle - middle_off_set - (pip_size - 1) / 2) as i32;

    Circle::new(
        Point::new(first_pip_starts_at, first_pip_starts_at),
        pip_size,
    )
    .into_styled(stroke_off_fill_on)
    .draw(target)?;

    let second_pip_starts_at = (middle + middle_off_set - (pip_size - 1) / 2) as i32;

    Circle::new(
        Point::new(second_pip_starts_at, second_pip_starts_at),
        pip_size,
    )
    .into_styled(stroke_off_fill_on)
    .draw(target)?;

    RoundedRectangle::new(
        Rectangle::new(Point::new(0, 0), Size::new(side_length, side_length)),
        CornerRadii::new(Size::new(16, 16)),
    )
    .translate(Point::new(0, 0))
    .into_styled(stroke)
    .draw(target)
}

fn percent_of_to_nearest_odd(numer: u32, percent: u32) -> u32 {
    let result = (numer as f64) * (percent as f64) / 100.0;
    let rounded = result.round() as u32;

    if rounded % 2 == 1 {
        rounded
    } else if rounded == 0 {
        1
    } else {
        let dist_down = (result - (rounded - 1) as f64).abs();
        let dist_up = (result - (rounded + 1) as f64).abs();

        if dist_down <= dist_up {
            rounded - 1
        } else {
            rounded + 1
        }
    }
}
