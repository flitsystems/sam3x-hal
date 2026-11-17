//! Blinks the "L" LED (D13) at 5 Hz for two seconds with a periodic SysTick
//! countdown, checks `cancel` and the period limit, then leaves the LED on.
//! A failed check panics and leaves the LED in its last state.

#![no_std]
#![no_main]

use arduino_due::hal::systick::{Error, SysCounter};
use arduino_due::{pac, Pins};
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let clocks = arduino_due::init(p.wdt, &p.pmc, &p.supc, &p.efc0, &p.efc1);
    let pins = Pins::new(p.pioa, p.piob, p.pioc, p.piod, &p.pmc);

    let mut led = pins.d13.into_push_pull_output();
    let mut timer = SysCounter::new(cp.SYST, &clocks);

    // Twenty periods of 100 ms: ten blinks.
    timer.start(100_000).unwrap();
    for _ in 0..20 {
        while !timer.wait() {}
        led.toggle();
    }

    // The first cancel stops a running counter, the second finds it stopped.
    assert!(timer.cancel());
    assert!(!timer.cancel());

    // 500 ms is more than 2^24 ticks at 84 MHz.
    assert_eq!(timer.start(500_000), Err(Error::TooLong));
    assert_eq!(timer.start(0), Err(Error::Zero));

    led.set_high();
    loop {
        cortex_m::asm::wfi();
    }
}
