//! Copies D2 to the "L" LED (D13): with its pull-up enabled, D2 lights the
//! LED while it is connected to GND.
//!
//! Test with a wire from D2 to GND.

#![no_std]
#![no_main]

use arduino_due::{pac, Pins};
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    arduino_due::init(p.wdt, &p.pmc, &p.supc, &p.efc0, &p.efc1);
    let pins = Pins::new(p.pioa, p.piob, p.pioc, p.piod, &p.pmc);

    let input = pins.d2.into_pull_up_input();
    let mut led = pins.d13.into_push_pull_output();
    loop {
        if input.is_low() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
