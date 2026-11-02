//! Starts the watchdog with a 2 s timeout, feeds it during five slow blinks
//! of the "L" LED (D13), then stops feeding and leaves the LED on. The chip
//! resets 2 s later and starts over with three quick flashes.
//!
//! Does not call the board's `init`, which disables the watchdog.

#![no_std]
#![no_main]

use arduino_due::hal::clocks;
use arduino_due::hal::watchdog::Watchdog;
use arduino_due::{pac, Pins, CLOCK_CONFIG};
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(p.wdt);
    let clocks = clocks::init(&CLOCK_CONFIG, &p.pmc, &p.supc, &p.efc0, &p.efc1).unwrap();
    let pins = Pins::new(p.pioa, p.piob, p.pioc, p.piod, &p.pmc);
    let mut led = pins.d13.into_push_pull_output();

    // Boot marker: three quick flashes.
    let tenth = clocks.mck_hz / 10;
    for _ in 0..3 {
        led.set_high();
        cortex_m::asm::delay(tenth);
        led.set_low();
        cortex_m::asm::delay(tenth);
    }

    watchdog.start(2_000, &clocks).unwrap();

    // Five slow blinks: ten toggles, fed every half second.
    let half = clocks.mck_hz / 2;
    for _ in 0..10 {
        led.toggle();
        cortex_m::asm::delay(half);
        watchdog.feed();
    }

    // Stop feeding: the chip resets within 2 s.
    led.set_high();
    loop {
        cortex_m::asm::wfi();
    }
}
