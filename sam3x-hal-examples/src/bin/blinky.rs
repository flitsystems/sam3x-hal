//! Blinks the Arduino Due's amber "L" LED (D13) twice a second.
//!
//! The delay is a busy loop on the core clock, without a timer.

#![no_std]
#![no_main]

use arduino_due::{pac, Pins};
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let clocks = arduino_due::init(p.wdt, &p.pmc, &p.supc, &p.efc0, &p.efc1);
    let pins = Pins::new(p.pioa, p.piob, p.pioc, p.piod, &p.pmc);

    let mut led = pins.d13.into_push_pull_output();
    let quarter_second = clocks.mck_hz / 4;
    loop {
        led.toggle();
        cortex_m::asm::delay(quarter_second);
    }
}
