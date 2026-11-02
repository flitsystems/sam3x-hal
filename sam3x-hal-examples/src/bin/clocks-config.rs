//! Runs the core at 48 MHz instead of 84.
//!
//! 12 MHz crystal, PLLA at 96 MHz, master clock at 96 / 2. Prints the
//! resulting frequencies on the console at 115200 and blinks the LED once
//! a second. Both the baud divisor and the LED delay come from the
//! returned `Clocks`, so a readable console and a 1 Hz LED confirm the
//! configuration.
//!
//! Does not call the board's `init`, which sets 84 MHz.

#![no_std]
#![no_main]

use core::fmt::Write as _;

use arduino_due::hal::clocks::{self, Config, MainClock, MasterSource, Plla, Prescaler, SlowClock};
use arduino_due::hal::watchdog::Watchdog;
use arduino_due::{pac, Pins};
use cortex_m_rt::entry;
use panic_halt as _;

const CONFIG: Config = Config {
    slow: SlowClock::InternalRc,
    main: MainClock::Crystal {
        hz: arduino_due::XOSC_CRYSTAL_FREQ,
    },
    plla: Some(Plla { mul: 8, div: 1 }),
    master: MasterSource::Plla,
    prescaler: Prescaler::Div2,
};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    Watchdog::new(p.wdt).disable();
    let clocks = clocks::init(&CONFIG, &p.pmc, &p.supc, &p.efc0, &p.efc1).unwrap();
    let pins = Pins::new(p.pioa, p.piob, p.pioc, p.piod, &p.pmc);

    let mut console = arduino_due::console(p.uart, &p.pmc, pins.d0, pins.d1, &clocks);
    writeln!(
        console,
        "slow {} Hz, main {} Hz, plla {:?} Hz, mck {} Hz",
        clocks.slow_hz, clocks.main_hz, clocks.plla_hz, clocks.mck_hz
    )
    .ok();

    let mut led = pins.d13.into_push_pull_output();
    let half_second = clocks.mck_hz / 2;
    loop {
        led.toggle();
        cortex_m::asm::delay(half_second);
    }
}
