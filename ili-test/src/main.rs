#![no_std]
#![no_main]

/**** low-level imports *****/
use panic_halt as _;
//use cortex_m::prelude::*;
use cortex_m_rt::entry;
use embedded_hal::{
        digital::v2::{OutputPin},
    };
// use embedded_time::rate::*;

use ili9341::Ili9341;
use display_interface_spi::SPIInterface;

use cortex_m::delay::Delay;
use fugit::RateExtU32;

/***** board-specific imports *****/
use adafruit_feather_rp2040::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        watchdog::Watchdog,
        Sio,
        gpio::{FunctionSpi, PushPullOutput},
        Spi,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};

#[entry]
fn main() -> ! {
    // Grab the singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    // Init the watchdog timer, to pass into the clock init
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();

    // initialize the Single Cycle IO
    let sio = Sio::new(pac.SIO);
    // initialize the pins to default state
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mosi = pins.mosi.into_mode::<FunctionSpi>();
    let miso = pins.miso.into_mode::<FunctionSpi>();
    let sclk = pins.sclk.into_mode::<FunctionSpi>();
    let spi = Spi::<_, _, 8>::new(pac.SPI0);

    // Exchange the uninitialised SPI driver for an initialised one
    let mut spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        12.MHz(),
        &embedded_hal::spi::MODE_0,
    );

    let iface = SPIInterface::new(spi, pins.d25.into_mode::<PushPullOutput>(), pins.d4.into_mode::<PushPullOutput>());
    let mut dt = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let display = Ili9341::new(
        iface,
        pins.d24.into_mode::<PushPullOutput>(),
        &mut dt,
        ili9341::Orientation::LandscapeFlipped,
        ili9341::DisplaySize240x320,
    ).unwrap();
    // setup the general-purpose delay timer
    

    // setup the on-board neopixel
    // let _timer = Timer::new(pac.TIMER, &mut pac.RESETS);

    // let mut timer = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let mut led_pin = pins.d13.into_push_pull_output();

    /* Loop Section */
    let delay: u32 = 70;   // loop delay in ms
    loop {
        led_pin.set_low().unwrap();
        dt.delay_ms((delay + 1200) as u32);
        led_pin.set_high().unwrap();
        dt.delay_ms(delay as u32);
    }

}