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
    point: PipPoint,
}

impl Pip {
    fn new(face_side_length: u32) -> Self {
        let size = percent_of_to_nearest_odd(face_side_length, 13);
        let point = PipPoint::new(face_side_length, size);
        let style = PrimitiveStyle::with_fill(BinaryColor::On);
        Self { size, style, point }
    }

    fn draw<T>(&self, target: &mut T, point: Point) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = BinaryColor>,
    {
        Circle::new(point, self.size)
            .into_styled(self.style)
            .draw(target)
    }

    fn draw_center_pip<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = BinaryColor>,
    {
        self.draw(target, self.point.center_pip_point())
    }

    fn draw_upper_left_pip<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = BinaryColor>,
    {
        self.draw(target, self.point.upper_left_pip_point())
    }

    fn draw_bottom_right_pip<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = BinaryColor>,
    {
        self.draw(target, self.point.bottom_right_pip_point())
    }

    fn draw_bottom_left_pip<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = BinaryColor>,
    {
        self.draw(target, self.point.bottom_left_pip_point())
    }

    fn draw_upper_right_pip<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = BinaryColor>,
    {
        self.draw(target, self.point.upper_right_pip_point())
    }

    fn draw_center_left_pip<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = BinaryColor>,
    {
        self.draw(target, self.point.center_left_pip_point())
    }

    fn draw_center_right_pip<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = BinaryColor>,
    {
        self.draw(target, self.point.center_right_pip_point())
    }
}

struct PipPoint {
    pip_size: u32,
    face_middle: u32,
    face_middle_offset: u32,
}

impl PipPoint {
    fn new(face_side_length: u32, pip_size: u32) -> Self {
        let face_middle = (face_side_length - 1) / 2 + 1;
        let face_middle_offset = (face_middle - 1) / 2;
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

    fn bottom_right_pip_point(&self) -> Point {
        let pip_starts_at =
            (self.face_middle + self.face_middle_offset - (self.pip_size - 1) / 2) as i32;
        Point::new(pip_starts_at, pip_starts_at)
    }

    fn bottom_left_pip_point(&self) -> Point {
        let pip_starts_at_x =
            (self.face_middle - self.face_middle_offset - (self.pip_size - 1) / 2) as i32;
        let pip_starts_at_y =
            (self.face_middle + self.face_middle_offset - (self.pip_size - 1) / 2) as i32;
        Point::new(pip_starts_at_x, pip_starts_at_y)
    }

    fn upper_right_pip_point(&self) -> Point {
        let pip_starts_at_x =
            (self.face_middle + self.face_middle_offset - (self.pip_size - 1) / 2) as i32;
        let pip_starts_at_y =
            (self.face_middle - self.face_middle_offset - (self.pip_size - 1) / 2) as i32;
        Point::new(pip_starts_at_x, pip_starts_at_y)
    }

    fn center_left_pip_point(&self) -> Point {
        let pip_starts_at_x =
            (self.face_middle - self.face_middle_offset - (self.pip_size - 1) / 2) as i32;
        let pip_starts_at_y = (self.face_middle - (self.pip_size - 1) / 2) as i32;
        Point::new(pip_starts_at_x, pip_starts_at_y)
    }

    fn center_right_pip_point(&self) -> Point {
        let pip_starts_at_x =
            (self.face_middle + self.face_middle_offset - (self.pip_size - 1) / 2) as i32;
        let pip_starts_at_y = (self.face_middle - (self.pip_size - 1) / 2) as i32;
        Point::new(pip_starts_at_x, pip_starts_at_y)
    }
}

pub fn draw_one<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let pip = Pip::new(side_length);

    pip.draw_center_pip(target)?;

    let face = Face::new(side_length);
    face.draw(target)
}

pub fn draw_two<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let pip = Pip::new(side_length);

    pip.draw_upper_left_pip(target)?;
    pip.draw_bottom_right_pip(target)?;

    let face = Face::new(side_length);
    face.draw(target)
}

pub fn draw_three<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let pip = Pip::new(side_length);

    pip.draw_center_pip(target)?;
    pip.draw_upper_left_pip(target)?;
    pip.draw_bottom_right_pip(target)?;

    let face = Face::new(side_length);
    face.draw(target)
}

pub fn draw_four<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let pip = Pip::new(side_length);

    pip.draw_upper_left_pip(target)?;
    pip.draw_upper_right_pip(target)?;
    pip.draw_bottom_right_pip(target)?;
    pip.draw_bottom_left_pip(target)?;

    let face = Face::new(side_length);
    face.draw(target)
}

pub fn draw_five<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let pip = Pip::new(side_length);

    pip.draw_center_pip(target)?;
    pip.draw_upper_left_pip(target)?;
    pip.draw_upper_right_pip(target)?;
    pip.draw_bottom_right_pip(target)?;
    pip.draw_bottom_left_pip(target)?;

    let face = Face::new(side_length);
    face.draw(target)
}

pub fn draw_six<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let pip = Pip::new(side_length);

    pip.draw_upper_left_pip(target)?;
    pip.draw_upper_right_pip(target)?;
    pip.draw_bottom_right_pip(target)?;
    pip.draw_bottom_left_pip(target)?;
    pip.draw_center_left_pip(target)?;
    pip.draw_center_right_pip(target)?;

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
