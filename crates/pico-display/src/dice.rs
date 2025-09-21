use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, CornerRadii, PrimitiveStyle, Rectangle, RoundedRectangle},
};

use num_traits::float::FloatCore;

struct Pip {
    size: u32,
    style: PrimitiveStyle<BinaryColor>,
}

impl Pip {
    fn new(size: u32) -> Self {
        let style = PrimitiveStyle::with_fill(BinaryColor::On);
        Self { size, style }
    }

    fn draw<T>(&self, target: &mut T, x: i32, y: i32) -> Result<(), <T as DrawTarget>::Error>
    where
        T: DrawTarget<Color = BinaryColor>,
    {
        Circle::new(Point::new(x, y), self.size)
            .into_styled(self.style)
            .draw(target)
    }
}

pub fn draw_one<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

    let pip_size = percent_of_to_nearest_odd(side_length, 13);
    let pip = Pip::new(pip_size);

    let middle = (side_length - 1) / 2;
    let pip_starts_at = (middle - (pip_size - 1) / 2) as i32;

    pip.draw(target, pip_starts_at, pip_starts_at)?;

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

    let pip_size = percent_of_to_nearest_odd(side_length, 13);
    let pip = Pip::new(pip_size);

    let middle = (side_length - 1) / 2;
    let middle_offset = (middle - 1) / 2;
    let first_pip_starts_at = (middle - middle_offset - (pip_size - 1) / 2) as i32;

    pip.draw(target, first_pip_starts_at, first_pip_starts_at)?;

    let second_pip_starts_at = (middle + middle_offset - (pip_size - 1) / 2) as i32;

    pip.draw(target, second_pip_starts_at, second_pip_starts_at)?;

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
