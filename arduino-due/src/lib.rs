//! Board support for the Arduino Due
//!
//! The Arduino Due is built around the SAM3X8E. This crate gives its pins
//! the names printed on the board, such as `d13`, `a0`, `dac0`, `sda` or
//! `can_tx`, and brings up the clock tree the board is designed for:
//! the 12 MHz crystal and an 84 MHz core. [`console()`] opens the UART on
//! the programming port, which the board's 16U2 bridges to USB.
//!
//! Its build script hands the board's memory layout, `memory.x`, to the
//! linker, so a firmware for the Due has none of its own. `flash.sh`, the
//! cargo runner, writes a firmware through the programming port and the
//! SAM-BA bootloader in ROM. That bootloader leaves the vector table
//! register where it was: build the firmware with cortex-m-rt's `set-vtor`
//! feature.
//!
//! See the [Arduino Due documentation] for the board itself.
//!
//! [Arduino Due documentation]: https://docs.arduino.cc/hardware/due/
#![no_std]
#![deny(missing_docs)]

pub use sam3x_hal as hal;
pub use sam3x_hal::pac;

use hal::clocks::{self, Clocks, Config};
use hal::gpio::{Peripheral, Pin, PinMode, Reset, A};
use hal::gpio::{
    PA0, PA1, PA10, PA11, PA12, PA13, PA14, PA15, PA16, PA17, PA18, PA19, PA2, PA20, PA21, PA22,
    PA23, PA24, PA25, PA26, PA27, PA28, PA29, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PB0, PB1, PB10,
    PB11, PB12, PB13, PB14, PB15, PB16, PB17, PB18, PB19, PB2, PB20, PB21, PB22, PB23, PB24, PB25,
    PB26, PB27, PB28, PB29, PB3, PB30, PB31, PB4, PB5, PB6, PB7, PB8, PB9, PC0, PC1, PC10, PC11,
    PC12, PC13, PC14, PC15, PC16, PC17, PC18, PC19, PC2, PC20, PC21, PC22, PC23, PC24, PC25, PC26,
    PC27, PC28, PC29, PC3, PC30, PC4, PC5, PC6, PC7, PC8, PC9, PD0, PD1, PD10, PD2, PD3, PD4, PD5,
    PD6, PD7, PD8, PD9,
};
use hal::usart::{Config as SerialConfig, Serial, UartPins};
use hal::watchdog::Watchdog;

/// The main crystal.
pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

/// The clock tree the board runs: 12 MHz crystal, PLLA at 168 MHz, core at
/// 84 MHz.
pub const CLOCK_CONFIG: Config = Config::CRYSTAL_12MHZ_84MHZ;

/// The console baud rate, the Arduino default.
pub const CONSOLE_BAUD: u32 = 115_200;

/// Disable the watchdog and bring the clock tree to [`CLOCK_CONFIG`].
///
/// To use the watchdog, do not call this function: start a [`Watchdog`]
/// and call [`clocks::init`] with [`CLOCK_CONFIG`] instead. The watchdog
/// can only be configured once after reset.
pub fn init(
    wdt: pac::Wdt,
    pmc: &pac::Pmc,
    supc: &pac::Supc,
    efc0: &pac::Efc0,
    efc1: &pac::Efc1,
) -> Clocks {
    Watchdog::new(wdt).disable();
    match clocks::init(&CLOCK_CONFIG, pmc, supc, efc0, efc1) {
        Ok(clocks) => clocks,
        // CLOCK_CONFIG is checked at compile time.
        Err(_) => unreachable!(),
    }
}

/// The programming port as a serial port: the UART on D0 (RX) and D1 (TX),
/// behind the 16U2's USB bridge.
pub type Console = Serial<pac::Uart, Pin<PA8, Peripheral<A>>, Pin<PA9, Peripheral<A>>>;

/// The programming-port console at [`CONSOLE_BAUD`], 8N1.
pub fn console<M0: PinMode, M1: PinMode>(
    uart: pac::Uart,
    pmc: &pac::Pmc,
    d0: Pin<PA8, M0>,
    d1: Pin<PA9, M1>,
    clocks: &Clocks,
) -> Console {
    let pins: UartPins = (d0.into_peripheral_a(), d1.into_peripheral_a());
    match Serial::new(uart, pins, &SerialConfig::new(CONSOLE_BAUD), clocks, pmc) {
        Ok(console) => console,
        // 115200 8N1 is valid at every master clock the board runs.
        Err(_) => unreachable!(),
    }
}

macro_rules! board_pins {
    ($( $(#[$doc:meta])* $field:ident : $Id:ident $hal:ident ),* $(,)?) => {
        /// Every pin of the SAM3X8E, under the board's names, each in its
        /// reset state.
        ///
        /// Pins the board does not bring to a header keep their SAM3X name
        /// (`pb0` to `pb10` are the EMAC's, `pb28` to `pb31` the SWD/JTAG
        /// port, `pc0` is ERASE). Two header pins are wired to two SAM3X
        /// pins at once: D4 to PC26 and PA29, D10 to PC29 and PA28.
        pub struct Pins {
            $(
                $(#[$doc])*
                pub $field: Pin<$Id, Reset>,
            )*
        }

        impl Pins {
            /// Take the four controllers and split them into the board's pins.
            pub fn new(
                pioa: pac::Pioa,
                piob: pac::Piob,
                pioc: pac::Pioc,
                piod: pac::Piod,
                pmc: &pac::Pmc,
            ) -> Self {
                let p = hal::gpio::Pins::new(pioa, piob, pioc, piod, pmc);
                Pins { $( $field: p.$hal, )* }
            }
        }
    };
}

board_pins! {
    /// D0, RX0: the programming port's receive line (UART RXD).
    d0: PA8 pa8,
    /// D1, TX0: the programming port's transmit line (UART TXD).
    d1: PA9 pa9,
    /// D2 (TIOA0).
    d2: PB25 pb25,
    /// D3 (TIOA7).
    d3: PC28 pc28,
    /// D4 (TIOB6). Also wired to PA29, [`Pins::d4_pa29`].
    d4: PC26 pc26,
    /// D5 (TIOA6).
    d5: PC25 pc25,
    /// D6 (PWML7).
    d6: PC24 pc24,
    /// D7 (PWML6).
    d7: PC23 pc23,
    /// D8 (PWML5).
    d8: PC22 pc22,
    /// D9 (PWML4).
    d9: PC21 pc21,
    /// D10 (TIOB7). Also wired to PA28, [`Pins::d10_pa28`].
    d10: PC29 pc29,
    /// D11 (TIOA8).
    d11: PD7 pd7,
    /// D12 (TIOB8).
    d12: PD8 pd8,
    /// D13, and the amber "L" LED (TIOB0).
    d13: PB27 pb27,
    /// D14, TX3: USART3 transmit, peripheral B.
    d14: PD4 pd4,
    /// D15, RX3: USART3 receive, peripheral B.
    d15: PD5 pd5,
    /// D16, TX2: USART1 transmit.
    d16: PA13 pa13,
    /// D17, RX2: USART1 receive.
    d17: PA12 pa12,
    /// D18, TX1: USART0 transmit.
    d18: PA11 pa11,
    /// D19, RX1: USART0 receive.
    d19: PA10 pa10,
    /// D20, SDA: TWI1 data.
    sda: PB12 pb12,
    /// D21, SCL: TWI1 clock.
    scl: PB13 pb13,
    /// D22.
    d22: PB26 pb26,
    /// D23.
    d23: PA14 pa14,
    /// D24.
    d24: PA15 pa15,
    /// D25.
    d25: PD0 pd0,
    /// D26.
    d26: PD1 pd1,
    /// D27.
    d27: PD2 pd2,
    /// D28.
    d28: PD3 pd3,
    /// D29.
    d29: PD6 pd6,
    /// D30.
    d30: PD9 pd9,
    /// D31.
    d31: PA7 pa7,
    /// D32.
    d32: PD10 pd10,
    /// D33.
    d33: PC1 pc1,
    /// D34.
    d34: PC2 pc2,
    /// D35.
    d35: PC3 pc3,
    /// D36.
    d36: PC4 pc4,
    /// D37.
    d37: PC5 pc5,
    /// D38.
    d38: PC6 pc6,
    /// D39.
    d39: PC7 pc7,
    /// D40.
    d40: PC8 pc8,
    /// D41.
    d41: PC9 pc9,
    /// D42.
    d42: PA19 pa19,
    /// D43.
    d43: PA20 pa20,
    /// D44.
    d44: PC19 pc19,
    /// D45.
    d45: PC18 pc18,
    /// D46.
    d46: PC17 pc17,
    /// D47.
    d47: PC16 pc16,
    /// D48.
    d48: PC15 pc15,
    /// D49.
    d49: PC14 pc14,
    /// D50.
    d50: PC13 pc13,
    /// D51.
    d51: PC12 pc12,
    /// D52 (SPI NPCS2).
    d52: PB21 pb21,
    /// D53 (CANTX1).
    d53: PB14 pb14,
    /// A0 (AD7 on the ADC).
    a0: PA16 pa16,
    /// A1 (AD6).
    a1: PA24 pa24,
    /// A2 (AD5).
    a2: PA23 pa23,
    /// A3 (AD4).
    a3: PA22 pa22,
    /// A4 (AD3).
    a4: PA6 pa6,
    /// A5 (AD2).
    a5: PA4 pa4,
    /// A6 (AD1).
    a6: PA3 pa3,
    /// A7 (AD0).
    a7: PA2 pa2,
    /// A8 (AD10).
    a8: PB17 pb17,
    /// A9 (AD11).
    a9: PB18 pb18,
    /// A10 (AD12).
    a10: PB19 pb19,
    /// A11 (AD13).
    a11: PB20 pb20,
    /// DAC0 (also CANRX1).
    dac0: PB15 pb15,
    /// DAC1.
    dac1: PB16 pb16,
    /// CANRX (CAN0).
    can_rx: PA1 pa1,
    /// CANTX (CAN0).
    can_tx: PA0 pa0,
    /// SDA1: TWI0 data.
    sda1: PA17 pa17,
    /// SCL1: TWI0 clock.
    scl1: PA18 pa18,
    /// MISO on the SPI header.
    miso: PA25 pa25,
    /// MOSI on the SPI header.
    mosi: PA26 pa26,
    /// SCK on the SPI header.
    sck: PA27 pa27,
    /// The amber RX LED by the programming port, active low.
    rx_led: PC30 pc30,
    /// The amber TX LED by the programming port, active low.
    tx_led: PA21 pa21,
    /// The second SAM3X pin behind D4 (SPI NPCS1).
    d4_pa29: PA29 pa29,
    /// The second SAM3X pin behind D10 (SPI NPCS0).
    d10_pa28: PA28 pa28,
    /// The native USB port's ID line (UOTGID).
    usb_id: PB11 pb11,
    /// PA5, not on a header.
    pa5: PA5 pa5,
    /// PB0, EMAC, not on a header.
    pb0: PB0 pb0,
    /// PB1, EMAC, not on a header.
    pb1: PB1 pb1,
    /// PB2, EMAC, not on a header.
    pb2: PB2 pb2,
    /// PB3, EMAC, not on a header.
    pb3: PB3 pb3,
    /// PB4, EMAC, not on a header.
    pb4: PB4 pb4,
    /// PB5, EMAC, not on a header.
    pb5: PB5 pb5,
    /// PB6, EMAC, not on a header.
    pb6: PB6 pb6,
    /// PB7, EMAC, not on a header.
    pb7: PB7 pb7,
    /// PB8, EMAC, not on a header.
    pb8: PB8 pb8,
    /// PB9, EMAC, not on a header.
    pb9: PB9 pb9,
    /// PB10, EMAC, not on a header.
    pb10: PB10 pb10,
    /// PB22, not on a header.
    pb22: PB22 pb22,
    /// PB23 (SPI NPCS3), not on a header.
    pb23: PB23 pb23,
    /// PB24, not on a header.
    pb24: PB24 pb24,
    /// PB28, SWD/JTAG TCK, not on a header.
    pb28: PB28 pb28,
    /// PB29, SWD/JTAG TDI, not on a header.
    pb29: PB29 pb29,
    /// PB30, SWD/JTAG TDO, not on a header.
    pb30: PB30 pb30,
    /// PB31, SWD/JTAG TMS, not on a header.
    pb31: PB31 pb31,
    /// PC0, ERASE, not on a header.
    pc0: PC0 pc0,
    /// PC10, not on a header.
    pc10: PC10 pc10,
    /// PC11, not on a header.
    pc11: PC11 pc11,
    /// PC20, not on a header.
    pc20: PC20 pc20,
    /// PC27, not on a header.
    pc27: PC27 pc27,
}
