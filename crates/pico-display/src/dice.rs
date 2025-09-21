use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, CornerRadii, PrimitiveStyle, Rectangle, RoundedRectangle},
};

use num_traits::float::FloatCore;

struct Face {
    size: u32,
    style: PrimitiveStyle<BinaryColor>,
}

impl Face {
    fn new(size: u32) -> Self {
        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        Self { size, style }
    }

    fn draw<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = BinaryColor>,
    {
        RoundedRectangle::new(
            Rectangle::new(Point::new(0, 0), Size::new(self.size, self.size)),
            CornerRadii::new(Size::new(16, 16)),
        )
        .translate(Point::new(0, 0))
        .into_styled(self.style)
        .draw(target)
    }
}

struct Pip {
    size: u32,
    style: PrimitiveStyle<BinaryColor>,
}

impl Pip {
    fn new(size: u32) -> Self {
        let style = PrimitiveStyle::with_fill(BinaryColor::On);
        Self { size, style }
    }

    fn draw<T>(&self, target: &mut T, point: Point) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = BinaryColor>,
    {
        Circle::new(point, self.size)
            .into_styled(self.style)
            .draw(target)
    }
}

struct PipPoint {
    pip_size: u32,
    face_middle: u32,
    face_middle_offset: u32,
}

impl PipPoint {
    fn new(face_side_length: u32) -> Self {
        let face_middle = (face_side_length - 1) / 2 + 1;
        let face_middle_offset = (face_middle - 1) / 2;
        let pip_size = pip_size(face_side_length);
        Self {
            pip_size,
            face_middle,
            face_middle_offset,
        }
    }

    fn center_pip_point(&self) -> Point {
        let pip_starts_at = (self.face_middle - (self.pip_size - 1) / 2) as i32;
        Point::new(pip_starts_at, pip_starts_at)
    }

    fn upper_left_pip_point(&self) -> Point {
        let pip_starts_at =
            (self.face_middle - self.face_middle_offset - (self.pip_size - 1) / 2) as i32;
        Point::new(pip_starts_at, pip_starts_at)
    }

    fn button_right_pip_point(&self) -> Point {
        let pip_starts_at =
            (self.face_middle + self.face_middle_offset - (self.pip_size - 1) / 2) as i32;
        Point::new(pip_starts_at, pip_starts_at)
    }
}

fn pip_size(side_length: u32) -> u32 {
    percent_of_to_nearest_odd(side_length, 13)
}

pub fn draw_one<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let pip_size = pip_size(side_length);
    let pip = Pip::new(pip_size);
    let pip_point = PipPoint::new(side_length);

    pip.draw(target, pip_point.center_pip_point())?;

    let face = Face::new(side_length);
    face.draw(target)
}

pub fn draw_two<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let pip_size = pip_size(side_length);
    let pip = Pip::new(pip_size);
    let pip_point = PipPoint::new(side_length);

    pip.draw(target, pip_point.upper_left_pip_point())?;
    pip.draw(target, pip_point.button_right_pip_point())?;

    let face = Face::new(side_length);
    face.draw(target)
}

pub fn draw_three<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let pip_size = pip_size(side_length);
    let pip = Pip::new(pip_size);
    let pip_point = PipPoint::new(side_length);

    pip.draw(target, pip_point.center_pip_point())?;
    pip.draw(target, pip_point.upper_left_pip_point())?;
    pip.draw(target, pip_point.button_right_pip_point())?;

    let face = Face::new(side_length);
    face.draw(target)
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
