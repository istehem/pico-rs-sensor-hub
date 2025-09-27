use embedded_graphics::{geometry::OriginDimensions, pixelcolor::BinaryColor, prelude};
use trait_set::trait_set;

trait_set! {
    pub trait Display = DrawTarget<Color = BinaryColor> + OriginDimensions;
}

trait_set! {
    pub trait DrawTarget = prelude::DrawTarget<Color = BinaryColor>;
}
