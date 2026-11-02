//! Clock Generator and Power Management Controller (PMC)
//!
//! Every part of the chip runs from a clock that the clock tree derives from
//! a few oscillators. Software chooses the oscillators, programs the PLL that
//! multiplies their frequency, and selects the master clock that the
//! processor and most peripherals run from.
//!
//! ```text
//!  32 kHz RC ──────┐
//!  32.768 kHz XTAL ┴─ slow clock ───────────────────┐
//!                                                   │
//!  4/8/12 MHz RC ──┐                                ├─ prescaler ─ master clock
//!  3-20 MHz XTAL ──┴─ main clock ──┬────────────────┤
//!                                  └─ PLLA ×mul/div ┘
//! ```
//!
//! The slow clock drives the watchdog, the real-time timer, the real-time
//! clock and the input debouncing filters. The master clock is also the
//! processor clock, at most 84 MHz; the UART, USART, SPI, TWI, timer
//! counters and the other peripherals divide their own clocks from it.
//!
//! A [`Config`] describes the tree to build. [`init`] brings the chip to it,
//! in the order the datasheet requires, and returns the frequencies actually
//! running as [`Clocks`], which peripheral drivers take to compute their
//! dividers. [`Config::CRYSTAL_12MHZ_84MHZ`] is the configuration of the
//! Arduino Due and of the SAM3X-EK: a 12 MHz crystal, PLLA at 168 MHz, and
//! the master clock at 84 MHz.
//!
//! The USB PLL (UPLL) is not handled.
//!
//! See chapters 27 and 28 of the SAM3X/SAM3A datasheet for more details.

use crate::efc;
use crate::pac;

/// Source of the slow clock.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowClock {
    /// The embedded 32 kHz RC oscillator, 20 to 44 kHz in practice. Selected
    /// at reset.
    InternalRc,
    /// A 32.768 kHz crystal on XIN32/XOUT32. Once selected, it stays
    /// selected until the next power cycle.
    Crystal,
}

/// Frequency of the embedded fast RC oscillator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcFrequency {
    /// 4 MHz, the reset value.
    Mhz4,
    /// 8 MHz.
    Mhz8,
    /// 12 MHz.
    Mhz12,
}

/// Source of the main clock.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MainClock {
    /// The embedded fast RC oscillator, accurate to a few percent.
    InternalRc(RcFrequency),
    /// A crystal on XIN/XOUT, 3 to 20 MHz.
    Crystal {
        /// The crystal's frequency.
        hz: u32,
    },
}

/// PLLA: main clock × `mul` / `div`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Plla {
    /// Multiplier, 2 to 2048 (the register holds `mul - 1`, and 0 there
    /// means off).
    pub mul: u16,
    /// Divider, 1 to 255.
    pub div: u8,
}

/// Source of the master clock.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSource {
    /// The slow clock.
    Slow,
    /// The main clock.
    Main,
    /// The PLLA output. Requires [`Config::plla`].
    Plla,
}

/// The master clock prescaler.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prescaler {
    /// No division.
    Div1,
    /// Source / 2.
    Div2,
    /// Source / 3.
    Div3,
    /// Source / 4.
    Div4,
    /// Source / 8.
    Div8,
    /// Source / 16.
    Div16,
    /// Source / 32.
    Div32,
    /// Source / 64.
    Div64,
}

impl Prescaler {
    const fn value(self) -> u32 {
        match self {
            Prescaler::Div1 => 1,
            Prescaler::Div2 => 2,
            Prescaler::Div3 => 3,
            Prescaler::Div4 => 4,
            Prescaler::Div8 => 8,
            Prescaler::Div16 => 16,
            Prescaler::Div32 => 32,
            Prescaler::Div64 => 64,
        }
    }

    fn pres(self) -> pac::pmc::pmc_mckr::Pres {
        use pac::pmc::pmc_mckr::Pres;
        match self {
            Prescaler::Div1 => Pres::Clk1,
            Prescaler::Div2 => Pres::Clk2,
            Prescaler::Div3 => Pres::Clk3,
            Prescaler::Div4 => Pres::Clk4,
            Prescaler::Div8 => Pres::Clk8,
            Prescaler::Div16 => Pres::Clk16,
            Prescaler::Div32 => Pres::Clk32,
            Prescaler::Div64 => Pres::Clk64,
        }
    }
}

/// A clock tree configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// Slow clock source.
    pub slow: SlowClock,
    /// Main clock source.
    pub main: MainClock,
    /// PLLA, if it runs.
    pub plla: Option<Plla>,
    /// Master clock source.
    pub master: MasterSource,
    /// Master clock prescaler.
    pub prescaler: Prescaler,
}

/// The maximum master clock frequency.
pub const MCK_MAX_HZ: u32 = 84_000_000;

/// A configuration the chip cannot run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The main crystal is outside 3 to 20 MHz.
    CrystalOutOfRange,
    /// PLLA's multiplier or divider is outside the register's range.
    PllaOutOfRange,
    /// The master clock source is PLLA but [`Config::plla`] is `None`.
    PllaMissing,
    /// The master clock would exceed [`MCK_MAX_HZ`].
    MasterTooFast,
}

impl Config {
    /// The reset configuration: slow clock on its RC, main clock on the
    /// 4 MHz RC, no PLL, master clock on the main clock.
    pub const RESET: Config = Config {
        slow: SlowClock::InternalRc,
        main: MainClock::InternalRc(RcFrequency::Mhz4),
        plla: None,
        master: MasterSource::Main,
        prescaler: Prescaler::Div1,
    };

    /// 12 MHz crystal, PLLA at 168 MHz, master clock at 84 MHz, as on the
    /// SAM3X-EK and the Arduino Due.
    pub const CRYSTAL_12MHZ_84MHZ: Config = Config {
        slow: SlowClock::InternalRc,
        main: MainClock::Crystal { hz: 12_000_000 },
        plla: Some(Plla { mul: 14, div: 1 }),
        master: MasterSource::Plla,
        prescaler: Prescaler::Div2,
    };

    /// Compute the frequencies of this configuration.
    pub const fn clocks(&self) -> Result<Clocks, Error> {
        let slow_hz = match self.slow {
            SlowClock::InternalRc => 32_000,
            SlowClock::Crystal => 32_768,
        };
        let main_hz = match self.main {
            MainClock::InternalRc(RcFrequency::Mhz4) => 4_000_000,
            MainClock::InternalRc(RcFrequency::Mhz8) => 8_000_000,
            MainClock::InternalRc(RcFrequency::Mhz12) => 12_000_000,
            MainClock::Crystal { hz } => {
                if hz < 3_000_000 || hz > 20_000_000 {
                    return Err(Error::CrystalOutOfRange);
                }
                hz
            }
        };
        let plla_hz = match self.plla {
            None => None,
            Some(Plla { mul, div }) => {
                if mul < 2 || mul > 2048 || div == 0 {
                    return Err(Error::PllaOutOfRange);
                }
                let hz = main_hz as u64 * mul as u64 / div as u64;
                if hz > u32::MAX as u64 {
                    return Err(Error::PllaOutOfRange);
                }
                Some(hz as u32)
            }
        };
        let source_hz = match self.master {
            MasterSource::Slow => slow_hz,
            MasterSource::Main => main_hz,
            MasterSource::Plla => match plla_hz {
                Some(hz) => hz,
                None => return Err(Error::PllaMissing),
            },
        };
        let mck_hz = source_hz / self.prescaler.value();
        if mck_hz > MCK_MAX_HZ {
            return Err(Error::MasterTooFast);
        }
        Ok(Clocks {
            slow_hz,
            main_hz,
            plla_hz,
            mck_hz,
        })
    }
}

// Check the reference configuration at compile time.
const _: () = match Config::CRYSTAL_12MHZ_84MHZ.clocks() {
    Ok(c) => assert!(c.mck_hz == 84_000_000 && c.main_hz == 12_000_000),
    Err(_) => panic!("the reference configuration must be valid"),
};
const _: () = match Config::RESET.clocks() {
    Ok(c) => assert!(c.mck_hz == 4_000_000 && c.plla_hz.is_none()),
    Err(_) => panic!("the reset configuration must be valid"),
};

/// The running clock frequencies, passed to the peripheral drivers to
/// compute their dividers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Clocks {
    /// Slow clock, used by the RTT, the watchdog and the debounce filters.
    pub slow_hz: u32,
    /// Main clock.
    pub main_hz: u32,
    /// PLLA's output, if it runs.
    pub plla_hz: Option<u32>,
    /// Master clock, also the processor clock. Most peripherals divide
    /// their clock from it.
    pub mck_hz: u32,
}

/// Apply `config`, from reset or from any earlier configuration.
///
/// The master clock runs from the main clock while the oscillators and the
/// PLL are reprogrammed, and the flash uses its maximum wait states until
/// the final frequency is set.
pub fn init(
    config: &Config,
    pmc: &pac::Pmc,
    supc: &pac::Supc,
    efc0: &pac::Efc0,
    efc1: &pac::Efc1,
) -> Result<Clocks, Error> {
    let clocks = config.clocks()?;

    efc::set_wait_states(efc0, efc1, efc::FWS_MAX);

    // Run the master clock from the main clock while the PLL is
    // reprogrammed. The main clock switches between its sources without
    // glitches.
    use pac::pmc::pmc_mckr::Css;
    if !pmc.pmc_mckr().read().css().is_main_clk() {
        pmc.pmc_mckr().modify(|_, w| w.css().main_clk());
        while pmc.pmc_sr().read().mckrdy().bit_is_clear() {}
    }

    if config.slow == SlowClock::Crystal && supc.sr().read().oscsel().is_rc() {
        // SAFETY: SUPC_CR is write-only: a key and one command bit.
        unsafe {
            supc.cr()
                .write_with_zero(|w| w.key().passwd().xtalsel().crystal_sel());
        }
        while supc.sr().read().oscsel().is_rc() {}
    }

    match config.main {
        MainClock::Crystal { .. } => {
            // Enable the crystal and keep the RC running until the switch.
            // Start-up time of 8 × 8 slow-clock cycles: 2 ms at 32 kHz,
            // enough for any crystal in range.
            pmc.ckgr_mor().modify(|_, w| unsafe {
                w.key().passwd();
                w.moscrcen().set_bit();
                w.moscxtby().clear_bit();
                w.moscxten().set_bit();
                w.moscxtst().bits(8)
            });
            while pmc.pmc_sr().read().moscxts().bit_is_clear() {}
            pmc.ckgr_mor()
                .modify(|_, w| w.key().passwd().moscsel().set_bit());
            while pmc.pmc_sr().read().moscsels().bit_is_clear() {}
        }
        MainClock::InternalRc(f) => {
            use pac::pmc::ckgr_mor::Moscrcf;
            let f = match f {
                RcFrequency::Mhz4 => Moscrcf::_4Mhz,
                RcFrequency::Mhz8 => Moscrcf::_8Mhz,
                RcFrequency::Mhz12 => Moscrcf::_12Mhz,
            };
            pmc.ckgr_mor()
                .modify(|_, w| w.key().passwd().moscrcen().set_bit().moscrcf().variant(f));
            while pmc.pmc_sr().read().moscrcs().bit_is_clear() {}
            pmc.ckgr_mor()
                .modify(|_, w| w.key().passwd().moscsel().clear_bit());
            while pmc.pmc_sr().read().moscsels().bit_is_clear() {}
        }
    }

    match config.plla {
        Some(Plla { mul, div }) => {
            pmc.ckgr_pllar().write(|w| unsafe {
                w.one().set_bit();
                w.mula().bits(mul - 1);
                w.pllacount().bits(0x3f);
                w.diva().bits(div)
            });
            while pmc.pmc_sr().read().locka().bit_is_clear() {}
        }
        None => {
            // MULA = 0 disables the PLL.
            pmc.ckgr_pllar().write(|w| unsafe {
                w.one().set_bit();
                w.mula().bits(0);
                w.diva().bits(0)
            });
        }
    }

    // Switching to the PLL: set the prescaler, then the source, waiting for
    // each to be acknowledged. Other sources: the reverse order. This is the
    // datasheet's programming sequence.
    let pres = config.prescaler.pres();
    match config.master {
        MasterSource::Plla => {
            pmc.pmc_mckr().modify(|_, w| w.pres().variant(pres));
            while pmc.pmc_sr().read().mckrdy().bit_is_clear() {}
            pmc.pmc_mckr().modify(|_, w| w.css().variant(Css::PllaClk));
            while pmc.pmc_sr().read().mckrdy().bit_is_clear() {}
        }
        MasterSource::Main | MasterSource::Slow => {
            let css = match config.master {
                MasterSource::Slow => Css::SlowClk,
                _ => Css::MainClk,
            };
            pmc.pmc_mckr().modify(|_, w| w.css().variant(css));
            while pmc.pmc_sr().read().mckrdy().bit_is_clear() {}
            pmc.pmc_mckr().modify(|_, w| w.pres().variant(pres));
            while pmc.pmc_sr().read().mckrdy().bit_is_clear() {}
        }
    }

    efc::set_wait_states(efc0, efc1, efc::wait_states_for(clocks.mck_hz));
    Ok(clocks)
}
