use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, CornerRadii, PrimitiveStyle, Rectangle, RoundedRectangle},
};

use core::cmp::Ordering;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::aliases::{Display, DrawTarget};
use crate::utils;

const PADDING_IN_PERCENT: u32 = 3;

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
        T: DrawTarget,
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
        let size = utils::percent_of_to_nearest_odd(face_side_length, 13);
        let point = PipPoint::new(face_side_length, size);
        let style = PrimitiveStyle::with_fill(BinaryColor::On);
        Self { size, style, point }
    }

    fn draw<T>(&self, target: &mut T, point: Point) -> Result<(), T::Error>
    where
        T: DrawTarget,
    {
        Circle::new(point, self.size)
            .into_styled(self.style)
            .draw(target)
    }

    fn draw_center_pip<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget,
    {
        self.draw(target, self.point.center_pip_point())
    }

    fn draw_upper_left_pip<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget,
    {
        self.draw(target, self.point.upper_left_pip_point())
    }

    fn draw_bottom_right_pip<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget,
    {
        self.draw(target, self.point.bottom_right_pip_point())
    }

    fn draw_bottom_left_pip<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget,
    {
        self.draw(target, self.point.bottom_left_pip_point())
    }

    fn draw_upper_right_pip<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget,
    {
        self.draw(target, self.point.upper_right_pip_point())
    }

    fn draw_center_left_pip<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget,
    {
        self.draw(target, self.point.center_left_pip_point())
    }

    fn draw_center_right_pip<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget,
    {
        self.draw(target, self.point.center_right_pip_point())
    }
}

struct PipPoint {
    starts_at_center: i32,
    starts_at_upper_or_left: i32,
    starts_at_bottom_or_right: i32,
}

impl PipPoint {
    fn new(face_side_length: u32, pip_size: u32) -> Self {
        let face_middle = (face_side_length - 1) / 2 + 1;
        let face_middle_offset = (face_middle - 1) / 2;

        let starts_at_center = (face_middle - (pip_size - 1) / 2) as i32;
        let starts_at_upper_or_left =
            (face_middle - face_middle_offset - (pip_size - 1) / 2) as i32;
        let starts_at_bottom_or_right =
            (face_middle + face_middle_offset - (pip_size - 1) / 2) as i32;

        Self {
            starts_at_center,
            starts_at_upper_or_left,
            starts_at_bottom_or_right,
        }
    }

    fn center_pip_point(&self) -> Point {
        Point::new(self.starts_at_center, self.starts_at_center)
    }

    fn upper_left_pip_point(&self) -> Point {
        Point::new(self.starts_at_upper_or_left, self.starts_at_upper_or_left)
    }

    fn bottom_right_pip_point(&self) -> Point {
        Point::new(
            self.starts_at_bottom_or_right,
            self.starts_at_bottom_or_right,
        )
    }

    fn bottom_left_pip_point(&self) -> Point {
        Point::new(self.starts_at_upper_or_left, self.starts_at_bottom_or_right)
    }

    fn upper_right_pip_point(&self) -> Point {
        Point::new(self.starts_at_bottom_or_right, self.starts_at_upper_or_left)
    }

    fn center_left_pip_point(&self) -> Point {
        Point::new(self.starts_at_upper_or_left, self.starts_at_center)
    }

    fn center_right_pip_point(&self) -> Point {
        Point::new(self.starts_at_bottom_or_right, self.starts_at_center)
    }
}

fn draw_one<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget,
{
    let pip = Pip::new(side_length);

    pip.draw_center_pip(target)?;

    let face = Face::new(side_length);
    face.draw(target)
}

fn draw_two<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget,
{
    let pip = Pip::new(side_length);

    pip.draw_upper_left_pip(target)?;
    pip.draw_bottom_right_pip(target)?;

    let face = Face::new(side_length);
    face.draw(target)
}

fn draw_three<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget,
{
    let pip = Pip::new(side_length);

    pip.draw_center_pip(target)?;
    pip.draw_upper_left_pip(target)?;
    pip.draw_bottom_right_pip(target)?;

    let face = Face::new(side_length);
    face.draw(target)
}

fn draw_four<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget,
{
    let pip = Pip::new(side_length);

    pip.draw_upper_left_pip(target)?;
    pip.draw_upper_right_pip(target)?;
    pip.draw_bottom_right_pip(target)?;
    pip.draw_bottom_left_pip(target)?;

    let face = Face::new(side_length);
    face.draw(target)
}

fn draw_five<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget,
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

fn draw_six<T>(target: &mut T, side_length: u32) -> Result<(), T::Error>
where
    T: DrawTarget,
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum FaceValue {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(Eq)]
pub struct Die {
    target_side_length: u32,
    value: FaceValue,
}

impl Die {
    pub fn new(value: FaceValue, target_side_length: u32) -> Self {
        Self {
            value,
            target_side_length,
        }
    }

    pub fn draw<T>(&mut self, target: &mut T) -> Result<(), T::Error>
    where
        T: Display,
    {
        let padding = utils::percent_of(self.target_side_length, PADDING_IN_PERCENT);

        let mut padded_target = target.translated(Point::new(padding as i32, padding as i32));
        let face_side_length = self.target_side_length - 2 * padding;

        match &self.value {
            FaceValue::One => draw_one(&mut padded_target, face_side_length),
            FaceValue::Two => draw_two(&mut padded_target, face_side_length),
            FaceValue::Three => draw_three(&mut padded_target, face_side_length),
            FaceValue::Four => draw_four(&mut padded_target, face_side_length),
            FaceValue::Five => draw_five(&mut padded_target, face_side_length),
            FaceValue::Six => draw_six(&mut padded_target, face_side_length),
        }
    }
}

impl PartialEq for Die {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Ord for Die {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Die {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Distribution<FaceValue> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FaceValue {
        let index: u8 = rng.gen_range(1..7);
        match index {
            1 => FaceValue::One,
            2 => FaceValue::Two,
            3 => FaceValue::Three,
            4 => FaceValue::Four,
            5 => FaceValue::Five,
            6 => FaceValue::Six,
            _ => unreachable!(),
        }
    }
}
