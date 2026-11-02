//! Console echo through the PDC.
//!
//! Sends a banner, then receives eight-byte blocks into RAM and
//! sends each one back, all through PDC transfers. Open the port at 115200
//! and type: the characters come back eight at a time.

#![no_std]
#![no_main]

use arduino_due::hal::pdc;
use arduino_due::{pac, Pins};
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let clocks = arduino_due::init(p.wdt, &p.pmc, &p.supc, &p.efc0, &p.efc1);
    let pins = Pins::new(p.pioa, p.piob, p.pioc, p.piod, &p.pmc);

    let console = arduino_due::console(p.uart, &p.pmc, pins.d0, pins.d1, &clocks);
    let (mut rx, tx) = console.split();

    // The PDC cannot read flash: the banner goes through a buffer in RAM.
    let banner =
        cortex_m::singleton!(: [u8; 33] = *b"uart-dma: type eight characters\r\n").unwrap();
    let (_, mut tx) = pdc::write(banner, tx).unwrap().wait();

    // The PDC needs a buffer with a fixed address.
    let mut block = cortex_m::singleton!(: [u8; 8] = [0; 8]).unwrap();
    loop {
        let (filled, rx_back) = pdc::read(block, rx).unwrap().wait();
        let (sent, tx_back) = pdc::write(filled, tx).unwrap().wait();
        block = sent;
        rx = rx_back;
        tx = tx_back;
    }
}
