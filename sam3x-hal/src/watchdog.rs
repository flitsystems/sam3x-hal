//! Watchdog Timer (WDT)
//!
//! The watchdog is a 12-bit down counter that resets the chip when it reaches
//! zero. Software that is still running restarts it regularly with
//! [`Watchdog::feed`]; software stuck in a loop stops doing so, and the chip
//! resets.
//!
//! On the SAM3X the watchdog is already running when the chip leaves reset,
//! at its longest period, about 16 s. Its mode register can be written only
//! once: the first call to [`Watchdog::disable`] or [`Watchdog::start`] fixes
//! its behaviour until the next reset, and later writes are ignored by the
//! hardware. Call one of them early, before any long initialisation.
//!
//! The counter is clocked by the slow clock divided by 128. On the internal
//! RC oscillator, 32 kHz nominal but 20 to 44 kHz in practice, the actual
//! period can be about 30 % away from the requested one: choose a timeout
//! with margin.
//!
//! See chapter 15 of the SAM3X/SAM3A datasheet for more details.
//!
use crate::clocks::Clocks;
use crate::pac;

/// The counter's prescaler from the slow clock.
pub const TICK_DIV: u32 = 128;

/// The counter's width: 4096 ticks at most.
const TICKS_MAX: u64 = 4096;

/// A timeout out of the counter's range.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Shorter than one tick (4 ms at 32 kHz) or longer than 4096
    /// ([`Watchdog::max_timeout_ms`]).
    OutOfRange,
}

/// The watchdog status flags since the last read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Status {
    /// The counter reached zero.
    pub underflow: bool,
    /// Fed during the forbidden window.
    pub error: bool,
}

/// The watchdog, owning its peripheral.
pub struct Watchdog {
    wdt: pac::Wdt,
}

impl Watchdog {
    /// Take the peripheral without writing to it: the watchdog keeps its
    /// current state.
    pub fn new(wdt: pac::Wdt) -> Self {
        Self { wdt }
    }

    /// Stop the watchdog for the rest of the run.
    pub fn disable(self) {
        self.wdt.mr().write(|w| w.wddis().set_bit());
    }

    /// The longest timeout at the current slow clock frequency.
    pub const fn max_timeout_ms(clocks: &Clocks) -> u32 {
        (TICKS_MAX * TICK_DIV as u64 * 1000 / clocks.slow_hz as u64) as u32
    }

    /// Start the watchdog: the chip resets unless [`feed`](Self::feed) is
    /// called at least every `timeout_ms`. Feeding is allowed at any time.
    pub fn start(&mut self, timeout_ms: u32, clocks: &Clocks) -> Result<(), Error> {
        let ticks = timeout_ms as u64 * clocks.slow_hz as u64 / 1000 / TICK_DIV as u64;
        if ticks == 0 || ticks > TICKS_MAX {
            return Err(Error::OutOfRange);
        }
        let wdv = (ticks - 1) as u16;
        self.wdt.mr().write(|w| unsafe {
            w.wdv().bits(wdv);
            w.wdd().bits(wdv);
            w.wdrsten().set_bit();
            w.wdrproc().clear_bit();
            w.wdfien().clear_bit();
            w.wddbghlt().set_bit();
            w.wdidlehlt().clear_bit();
            w.wddis().clear_bit()
        });
        Ok(())
    }

    /// Restart the counter.
    pub fn feed(&mut self) {
        // SAFETY: WDT_CR is write-only, a key and one command bit.
        unsafe {
            self.wdt
                .cr()
                .write_with_zero(|w| w.key().passwd().wdrstt().set_bit());
        }
    }

    /// Read and clear the status flags.
    pub fn status(&mut self) -> Status {
        let sr = self.wdt.sr().read();
        Status {
            underflow: sr.wdunf().bit_is_set(),
            error: sr.wderr().bit_is_set(),
        }
    }
}
