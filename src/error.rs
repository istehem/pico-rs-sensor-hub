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
pub enum InfallibleDrawError {
    InfallibleFontError(()),
    Infallible(Infallible),
}

impl From<Infallible> for InfallibleDrawError {
    fn from(e: Infallible) -> Self {
        InfallibleDrawError::Infallible(e)
    }
}

impl From<u8g2_fonts::Error<Infallible>> for InfallibleDrawError {
    fn from(e: u8g2_fonts::Error<Infallible>) -> Self {
        match e {
            u8g2_fonts::Error::BackgroundColorNotSupported => Self::InfallibleFontError(()),
            u8g2_fonts::Error::GlyphNotFound(_) => Self::InfallibleFontError(()),
            u8g2_fonts::Error::DisplayError(e) => Self::Infallible(e),
        }
    }
}
