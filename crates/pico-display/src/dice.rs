//! # Example: Primitive fill styles
//!
//! This example demonstrates the different fill and stroke styles available for primitives.

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{
        Circle, CornerRadii, Ellipse, PrimitiveStyle, Rectangle, RoundedRectangle, Triangle,
    },
};

static CIRCLE_SIZE: i32 = 65;
static ELLIPSE_SIZE: Size = Size::new(90, 65);

pub fn draw_shapes<T>(target: &mut T, style: PrimitiveStyle<Rgb888>) -> Result<(), T::Error>
where
    T: DrawTarget<Color = Rgb888>,
{
    Circle::new(Point::new(0, 0), CIRCLE_SIZE as u32)
        .into_styled(style)
        .draw(target)?;

    Rectangle::new(Point::new(105, 0), Size::new(64, 64))
        .into_styled(style)
        .draw(target)?;

    Triangle::new(Point::new(33, 0), Point::new(0, 64), Point::new(64, 64))
        .translate(Point::new(96 * 2 + 16, 0))
        .into_styled(style)
        .draw(target)?;

    Ellipse::new(Point::new(24, 108), ELLIPSE_SIZE)
        .into_styled(style)
        .draw(target)?;

    RoundedRectangle::new(
        Rectangle::new(Point::new(32, 0), Size::new(64, 64)),
        CornerRadii::new(Size::new(16, 16)),
    )
    .translate(Point::new(96 + 24, 108))
    .into_styled(style)
    .draw(target)
}
