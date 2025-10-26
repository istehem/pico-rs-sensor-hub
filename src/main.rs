#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::i2c::{self, Config as I2cConfig, I2c};
use embassy_rp::peripherals::I2C1;
//use embassy_time::Timer;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::BinaryColor;
use gpio::{Level, Output};
use ssd1306::mode::DisplayConfig;
use ssd1306::{rotation::DisplayRotation, size::DisplaySize128x64, I2CDisplayInterface, Ssd1306};
use static_cell::StaticCell;
//use embassy_sync::channel::Channel;
//use embassy_sync::blocking_mutex::raw::NoopRawMutex;

use embedded_alloc::LlffHeap;

use pico_display::messages;

#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();

static I2C: StaticCell<I2c<'static, I2C1, i2c::Async>> = StaticCell::new();
//static CHANNEL: StaticCell<Channel<NoopRawMutex, u32, 4>> = StaticCell::new();
static LED: StaticCell<Output<'static>> = StaticCell::new();

bind_interrupts!(struct Irqs {
    I2C1_IRQ => i2c::InterruptHandler<I2C1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    {
        unsafe { HEAP.init(cortex_m_rt::heap_start() as usize, 8 * 1024) }
    }
    let p = embassy_rp::init(Default::default());

    let led = Output::new(p.PIN_25, Level::Low);
    let led = LED.init(led);

    let sensor = Input::new(p.PIN_21, Pull::Up);
    spawner.spawn(ir_task(sensor, led)).unwrap();

    let config = I2cConfig::default();
    let i2c = I2c::new_async(p.I2C1, p.PIN_7, p.PIN_6, Irqs, config);
    let i2c = I2C.init(i2c);

    spawner.spawn(oled_task(i2c)).unwrap();
}

#[embassy_executor::task]
async fn ir_task(mut sensor: Input<'static>, led: &'static mut Output<'static>) {
    loop {
        sensor.wait_for_any_edge().await;
        if sensor.is_high() {
            led.set_high();
        } else {
            led.set_low();
        }
        info!("Edge detected, level: {}", sensor.is_high());
    }
}

#[embassy_executor::task]
async fn oled_task(i2c: &'static mut I2c<'static, I2C1, i2c::Async>) {
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();
    display.clear(BinaryColor::Off).unwrap();
    display.flush().unwrap();

    messages::medium_sized_centered_message(
        "Break the beam for\n at least one second\n to start the game.",
        &mut display,
    )
    .unwrap();
    display.flush().unwrap();
}

/*
#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use bsp::entry;
use core::cell::RefCell;
use critical_section::Mutex;
use defmt::info;
use defmt_rtt as _;
use embedded_alloc::LlffHeap;
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

use game_logic::two_four_eighteen::{Game, NumberOfDice};
use pico_display::messages;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
clocks::init_clocks_and_plls,
fugit::RateExtU32,
gpio, pac,
pac::interrupt,
sio::Sio,
timer::{Instant, Timer},
watchdog::Watchdog,
I2C,
};

use crate::gpio::bank0::Gpio6;
use crate::gpio::bank0::Gpio7;
use crate::gpio::FunctionSio;
use crate::gpio::Interrupt;

mod error;
use crate::error::DrawError;

#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();

type IrBreakBeamPin = Option<gpio::Pin<gpio::bank0::Gpio21, gpio::FunctionSioInput, gpio::PullUp>>;
type OnBoardLedPin =
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
static ON_BOARD_LED: Mutex<RefCell<OnBoardLedPin>> = Mutex::new(RefCell::new(None));
static DISPLAY: Mutex<RefCell<Option<Display>>> = Mutex::new(RefCell::new(None));
static TIMER: Mutex<RefCell<Option<Timer>>> = Mutex::new(RefCell::new(None));

const ONE_SECOND_IN_MUS: u64 = 1000000;

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
    display.clear(BinaryColor::Off).unwrap();
    display.flush().unwrap();

    let mut led_pin = pins.led.into_push_pull_output();
    led_pin.set_low().unwrap();

    // enable IRQ for the GPIO pin the IR sensor is connected to.
    let gpio21 = pins.gpio21.into_pull_up_input();
    gpio21.set_interrupt_enabled(Interrupt::EdgeLow, true);
    gpio21.set_interrupt_enabled(Interrupt::EdgeHigh, true);

    critical_section::with(|cs| {
        IR_BREAK_BEAM.borrow(cs).replace(Some(gpio21));
        ON_BOARD_LED.borrow(cs).replace(Some(led_pin));
        DISPLAY.borrow(cs).replace(Some(display));
        TIMER
            .borrow(cs)
            .replace(Some(Timer::new(pac.TIMER, &mut pac.RESETS, &clocks)));
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
    static mut IR_BREAK_BEAM_IN_IRQ: IrBreakBeamPin = None;
    static mut ON_BOARD_LED_IN_IRQ: OnBoardLedPin = None;
    static mut DISPLAY_IN_IRQ: Option<Display> = None;
    static mut TIMER_IN_IRQ: Option<Timer> = None;

    static mut BEAM_BROKEN_INSTANT: Option<Instant> = None;
    static mut GAME: Option<Game> = None;

    if IR_BREAK_BEAM_IN_IRQ.is_none() {
        critical_section::with(|cs| {
            *IR_BREAK_BEAM_IN_IRQ = IR_BREAK_BEAM.borrow(cs).take();
        });
    }

    if ON_BOARD_LED_IN_IRQ.is_none() {
        critical_section::with(|cs| {
            *ON_BOARD_LED_IN_IRQ = ON_BOARD_LED.borrow(cs).take();
        });
    }

    if DISPLAY_IN_IRQ.is_none() {
        critical_section::with(|cs| {
            *DISPLAY_IN_IRQ = DISPLAY.borrow(cs).take();
        });
    }

    if TIMER_IN_IRQ.is_none() {
        critical_section::with(|cs| {
            *TIMER_IN_IRQ = TIMER.borrow(cs).take();
        });
    }

    // Check and handle the interrupt
    if let Some(beam_pin) = IR_BREAK_BEAM_IN_IRQ {
        if beam_pin.interrupt_status(Interrupt::EdgeLow) {
            // Always clear the interrupt flag
            beam_pin.clear_interrupt(Interrupt::EdgeLow);
            info!("Beam broken!");

            if let Some(led_pin) = ON_BOARD_LED_IN_IRQ {
                led_pin.toggle().unwrap();
            }
            if let Some(timer) = TIMER_IN_IRQ {
                let instant = timer.get_counter();
                info!("Beam broken after {} mus.", instant.ticks());
                *BEAM_BROKEN_INSTANT = Some(instant);
            }

            if let Some(display) = DISPLAY_IN_IRQ {
                if let Some(game) = GAME {
                    play_and_draw(game, display).unwrap();
                    display.flush().unwrap();
                } else {
                    messages::medium_sized_centered_message(
                        "Break the beam for\n at least one second\n to start the game.",
                        display,
                    )
                    .unwrap();
                    display.flush().unwrap();
                }
            }
        } else if beam_pin.interrupt_status(Interrupt::EdgeHigh) {
            beam_pin.clear_interrupt(Interrupt::EdgeHigh);
            info!("Beam restored!");
            if let (Some(timer), Some(broken_instant), Some(display)) =
                (TIMER_IN_IRQ, BEAM_BROKEN_INSTANT, DISPLAY_IN_IRQ)
            {
                let broken_for = timer.get_counter().ticks() - broken_instant.ticks();
                info!("Beam broken for {} mus.", broken_for);

                // seeding must take a least on second
                if broken_for > ONE_SECOND_IN_MUS {
                    beam_pin.set_interrupt_enabled(Interrupt::EdgeHigh, false);

                    let mut game = Game::new(SmallRng::seed_from_u64(broken_for));
                    play_and_draw(&mut game, display).unwrap();
                    display.flush().unwrap();
                    *GAME = Some(game);
                }
            }
        }
    }
}

fn play_and_draw(
    game: &mut Game,
    display: &mut Display,
) -> Result<(), DrawError<<Display as DrawTarget>::Error>> {
    display.clear(BinaryColor::Off)?;
    if game.dice_left > NumberOfDice::Zero {
        game.roll();
        game.rolled.draw(display)?;
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
*/
