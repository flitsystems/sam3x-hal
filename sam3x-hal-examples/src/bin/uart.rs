//! Console echo on the programming port.
//!
//! Prints a banner, then sends back every byte it receives. Open the port
//! with a serial terminal at 115200 8N1.

#![no_std]
#![no_main]

use core::fmt::Write as _;

use arduino_due::{pac, Pins};
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let clocks = arduino_due::init(p.wdt, &p.pmc, &p.supc, &p.efc0, &p.efc1);
    let pins = Pins::new(p.pioa, p.piob, p.pioc, p.piod, &p.pmc);

    let mut console = arduino_due::console(p.uart, &p.pmc, pins.d0, pins.d1, &clocks);
    writeln!(console, "sam3x-hal: core at {} Hz, echoing", clocks.mck_hz).ok();

    loop {
        if let Some(byte) = console.read_byte() {
            console.write_byte(byte);
        }
    }
}
