use embassy_rp::{
    i2c::{self, I2c},
    peripherals::I2C1,
};
use embedded_graphics::pixelcolor::BinaryColor;
use ssd1306::{
    mode::BufferedGraphicsModeAsync, prelude::I2CInterface, size::DisplaySize128x64, Ssd1306Async,
};

pub type DisplayFrame = [BinaryColor; 8192];

pub type Display = Ssd1306Async<
    I2CInterface<I2c<'static, I2C1, i2c::Async>>,
    DisplaySize128x64,
    BufferedGraphicsModeAsync<DisplaySize128x64>,
>;

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    Won(DisplayFrame),
    Fish(DisplayFrame),
    GameOver(DisplayFrame, i8),
}

impl GameState {
    pub fn is_final_state(&self) -> bool {
        matches!(
            self,
            GameState::GameOver(_, _) | GameState::Won(_) | GameState::Fish(_)
        )
    }
}
