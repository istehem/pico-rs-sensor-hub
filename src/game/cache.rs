extern crate alloc;

use alloc::string::ToString;
use display_interface::DisplayError;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};
use {defmt_rtt as _, panic_probe as _};

use embedded_graphics_framebuf::FrameBuf;

use pico_display::messages;

use crate::game::entities::{Display, DisplayFrame, GameState};
use crate::game::error::FontError;

pub struct FrameCache {
    you_won_frame: FrameBuf<BinaryColor, DisplayFrame>,
    fish_frame: FrameBuf<BinaryColor, DisplayFrame>,
    score_frame: FrameBuf<BinaryColor, DisplayFrame>,
    picked_dice_frame: FrameBuf<BinaryColor, DisplayFrame>,
}

impl FrameCache {
    pub fn init() -> Result<Self, FontError> {
        let buffer = [BinaryColor::Off; 8192];

        let mut you_won_frame = new_frame_buffer(buffer);
        let mut fish_frame = new_frame_buffer(buffer);

        messages::big_centered_message("18!\nYou Win!", &mut you_won_frame)?;
        messages::big_centered_message("Fish!", &mut fish_frame)?;

        Ok(Self {
            you_won_frame,
            fish_frame,
            score_frame: new_frame_buffer(buffer),
            picked_dice_frame: new_frame_buffer(buffer),
        })
    }

    pub fn update_score_frame(&mut self, score: i8) -> Result<(), FontError> {
        self.score_frame.clear(BinaryColor::Off)?;
        messages::big_centered_message(score.to_string().as_str(), &mut self.score_frame)?;
        Ok(())
    }

    pub fn replace_picked_dice_frame(&mut self, frame: DisplayFrame) {
        self.picked_dice_frame = new_frame_buffer(frame);
    }

    pub fn draw_message(
        &self,
        display: &mut Display,
        game_state: &GameState,
    ) -> Result<(), DisplayError> {
        match game_state {
            GameState::Won(_) => display.draw_iter(&self.you_won_frame),
            GameState::Fish(_) => display.draw_iter(&self.fish_frame),
            GameState::GameOver(_, _) => display.draw_iter(&self.score_frame),
            _ => Ok(()),
        }
    }

    pub fn draw_picked_dice(&self, display: &mut Display) -> Result<(), DisplayError> {
        display.draw_iter(&self.picked_dice_frame)
    }
}

fn new_frame_buffer(frame: DisplayFrame) -> FrameBuf<BinaryColor, DisplayFrame> {
    FrameBuf::new(frame, 128, 64)
}
