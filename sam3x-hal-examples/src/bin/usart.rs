//! Bridges the console to USART0 (Serial1), both at 115200 8N1.
//!
//! Bytes received on the console are sent on D18 (TX1), bytes received on
//! D19 (RX1) are sent to the console. Wire D18 to D19 for an echo through
//! USART0, or connect a USB-serial adapter to D18/D19. Receive errors are
//! printed on the console.

#![no_std]
#![no_main]

use core::fmt::Write as _;

use arduino_due::hal::usart::{Config, Serial};
use arduino_due::{pac, Pins};
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let clocks = arduino_due::init(p.wdt, &p.pmc, &p.supc, &p.efc0, &p.efc1);
    let pins = Pins::new(p.pioa, p.piob, p.pioc, p.piod, &p.pmc);

    let mut console = arduino_due::console(p.uart, &p.pmc, pins.d0, pins.d1, &clocks);
    let serial1_pins = (pins.d19.into_peripheral_a(), pins.d18.into_peripheral_a());
    let mut serial1 = Serial::new(
        p.usart0,
        serial1_pins,
        &Config::new(115_200),
        &clocks,
        &p.pmc,
    )
    .unwrap();
    writeln!(console, "console <-> USART0 on D18/D19").ok();

    loop {
        if let Some(byte) = console.read_byte() {
            serial1.write_byte(byte);
        }
        if let Some(byte) = serial1.read_byte() {
            console.write_byte(byte);
        }
        if let Some(error) = serial1.take_error() {
            writeln!(console, "USART0: {:?}", error).ok();
        }
    }
}
