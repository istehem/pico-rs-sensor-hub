#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::ToString;
use defmt::info;
use display_interface::DisplayError;
use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_rp::{
    bind_interrupts,
    gpio::{Input, Level, Output, Pull},
    i2c::{self, Config as I2cConfig, I2c},
    peripherals::I2C1,
};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::Channel, mutex::Mutex};
use embassy_time::{Instant, Timer};
use embedded_alloc::LlffHeap;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use ssd1306::{
    mode::{BufferedGraphicsModeAsync, DisplayConfigAsync},
    prelude::I2CInterface,
    rotation::DisplayRotation,
    size::DisplaySize128x64,
    I2CDisplayInterface, Ssd1306Async,
};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

use embedded_graphics_framebuf::FrameBuf;

use game_logic::two_four_eighteen::Game;
use pico_display::messages;

mod error;
use crate::error::DrawError;
mod player;
use crate::player::GameResult;

#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();

const I2C_FREQUENCY: u32 = 400_000;
const ONE_SECOND_IN_MUS: u64 = 1000000;

type Display = Ssd1306Async<
    I2CInterface<I2c<'static, I2C1, i2c::Async>>,
    DisplaySize128x64,
    BufferedGraphicsModeAsync<DisplaySize128x64>,
>;
type DisplayMutex = Mutex<NoopRawMutex, Display>;
static DISPLAY: StaticCell<DisplayMutex> = StaticCell::new();

type RollChannel = Channel<NoopRawMutex, u64, 4>;
static ROLL_CHANNEL: StaticCell<RollChannel> = StaticCell::new();

#[derive(PartialEq, Clone)]
enum DisplayState {
    Blink,
    Solid,
}

type DisplayStateChannel = Channel<NoopRawMutex, DisplayState, 4>;
static DISPLAY_STATE_CHANNEL: StaticCell<DisplayStateChannel> = StaticCell::new();

bind_interrupts!(struct Irqs {
    I2C1_IRQ => i2c::InterruptHandler<I2C1>;
});

type DisplayFrame = [BinaryColor; 8192];

#[derive(PartialEq)]
enum GameState {
    Playing,
    Won(DisplayFrame),
    Fish(DisplayFrame),
    GameOver(DisplayFrame, i8),
}

impl GameState {
    fn is_final_state(&self) -> bool {
        matches!(
            self,
            GameState::GameOver(_, _) | GameState::Won(_) | GameState::Fish(_)
        )
    }
}

type GameStateChannel = Channel<NoopRawMutex, GameState, 4>;
static GAME_STATE_CHANNEL: StaticCell<GameStateChannel> = StaticCell::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    {
        unsafe { HEAP.init(cortex_m_rt::heap_start() as usize, 8 * 1024) }
    }
    let p = embassy_rp::init(Default::default());

    let roll_channel = ROLL_CHANNEL.init(Channel::new());

    let led = Output::new(p.PIN_25, Level::Low);
    let sensor = Input::new(p.PIN_21, Pull::Up);

    spawner
        .spawn(break_beam_roller_task(sensor, led, roll_channel))
        .unwrap();

    let mut config = I2cConfig::default();
    config.frequency = I2C_FREQUENCY;
    let i2c = I2c::new_async(p.I2C1, p.PIN_7, p.PIN_6, Irqs, config);
    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().await.unwrap();
    display.clear(BinaryColor::Off).unwrap();
    messages::medium_sized_centered_message(
        "Break the beam for\n at least one second\n to start the game.",
        &mut display,
    )
    .unwrap();
    display.flush().await.unwrap();

    let display = DISPLAY.init(Mutex::new(display));
    let display_state_channel = DISPLAY_STATE_CHANNEL.init(Channel::new());

    let game_state_channel = GAME_STATE_CHANNEL.init(Channel::new());
    spawner
        .spawn(play_and_draw_task(
            display,
            roll_channel,
            game_state_channel,
        ))
        .unwrap();
    spawner
        .spawn(display_state_handler_task(display, display_state_channel))
        .unwrap();
    spawner
        .spawn(display_animations_task(
            display,
            game_state_channel,
            display_state_channel,
        ))
        .unwrap();
}

#[embassy_executor::task]
async fn break_beam_roller_task(
    mut sensor: Input<'static>,
    mut led: Output<'static>,
    roll_channel: &'static RollChannel,
) {
    let mut seed: Option<u64> = None;
    let mut beam_broken_at: Option<Instant> = None;

    loop {
        sensor.wait_for_any_edge().await;
        if sensor.is_high() {
            led.set_high();

            if let (Some(beam_broken_at), None) = (beam_broken_at, seed) {
                let duration = beam_broken_at.elapsed().as_micros();
                if duration > ONE_SECOND_IN_MUS {
                    roll_channel.send(duration).await;
                    seed = Some(duration);
                }
                info!("Beam broken for {} mus.", duration);
            }
        } else {
            led.set_low();

            if let Some(seed) = seed {
                roll_channel.send(seed).await;
            } else {
                beam_broken_at = Some(Instant::now());
            }
        }
        info!("Edge detected, level: {}", sensor.is_high());
    }
}

#[embassy_executor::task]
async fn play_and_draw_task(
    display: &'static DisplayMutex,
    roll_channel: &'static RollChannel,
    game_state_channel: &'static GameStateChannel,
) {
    let seed = roll_channel.receive().await;
    let mut game = Game::new(SmallRng::seed_from_u64(seed));
    let mut buffer = [BinaryColor::Off; 8192];

    info!("Game starts!");
    loop {
        let game_result = {
            let mut framebuffer = FrameBuf::new(&mut buffer, 128, 64);
            let game_result = player::play_and_draw(&mut framebuffer, &mut game).unwrap();

            let mut display = display.lock().await;
            display.draw_iter(&framebuffer).unwrap();
            display.flush().await.unwrap();
            game_result
        };
        match game_result {
            GameResult::GameOver(score) => {
                game_state_channel
                    .send(GameState::GameOver(buffer, score))
                    .await;
            }
            GameResult::Won => {
                game_state_channel.send(GameState::Won(buffer)).await;
            }
            GameResult::Fish => {
                game_state_channel.send(GameState::Fish(buffer)).await;
            }
            GameResult::Playing => {
                game_state_channel.send(GameState::Playing).await;
            }
        }
        roll_channel.receive().await;
    }
}

fn new_frame_buffer(frame: DisplayFrame) -> FrameBuf<BinaryColor, DisplayFrame> {
    FrameBuf::new(frame, 128, 64)
}

struct CachedFrames {
    you_won_frame: FrameBuf<BinaryColor, DisplayFrame>,
    fish_frame: FrameBuf<BinaryColor, DisplayFrame>,
    score_frame: FrameBuf<BinaryColor, DisplayFrame>,
    dice_frame: FrameBuf<BinaryColor, DisplayFrame>,
}

impl CachedFrames {
    fn new() -> Self {
        let buffer = [BinaryColor::Off; 8192];
        Self {
            you_won_frame: new_frame_buffer(buffer),
            fish_frame: new_frame_buffer(buffer),
            score_frame: new_frame_buffer(buffer),
            dice_frame: new_frame_buffer(buffer),
        }
    }

    fn draw_message(
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

    fn draw_dice(&self, display: &mut Display) -> Result<(), DisplayError> {
        display.draw_iter(&self.dice_frame)
    }
}

#[embassy_executor::task]
async fn display_animations_task(
    display: &'static DisplayMutex,
    game_state_channel: &'static GameStateChannel,
    display_state_channel: &'static DisplayStateChannel,
) {
    let mut game_state = GameState::Playing;
    let mut show_message = true;

    let mut cached_frames = CachedFrames::new();

    messages::big_centered_message("18!\nYou Win!", &mut cached_frames.you_won_frame).unwrap();
    messages::big_centered_message("Fish!", &mut cached_frames.fish_frame).unwrap();

    loop {
        match select(Timer::after_millis(2000), game_state_channel.receive()).await {
            Either::First(_) => {
                if game_state.is_final_state() {
                    let mut display = display.lock().await;

                    if show_message {
                        cached_frames
                            .draw_message(&mut display, &game_state)
                            .unwrap();
                    } else {
                        cached_frames.draw_dice(&mut display).unwrap();
                    }
                    display.flush().await.unwrap();
                    show_message = !show_message;
                }
            }
            Either::Second(state) => match state {
                GameState::Won(frame) | GameState::Fish(frame) => {
                    display_state_channel.send(DisplayState::Blink).await;
                    game_state = state;
                    cached_frames.dice_frame = new_frame_buffer(frame);
                }
                GameState::GameOver(frame, score) => {
                    display_state_channel.send(DisplayState::Blink).await;
                    game_state = state;
                    cached_frames.dice_frame = new_frame_buffer(frame);
                    cached_frames.score_frame.clear(BinaryColor::Off).unwrap();
                    messages::big_centered_message(
                        score.to_string().as_str(),
                        &mut cached_frames.score_frame,
                    )
                    .unwrap();
                }
                state => {
                    display_state_channel.send(DisplayState::Solid).await;
                    game_state = state;
                    show_message = true;
                }
            },
        }
    }
}

#[embassy_executor::task]
async fn display_state_handler_task(
    display: &'static DisplayMutex,
    display_state_channel: &'static DisplayStateChannel,
) {
    let mut invert_display = false;
    let mut display_state = DisplayState::Solid;

    loop {
        match select(Timer::after_millis(1000), display_state_channel.receive()).await {
            Either::First(_) => {
                if display_state == DisplayState::Blink {
                    invert_display = !invert_display;
                    set_invert_display(display, invert_display).await.unwrap();
                }
            }
            Either::Second(state) => {
                display_state = state;
                invert_display = display_state != DisplayState::Solid;
                set_invert_display(display, invert_display).await.unwrap();
            }
        }
    }
}

async fn set_invert_display(display: &DisplayMutex, invert: bool) -> Result<(), DisplayError> {
    display.lock().await.set_invert(invert).await
}
