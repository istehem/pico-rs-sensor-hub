use core::convert::Infallible;

#[derive(Debug)]
pub enum DrawError<DisplayError> {
    FontError(u8g2_fonts::Error<DisplayError>),
    DisplayError(DisplayError),
}

impl<DisplayError> From<DisplayError> for DrawError<DisplayError> {
    fn from(e: DisplayError) -> Self {
        DrawError::DisplayError(e)
    }
}

impl<DisplayError> From<u8g2_fonts::Error<DisplayError>> for DrawError<DisplayError> {
    fn from(e: u8g2_fonts::Error<DisplayError>) -> Self {
        DrawError::FontError(e)
    }
}

#[derive(Debug)]
pub enum FontError {
    Infallible,
    BackgroundColorNotSupported,
    GlyphNotFound,
}

impl From<Infallible> for FontError {
    fn from(_: Infallible) -> Self {
        FontError::Infallible
    }
}

impl From<u8g2_fonts::Error<Infallible>> for FontError {
    fn from(e: u8g2_fonts::Error<Infallible>) -> Self {
        match e {
            u8g2_fonts::Error::DisplayError(_) => Self::Infallible,
            u8g2_fonts::Error::BackgroundColorNotSupported => Self::BackgroundColorNotSupported,
            u8g2_fonts::Error::GlyphNotFound(_) => Self::GlyphNotFound,
        }
    }
}
