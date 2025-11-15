use core::{convert::Infallible, fmt};
use u8g2_fonts::Error as U8g2Error;

#[derive(Debug)]
pub enum DrawError<DisplayError> {
    FontError(U8g2Error<DisplayError>),
    DisplayError(DisplayError),
}

impl<DisplayError> From<DisplayError> for DrawError<DisplayError> {
    fn from(e: DisplayError) -> Self {
        DrawError::DisplayError(e)
    }
}

impl<DisplayError> From<U8g2Error<DisplayError>> for DrawError<DisplayError> {
    fn from(e: U8g2Error<DisplayError>) -> Self {
        DrawError::FontError(e)
    }
}

pub enum FontError {
    Infallible(Infallible),
    U8g2Error(U8g2Error<Infallible>),
}

impl fmt::Debug for FontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Infallible(err) => err.fmt(f),
            Self::U8g2Error(err) => err.fmt(f),
        }
    }
}

impl From<Infallible> for FontError {
    fn from(e: Infallible) -> Self {
        FontError::Infallible(e)
    }
}

impl From<U8g2Error<Infallible>> for FontError {
    fn from(e: U8g2Error<Infallible>) -> Self {
        match e {
            U8g2Error::DisplayError(err) => Self::Infallible(err),
            err => Self::U8g2Error(err),
        }
    }
}
