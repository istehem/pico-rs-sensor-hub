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
    mode::DisplayConfigAsync, rotation::DisplayRotation, size::DisplaySize128x64,
    I2CDisplayInterface, Ssd1306Async,
};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

use embedded_graphics_framebuf::FrameBuf;

use game_logic::two_four_eighteen::Game;
use pico_display::messages;

use crate::error::DrawError;
use crate::player;
use crate::player::GameResult;
use crate::cache::FrameCache;
use crate::entities::{Display, GameState};

const I2C_FREQUENCY: u32 = 400_000;
const ONE_SECOND_IN_MUS: u64 = 1000000;

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

type GameStateChannel = Channel<NoopRawMutex, GameState, 4>;
static GAME_STATE_CHANNEL: StaticCell<GameStateChannel> = StaticCell::new();

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

#[embassy_executor::task]
async fn display_animations_task(
    display: &'static DisplayMutex,
    game_state_channel: &'static GameStateChannel,
    display_state_channel: &'static DisplayStateChannel,
) {
    let mut game_state = GameState::Playing;
    let mut show_message = true;

    let mut frame_cache = FrameCache::init().unwrap();

    loop {
        match select(Timer::after_millis(2000), game_state_channel.receive()).await {
            Either::First(_) => {
                if game_state.is_final_state() {
                    let mut display = display.lock().await;

                    if show_message {
                        frame_cache.draw_message(&mut display, &game_state).unwrap();
                    } else {
                        frame_cache.draw_picked_dice(&mut display).unwrap();
                    }
                    display.flush().await.unwrap();
                    show_message = !show_message;
                }
            }
            Either::Second(state) => match state {
                GameState::Won(frame) | GameState::Fish(frame) => {
                    display_state_channel.send(DisplayState::Blink).await;
                    game_state = state;
                    frame_cache.replace_picked_dice_frame(frame);
                }
                GameState::GameOver(frame, score) => {
                    display_state_channel.send(DisplayState::Blink).await;
                    game_state = state;
                    frame_cache.replace_picked_dice_frame(frame);
                    frame_cache.update_score_frame(score).unwrap();
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


