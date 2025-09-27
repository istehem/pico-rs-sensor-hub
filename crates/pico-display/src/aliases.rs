use embedded_graphics::{geometry::OriginDimensions, pixelcolor::BinaryColor, prelude::DrawTarget};
use trait_set::trait_set;

trait_set! {
    pub trait Display = DrawTarget<Color = BinaryColor> + OriginDimensions;
}
