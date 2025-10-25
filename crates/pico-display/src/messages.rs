use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::primitives::Rectangle;
use u8g2_fonts::fonts::{u8g2_font_logisoso22_tr, u8g2_font_t0_12_tr};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};
use u8g2_fonts::FontRenderer;

use crate::aliases::DrawTarget;
use u8g2_fonts::Error;

pub fn big_centered_message<T>(
    message: &str,
    target: &mut T,
) -> Result<Option<Rectangle>, Error<T::Error>>
where
    T: DrawTarget,
{
    let font = FontRenderer::new::<u8g2_font_logisoso22_tr>();
    font.render_aligned(
        message,
        target.bounding_box().center(),
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(BinaryColor::On),
        target,
    )
}

pub fn medium_sized_centered_message<T>(
    message: &str,
    target: &mut T,
) -> Result<Option<Rectangle>, Error<T::Error>>
where
    T: DrawTarget,
{
    let font = FontRenderer::new::<u8g2_font_t0_12_tr>();
    font.render_aligned(
        message,
        target.bounding_box().center(),
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(BinaryColor::On),
        target,
    )
}
