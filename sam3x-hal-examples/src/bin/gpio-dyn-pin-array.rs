//! Chases a light along D13 (the "L" LED), D12 and D11.
//!
//! Puts three pins from two controllers in one array by converting each to
//! a [`DynPin`].

#![no_std]
#![no_main]

use arduino_due::hal::gpio::DynPin;
use arduino_due::{pac, Pins};
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let clocks = arduino_due::init(p.wdt, &p.pmc, &p.supc, &p.efc0, &p.efc1);
    let pins = Pins::new(p.pioa, p.piob, p.pioc, p.piod, &p.pmc);

    let mut leds: [DynPin; 3] = [
        pins.d13.into_push_pull_output().downgrade(),
        pins.d12.into_push_pull_output().downgrade(),
        pins.d11.into_push_pull_output().downgrade(),
    ];
    let step = clocks.mck_hz / 8;
    loop {
        for led in leds.iter_mut() {
            // Every pin is an output, so the mode check cannot fail.
            led.set_high().unwrap();
            cortex_m::asm::delay(step);
            led.set_low().unwrap();
        }
    }
}
