use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, CornerRadii, PrimitiveStyle, Rectangle, RoundedRectangle},
};

static CIRCLE_SIZE: i32 = 33;

pub fn draw_one<T>(target: &mut T) -> Result<(), T::Error>
where
    T: DrawTarget<Color = Rgb888>,
{
    let stroke = PrimitiveStyle::with_stroke(Rgb888::WHITE, 1);
    let stroke_off_fill_on = PrimitiveStyle::with_fill(Rgb888::WHITE);

    Circle::new(Point::new(102, 102), CIRCLE_SIZE as u32)
        .into_styled(stroke_off_fill_on)
        .draw(target)?;

    RoundedRectangle::new(
        Rectangle::new(Point::new(0, 0), Size::new(239, 239)),
        CornerRadii::new(Size::new(16, 16)),
    )
    .translate(Point::new(0, 0))
    .into_styled(stroke)
    .draw(target)
}

pub fn draw_two<T>(target: &mut T) -> Result<(), T::Error>
where
    T: DrawTarget<Color = Rgb888>,
{
    let stroke = PrimitiveStyle::with_stroke(Rgb888::WHITE, 1);
    let stroke_off_fill_on = PrimitiveStyle::with_fill(Rgb888::WHITE);

    Circle::new(Point::new(0, 0), CIRCLE_SIZE as u32)
        .into_styled(stroke_off_fill_on)
        .draw(target)?;

    Circle::new(Point::new(102, 102), CIRCLE_SIZE as u32)
        .into_styled(stroke_off_fill_on)
        .draw(target)?;

    RoundedRectangle::new(
        Rectangle::new(Point::new(0, 0), Size::new(239, 239)),
        CornerRadii::new(Size::new(16, 16)),
    )
    .translate(Point::new(0, 0))
    .into_styled(stroke)
    .draw(target)
}
