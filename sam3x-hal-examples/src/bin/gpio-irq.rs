//! Toggles the "L" LED (D13) on each falling edge of D2, from the PIOB
//! interrupt. D2 has its pull-up and debounce filter enabled. The main loop
//! only sleeps.
//!
//! Test by tapping a wire from D2 to GND.

#![no_std]
#![no_main]

use core::cell::RefCell;

use arduino_due::hal::gpio::{InterruptMode, Output, Pin, Port, PushPull, PB27};
use arduino_due::pac::interrupt;
use arduino_due::{pac, Pins};
use cortex_m_rt::entry;
use critical_section::Mutex;
use panic_halt as _;

/// The LED pin type.
type Led = Pin<PB27, Output<PushPull>>;

/// The LED, shared with the interrupt handler.
static LED: Mutex<RefCell<Option<Led>>> = Mutex::new(RefCell::new(None));

/// D2's bit (PB25) in PIOB's status register.
const BUTTON: u32 = 1 << 25;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    arduino_due::init(p.wdt, &p.pmc, &p.supc, &p.efc0, &p.efc1);
    let pins = Pins::new(p.pioa, p.piob, p.pioc, p.piod, &p.pmc);

    let led = pins.d13.into_push_pull_output();
    critical_section::with(|cs| LED.borrow(cs).replace(Some(led)));

    // About 30 ms at the 32 kHz slow clock.
    Port::B.set_debounce_divider(500);
    let mut button = pins.d2.into_pull_up_input();
    button.enable_debounce_filter();
    button.enable_interrupt(InterruptMode::FallingEdge);

    // SAFETY: the handler only accesses the LED through a critical section.
    unsafe { cortex_m::peripheral::NVIC::unmask(Port::B.interrupt()) };

    loop {
        cortex_m::asm::wfi();
    }
}

/// PIOB interrupt handler.
#[interrupt]
fn PIOB() {
    // Reading the status clears every flag of the controller.
    let fired = Port::B.interrupt_status();
    if fired & BUTTON != 0 {
        critical_section::with(|cs| {
            if let Some(led) = LED.borrow(cs).borrow_mut().as_mut() {
                led.toggle();
            }
        });
    }
}
