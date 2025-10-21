#![no_std]
#![no_main]

use bsp::entry;
use core::cell::RefCell;
use critical_section::Mutex;
use defmt::info;
use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
use embedded_hal::digital::StatefulOutputPin;
use panic_probe as _;
use ssd1306::mode::DisplayConfig;
use ssd1306::{
    mode::BufferedGraphicsMode, prelude::I2CInterface, rotation::DisplayRotation,
    size::DisplaySize128x64, I2CDisplayInterface, Ssd1306,
};

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::BinaryColor;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use u8g2_fonts::Error as FontError;

use game_logic::two_four_eighteen::{Game, NumberOfDice};
use pico_display::messages;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::init_clocks_and_plls, fugit::RateExtU32, gpio, pac, pac::interrupt, sio::Sio,
    watchdog::Watchdog, I2C,
};

use crate::gpio::bank0::Gpio6;
use crate::gpio::bank0::Gpio7;
use crate::gpio::FunctionSio;
use crate::gpio::Interrupt;

extern crate alloc;

use embedded_alloc::LlffHeap;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();

type IrBreakBeamPin = Option<gpio::Pin<gpio::bank0::Gpio21, gpio::FunctionSioInput, gpio::PullUp>>;
type OnBoardLed =
    Option<gpio::Pin<gpio::bank0::Gpio25, FunctionSio<gpio::SioOutput>, gpio::PullDown>>;
type I2CConfig = I2C<
    pac::I2C1,
    (
        gpio::Pin<Gpio6, gpio::FunctionI2C, gpio::PullUp>,
        gpio::Pin<Gpio7, gpio::FunctionI2C, gpio::PullUp>,
    ),
>;
type Display =
    Ssd1306<I2CInterface<I2CConfig>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>;

static IR_BREAK_BEAM: Mutex<RefCell<IrBreakBeamPin>> = Mutex::new(RefCell::new(None));
static ON_BOARD_LED: Mutex<RefCell<OnBoardLed>> = Mutex::new(RefCell::new(None));
static DISPLAY: Mutex<RefCell<Option<Display>>> = Mutex::new(RefCell::new(None));
static GAME: Mutex<RefCell<Option<Game>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    info!("Program start");
    {
        unsafe { HEAP.init(cortex_m_rt::heap_start() as usize, 8 * 1024) }
    }

    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let sda_pin = pins
        .gpio6
        .into_pull_up_input()
        .into_function::<gpio::FunctionI2C>();
    let scl_pin = pins
        .gpio7
        .into_pull_up_input()
        .into_function::<gpio::FunctionI2C>();

    let i2c: I2CConfig = I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        400_u32.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();

    let mut led_pin = pins.led.into_push_pull_output();
    led_pin.set_low().unwrap();

    // enable IRQ for the GPIO pin the IR sensor is connected to.
    let gpio21 = pins.gpio21.into_pull_up_input();
    gpio21.set_interrupt_enabled(Interrupt::EdgeLow, true);

    critical_section::with(|cs| {
        IR_BREAK_BEAM.borrow(cs).replace(Some(gpio21));
        ON_BOARD_LED.borrow(cs).replace(Some(led_pin));
        DISPLAY.borrow(cs).replace(Some(display));
        GAME.borrow(cs)
            .replace(Some(Game::new(SmallRng::seed_from_u64(12345))));
    });
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::IO_IRQ_BANK0); // Unmask NVIC interrupt
    }

    loop {
        cortex_m::asm::wfi();
    }
}

#[interrupt]
fn IO_IRQ_BANK0() {
    static mut IR_BREAK_BEAM_PIN: IrBreakBeamPin = None;
    static mut ON_BOARD_LED_PIN: OnBoardLed = None;
    static mut DISPLAY_IN_IRQ: Option<Display> = None;
    static mut GAME_IN_IRQ: Option<Game> = None;

    if IR_BREAK_BEAM_PIN.is_none() {
        critical_section::with(|cs| {
            *IR_BREAK_BEAM_PIN = IR_BREAK_BEAM.borrow(cs).take();
        });
    }

    if ON_BOARD_LED_PIN.is_none() {
        critical_section::with(|cs| {
            *ON_BOARD_LED_PIN = ON_BOARD_LED.borrow(cs).take();
        });
    }

    if DISPLAY_IN_IRQ.is_none() {
        critical_section::with(|cs| {
            *DISPLAY_IN_IRQ = DISPLAY.borrow(cs).take();
        });
    }

    if GAME_IN_IRQ.is_none() {
        critical_section::with(|cs| {
            *GAME_IN_IRQ = GAME.borrow(cs).take();
        });
    }

    // Check and handle the interrupt
    if let Some(beam_pin) = IR_BREAK_BEAM_PIN {
        if beam_pin.interrupt_status(Interrupt::EdgeLow) {
            info!("Beam broken!");
            // Always clear the interrupt flag
            beam_pin.clear_interrupt(Interrupt::EdgeLow);

            if let Some(led_pin) = ON_BOARD_LED_PIN {
                led_pin.toggle().unwrap();
            }
            if let (Some(display), Some(game)) = (DISPLAY_IN_IRQ, GAME_IN_IRQ) {
                play_and_draw(game, display).unwrap();
                display.flush().unwrap();
            }
        }
    }
}

fn play_and_draw(
    game: &mut Game,
    display: &mut Display,
) -> Result<(), FontError<<Display as DrawTarget>::Error>> {
    display
        .clear(BinaryColor::Off)
        .map_err(FontError::DisplayError)?;
    if game.dice_left > NumberOfDice::Zero {
        game.roll();
        game.rolled.draw(display).map_err(FontError::DisplayError)?;
        info!("current score: {}", game.score());
    } else {
        let mut picked: Vec<String> = game
            .picked
            .iter()
            .map(|die| die.value.as_u8().to_string())
            .collect();
        picked.sort();
        info!("picked: {}", picked.join(",").as_str());
        let score = game.score();
        info!("final score: {}", score);
        if game.has_fish() {
            messages::big_centered_message("Fish!", display)?;
        } else if game.has_won() {
            messages::big_centered_message("18!\nYou Win!", display)?;
        } else {
            messages::big_centered_message(score.to_string().as_str(), display)?;
        }
        game.reset();
    }
    Ok(())
}
