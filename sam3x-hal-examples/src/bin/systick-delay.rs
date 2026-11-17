//! Blinks the "L" LED (D13) once a second, timed by SysTick through
//! `embedded_hal::delay::DelayNs`.

#![no_std]
#![no_main]

use arduino_due::hal::systick::SysDelay;
use arduino_due::{pac, Pins};
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let clocks = arduino_due::init(p.wdt, &p.pmc, &p.supc, &p.efc0, &p.efc1);
    let pins = Pins::new(p.pioa, p.piob, p.pioc, p.piod, &p.pmc);

    let mut led = pins.d13.into_push_pull_output();
    let mut delay = SysDelay::new(cp.SYST, &clocks);
    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
