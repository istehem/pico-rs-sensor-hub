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
