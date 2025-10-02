#![no_std]
#![no_main]

use bsp::entry;
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

use pico_display::player;

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
    // pac::interrupt,
    sio::Sio,
    watchdog::Watchdog,
    I2C,
};

#[entry]
fn main() -> ! {
    info!("Program start");
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
    display.clear(BinaryColor::Off).unwrap();

    let mut led_pin = pins.led.into_push_pull_output();
    led_pin.set_high().unwrap();

    let mut small_rng = SmallRng::seed_from_u64(12345);
    loop {
        info!("rolling!");
        small_rng = player::roll_five_dice(&mut display, small_rng).unwrap();
        display.flush().unwrap();
        delay.delay_ms(2000);
    }
}
