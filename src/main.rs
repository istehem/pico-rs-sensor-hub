#![no_std]
#![no_main]

use bsp::entry;
use core::cell::RefCell;
use critical_section::Mutex;
use defmt::info;
use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
use panic_probe as _;
use ssd1306::mode::DisplayConfig;
use ssd1306::{rotation::DisplayRotation, size::DisplaySize128x64, I2CDisplayInterface, Ssd1306};

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
    clocks::{init_clocks_and_plls, Clock},
    fugit::RateExtU32,
    gpio,
    pac,
    // needed to handle irq interrupts
    pac::interrupt,
    sio::Sio,
    watchdog::Watchdog,
    I2C,
};

use crate::gpio::Interrupt;

extern crate alloc;

use embedded_alloc::LlffHeap;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();

type IrBreakBeamPin = Option<gpio::Pin<gpio::bank0::Gpio21, gpio::FunctionSioInput, gpio::PullUp>>;

static IR_BREAK_BEAM: Mutex<RefCell<IrBreakBeamPin>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    info!("Program start");
    {
        unsafe { HEAP.init(cortex_m_rt::heap_start() as usize, 8 * 1024) }
    }

    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
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

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

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

    let i2c = I2C::i2c1(
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
    led_pin.set_high().unwrap();

    // enable IRQ for the GPIO pin the IR sensor in connected to.
    let gpio21 = pins.gpio21.into_pull_up_input();
    gpio21.set_interrupt_enabled(Interrupt::EdgeLow, true);

    critical_section::with(|cs| {
        IR_BREAK_BEAM.borrow(cs).replace(Some(gpio21));
    });

    let mut small_rng = SmallRng::seed_from_u64(12345);
    loop {
        info!("Starting new game!");

        let mut game = Game::new(small_rng.clone());

        while game.dice_left > NumberOfDice::Zero {
            display.clear(BinaryColor::Off).unwrap();
            game.roll();
            game.rolled.draw(&mut display).unwrap();

            info!("current score: {}", game.score());

            display.flush().unwrap();
            delay.delay_ms(5000);
        }
        let mut picked: Vec<String> = game
            .picked
            .iter()
            .map(|die| die.value.as_u8().to_string())
            .collect();
        picked.sort();
        info!("picked: {}", picked.join(",").as_str());
        let score = game.score();
        info!("final score: {}", score);
        display.clear(BinaryColor::Off).unwrap();
        if game.has_fish() {
            messages::big_centered_message("Fish!", &mut display).unwrap();
        } else if game.has_won() {
            messages::big_centered_message("18!\nYou Win!", &mut display).unwrap();
        } else {
            messages::big_centered_message(score.to_string().as_str(), &mut display).unwrap();
        }
        display.flush().unwrap();
        delay.delay_ms(5000);
        small_rng = game.small_rng;
    }
}

#[interrupt]
fn IO_IRQ_BANK0() {
    static mut IR_BREAK_BEAM_PIN: IrBreakBeamPin = None;

    // Initialize the static with the pin from the global
    if IR_BREAK_BEAM_PIN.is_none() {
        critical_section::with(|cs| {
            *IR_BREAK_BEAM_PIN = IR_BREAK_BEAM.borrow(cs).take();
        });
    }

    // Check and handle the interrupt
    if let Some(pin) = IR_BREAK_BEAM_PIN {
        if pin.interrupt_status(Interrupt::EdgeLow) {
            info!("Beam broken!");
            // Always clear the interrupt flag
            pin.clear_interrupt(Interrupt::EdgeLow);
        }
    }
}
