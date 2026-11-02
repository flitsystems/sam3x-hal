//! Enhanced Embedded Flash Controller (EEFC)
//!
//! The flash memory is slower than the processor. Above 19 MHz the core has
//! to wait one or more extra clock cycles, called wait states, on every read
//! from flash; the flash controller inserts them, and software tells it how
//! many.
//!
//! The SAM3X8E has two flash banks of 256 KB, each with its own controller,
//! EEFC0 and EEFC1. Both get the same number of wait states. That number
//! follows from the master clock frequency: [`wait_states_for`] reads it from
//! the datasheet's table for a 1.8 V core supply, from 0 up to 19 MHz to 4
//! at 84 MHz, the chip's maximum.
//!
//! Too few wait states and the core reads wrong instructions; too many only
//! cost speed. The wait states must therefore be raised before the master
//! clock speeds up. [`crate::clocks::init`] does it in that order: the
//! maximum while the clocks change, then the value the new frequency needs.
//!
//! See chapter 18 of the SAM3X/SAM3A datasheet for the controller, and
//! table 45-62 for the wait states.
//!

use crate::pac;

/// The wait states needed at the maximum master clock, 84 MHz.
pub const FWS_MAX: u8 = 4;

/// The wait states a master clock needs, based on its frequency.
pub const fn wait_states_for(mck_hz: u32) -> u8 {
    if mck_hz < 19_000_000 {
        0
    } else if mck_hz < 50_000_000 {
        1
    } else if mck_hz < 64_000_000 {
        2
    } else if mck_hz < 80_000_000 {
        3
    } else {
        FWS_MAX
    }
}

/// Program the wait states on both controllers. Call it before raising the
/// master clock above what the current setting allows.
pub fn set_wait_states(efc0: &pac::Efc0, efc1: &pac::Efc1, fws: u8) {
    efc0.fmr().modify(|_, w| unsafe { w.fws().bits(fws) });
    efc1.fmr().modify(|_, w| unsafe { w.fws().bits(fws) });
}
