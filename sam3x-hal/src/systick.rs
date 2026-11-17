//! SysTick, the Cortex-M3 core's 24-bit down counter.
//!
//! Two ways to use it, each owning the core's `SYST`:
//!
//! - [`SysDelay`] implements `embedded_hal::delay::DelayNs`. It splits a
//!   delay longer than 2^24 ticks (about 200 ms at 84 MHz) into several
//!   rounds.
//! - [`SysCounter`] counts a period over and over; [`SysCounter::wait`]
//!   says when one ended, without blocking. The period is at most 2^24
//!   ticks.
//!
//! Both count on the master clock.

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

/// A period [`SysCounter::start`] cannot count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The period is zero.
    Zero,
    /// The period is longer than 2^24 ticks (about 200 ms at 84 MHz).
    TooLong,
}

/// A periodic countdown on SysTick.
pub struct SysCounter {
    syst: SYST,
    mck_hz: u32,
}

impl SysCounter {
    /// Take SysTick and count on the master clock.
    pub fn new(mut syst: SYST, clocks: &Clocks) -> Self {
        syst.disable_counter();
        syst.set_clock_source(SystClkSource::Core);
        Self {
            syst,
            mck_hz: clocks.mck_hz,
        }
    }

    /// Start, or restart, a countdown of `us` microseconds that repeats
    /// until [`cancel`](Self::cancel).
    pub fn start(&mut self, us: u32) -> Result<(), Error> {
        // Microseconds to master clock ticks, rounded up.
        let ticks = (u64::from(us) * u64::from(self.mck_hz)).div_ceil(1_000_000);
        if ticks == 0 {
            return Err(Error::Zero);
        }
        if ticks > MAX_TICKS {
            return Err(Error::TooLong);
        }

        self.syst.disable_counter();
        // The counter runs from the reload value down to 0: reload + 1 ticks.
        self.syst.set_reload(ticks as u32 - 1);
        // Writing the current value also clears the wrap flag.
        self.syst.clear_current();
        self.syst.enable_counter();
        Ok(())
    }

    /// Whether a period ended since the last call. Always false once
    /// cancelled.
    pub fn wait(&mut self) -> bool {
        self.syst.has_wrapped()
    }

    /// Stop the countdown. Returns false if it was not running.
    pub fn cancel(&mut self) -> bool {
        let running = self.syst.is_counter_enabled();
        self.syst.disable_counter();
        running
    }

    /// Give SysTick back.
    pub fn free(self) -> SYST {
        self.syst
    }
}
