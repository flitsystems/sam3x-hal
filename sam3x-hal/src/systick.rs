//! SysTick, the Cortex-M3 core's 24-bit down counter, as a busy-wait delay.
//!
//! [`SysDelay`] owns the core's `SYST` and implements
//! `embedded_hal::delay::DelayNs`. It counts on the master clock and splits
//! a delay longer than 2^24 ticks (about 200 ms at 84 MHz) into several
//! rounds.

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;
use embedded_hal::delay::DelayNs;

use crate::clocks::Clocks;

/// The reload register is 24 bits wide: one round counts at most 2^24 ticks.
const MAX_TICKS: u64 = 1 << 24;

/// A busy-wait delay on SysTick.
pub struct SysDelay {
    syst: SYST,
    mck_hz: u32,
}

impl SysDelay {
    /// Take SysTick and count on the master clock.
    pub fn new(mut syst: SYST, clocks: &Clocks) -> Self {
        syst.disable_counter();
        syst.set_clock_source(SystClkSource::Core);
        Self {
            syst,
            mck_hz: clocks.mck_hz,
        }
    }

    /// Give SysTick back.
    pub fn free(self) -> SYST {
        self.syst
    }
}

impl DelayNs for SysDelay {
    fn delay_ns(&mut self, ns: u32) {
        // Nanoseconds to master clock ticks, rounded up.
        let mut total = (u64::from(ns) * u64::from(self.mck_hz)).div_ceil(1_000_000_000);

        while total != 0 {
            let ticks = total.min(MAX_TICKS);

            // The counter runs from the reload value down to 0: reload + 1 ticks.
            self.syst.set_reload(ticks as u32 - 1);
            self.syst.clear_current();
            self.syst.enable_counter();
            while !self.syst.has_wrapped() {}
            self.syst.disable_counter();

            total -= ticks;
        }
    }
}
