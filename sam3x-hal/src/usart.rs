//! Universal Asynchronous Receiver Transceiver (UART) and Universal
//! Synchronous Asynchronous Receiver Transmitter (USART)
//!
//! A serial port sends and receives characters on two wires, one bit at a
//! time at an agreed baud rate. Each character is a start bit, five to eight
//! data bits, an optional parity bit and one or more stop bits.
//!
//! The SAM3X has one UART and four USARTs, all driven by [`Serial`] in
//! asynchronous mode. The UART only runs eight data bits and one stop bit:
//! [`Serial::new`] refuses a [`Config`] it cannot run. The baud rate is the
//! master clock divided by 16 and by a 16-bit divisor, which has a
//! fractional part on the USARTs. Each direction has a single holding
//! register and no FIFO, so a byte not read before the next one arrives is
//! lost as an overrun. Longer transfers go through the [PDC](crate::pdc).
//!
//! [`RxPin`] and [`TxPin`] check the port's pins at compile time.
//! [`Serial::split`] separates the receiver from the transmitter. Both work
//! blocking, through `embedded-io`, or async, through `embedded-io-async`,
//! once the port's interrupt vector calls [`on_interrupt`].
//!
//! See chapters 34 and 35 of the SAM3X/SAM3A datasheet for more details.

use core::convert::Infallible;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::task::Poll;

use crate::clocks::Clocks;
use crate::gpio::{Peripheral, Pin, A, B, PA10, PA11, PA12, PA13, PA8, PA9, PB20, PB21, PD4, PD5};
use crate::pac;
use crate::pdc;
use crate::waker::AtomicWaker;

mod sealed {
    pub trait Sealed {}
}

/// Parity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Parity {
    /// No parity bit.
    None,
    /// Even.
    Even,
    /// Odd.
    Odd,
    /// Always 1.
    Mark,
    /// Always 0.
    Space,
}

/// Stop bits. The UART only does one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StopBits {
    /// One.
    One,
    /// One and a half.
    OneAndHalf,
    /// Two.
    Two,
}

/// Data bits per character. The UART only does eight.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DataBits {
    /// Five.
    Five,
    /// Six.
    Six,
    /// Seven.
    Seven,
    /// Eight.
    Eight,
}

/// A port's line settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// Bits per second.
    pub baud: u32,
    /// Data bits per character.
    pub data_bits: DataBits,
    /// Parity.
    pub parity: Parity,
    /// Stop bits.
    pub stop_bits: StopBits,
}

impl Config {
    /// `baud`, 8N1.
    pub const fn new(baud: u32) -> Self {
        Config {
            baud,
            data_bits: DataBits::Eight,
            parity: Parity::None,
            stop_bits: StopBits::One,
        }
    }

    /// The same, with this parity.
    pub const fn parity(self, parity: Parity) -> Self {
        Config { parity, ..self }
    }

    /// The same, with these stop bits.
    pub const fn stop_bits(self, stop_bits: StopBits) -> Self {
        Config { stop_bits, ..self }
    }

    /// The same, with this many data bits.
    pub const fn data_bits(self, data_bits: DataBits) -> Self {
        Config { data_bits, ..self }
    }
}

impl Default for Config {
    /// 115200 8N1.
    fn default() -> Self {
        Config::new(115_200)
    }
}

/// A [`Config`] the instance cannot run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigError {
    /// The baud rate needs a divisor outside 1 to 65535.
    BaudOutOfRange,
    /// The UART only supports eight data bits and one stop bit.
    Unsupported,
}

/// A receive error. Reading clears the flags, so each error is reported
/// once.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// A byte arrived before the previous one was read.
    Overrun,
    /// The stop bit is missing.
    Framing,
    /// The parity bit does not match.
    Parity,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Error::Overrun => "receiver overrun",
            Error::Framing => "framing error",
            Error::Parity => "parity error",
        })
    }
}

impl core::error::Error for Error {}

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

/// The interrupt flags that are both set and enabled.
#[derive(Clone, Copy, Debug, Default)]
pub struct Pending {
    /// Receive ready.
    pub rx_ready: bool,
    /// Transmit ready.
    pub tx_ready: bool,
    /// End of a PDC receive transfer.
    pub rx_done: bool,
    /// End of a PDC transmit transfer.
    pub tx_done: bool,
}

/// The wakers of an instance's receiver and transmitter, one static per
/// instance, woken by [`on_interrupt`].
pub struct State {
    rx: AtomicWaker,
    tx: AtomicWaker,
}

impl State {
    const fn new() -> Self {
        State {
            rx: AtomicWaker::new(),
            tx: AtomicWaker::new(),
        }
    }
}

/// One of the five serial instances.
pub trait Instance: sealed::Sealed + pdc::Instance {
    /// Peripheral ID, also the interrupt number.
    const PID: u8;
    /// The instance's wakers.
    fn state() -> &'static State;
    /// Reset both directions and the status, mask every interrupt.
    fn reset();
    /// Program the line settings.
    fn configure(config: &Config, clocks: &Clocks) -> Result<(), ConfigError>;
    /// Enable both directions.
    fn enable();
    /// Disable both directions.
    fn disable();
    /// The holding register can take a byte.
    fn tx_ready() -> bool;
    /// Everything written has left the shift register.
    fn tx_empty() -> bool;
    /// A byte is waiting.
    fn rx_ready() -> bool;
    /// Write a byte to the transmitter. Only when [`tx_ready`](Self::tx_ready).
    fn write(byte: u8);
    /// Read the received byte. Only when [`rx_ready`](Self::rx_ready).
    fn read() -> u8;
    /// Read and clear the receiver's error flags.
    fn take_error() -> Option<Error>;
    /// Enable the receive interrupt.
    fn set_rx_interrupt(enabled: bool);
    /// Enable the transmit interrupt.
    fn set_tx_interrupt(enabled: bool);
    /// The flags that are both set and unmasked.
    fn pending() -> Pending;
}

fn divisor(baud: u32, mck_hz: u32) -> Result<u32, ConfigError> {
    if baud == 0 {
        return Err(ConfigError::BaudOutOfRange);
    }
    // The divisor times eight: the low three bits are the fractional part.
    let x8 = (mck_hz as u64 * 8 / (16 * baud as u64)) as u32;
    if !(8..=0xFFFF * 8 + 7).contains(&x8) {
        return Err(ConfigError::BaudOutOfRange);
    }
    Ok(x8)
}

macro_rules! instance {
    (uart $Type:ty, $pid:literal) => {
        impl sealed::Sealed for $Type {}
        impl Instance for $Type {
            const PID: u8 = $pid;

            fn state() -> &'static State {
                static STATE: State = State::new();
                &STATE
            }

            fn reset() {
                // SAFETY: a fixed MMIO address, mapped for the whole run.
                let r = unsafe { &*<$Type>::ptr() };
                // SAFETY: CR, IDR and PTCR are write-only command registers.
                unsafe {
                    r.idr().write_with_zero(|w| w.bits(0xFFFF_FFFF));
                    r.ptcr()
                        .write_with_zero(|w| w.rxtdis().set_bit().txtdis().set_bit());
                    r.cr().write_with_zero(|w| {
                        w.rstrx().set_bit();
                        w.rsttx().set_bit();
                        w.rststa().set_bit();
                        w.rxdis().set_bit();
                        w.txdis().set_bit()
                    });
                }
            }

            fn configure(config: &Config, clocks: &Clocks) -> Result<(), ConfigError> {
                if config.data_bits != DataBits::Eight || config.stop_bits != StopBits::One {
                    return Err(ConfigError::Unsupported);
                }
                let cd = divisor(config.baud, clocks.mck_hz)? / 8;
                let r = unsafe { &*<$Type>::ptr() };
                r.mr().write(|w| {
                    match config.parity {
                        Parity::None => w.par().no(),
                        Parity::Even => w.par().even(),
                        Parity::Odd => w.par().odd(),
                        Parity::Mark => w.par().mark(),
                        Parity::Space => w.par().space(),
                    };
                    w.chmode().normal()
                });
                r.brgr().write(|w| unsafe { w.cd().bits(cd as u16) });
                Ok(())
            }

            fn enable() {
                let r = unsafe { &*<$Type>::ptr() };
                unsafe {
                    r.cr()
                        .write_with_zero(|w| w.rxen().set_bit().txen().set_bit());
                }
            }

            fn disable() {
                let r = unsafe { &*<$Type>::ptr() };
                unsafe {
                    r.cr()
                        .write_with_zero(|w| w.rxdis().set_bit().txdis().set_bit());
                }
            }

            fn tx_ready() -> bool {
                unsafe { &*<$Type>::ptr() }.sr().read().txrdy().bit_is_set()
            }

            fn tx_empty() -> bool {
                unsafe { &*<$Type>::ptr() }
                    .sr()
                    .read()
                    .txempty()
                    .bit_is_set()
            }

            fn rx_ready() -> bool {
                unsafe { &*<$Type>::ptr() }.sr().read().rxrdy().bit_is_set()
            }

            fn write(byte: u8) {
                let r = unsafe { &*<$Type>::ptr() };
                unsafe { r.thr().write_with_zero(|w| w.txchr().bits(byte)) };
            }

            fn read() -> u8 {
                unsafe { &*<$Type>::ptr() }.rhr().read().rxchr().bits()
            }

            fn take_error() -> Option<Error> {
                let r = unsafe { &*<$Type>::ptr() };
                let sr = r.sr().read();
                let error = if sr.ovre().bit_is_set() {
                    Some(Error::Overrun)
                } else if sr.frame().bit_is_set() {
                    Some(Error::Framing)
                } else if sr.pare().bit_is_set() {
                    Some(Error::Parity)
                } else {
                    None
                };
                if error.is_some() {
                    unsafe { r.cr().write_with_zero(|w| w.rststa().set_bit()) };
                }
                error
            }

            fn set_rx_interrupt(enabled: bool) {
                let r = unsafe { &*<$Type>::ptr() };
                unsafe {
                    if enabled {
                        r.ier().write_with_zero(|w| w.rxrdy().set_bit());
                    } else {
                        r.idr().write_with_zero(|w| w.rxrdy().set_bit());
                    }
                }
            }

            fn set_tx_interrupt(enabled: bool) {
                let r = unsafe { &*<$Type>::ptr() };
                unsafe {
                    if enabled {
                        r.ier().write_with_zero(|w| w.txrdy().set_bit());
                    } else {
                        r.idr().write_with_zero(|w| w.txrdy().set_bit());
                    }
                }
            }

            fn pending() -> Pending {
                let r = unsafe { &*<$Type>::ptr() };
                let sr = r.sr().read();
                let imr = r.imr().read();
                Pending {
                    rx_ready: sr.rxrdy().bit_is_set() && imr.rxrdy().bit_is_set(),
                    tx_ready: sr.txrdy().bit_is_set() && imr.txrdy().bit_is_set(),
                    rx_done: sr.endrx().bit_is_set() && imr.endrx().bit_is_set(),
                    tx_done: sr.endtx().bit_is_set() && imr.endtx().bit_is_set(),
                }
            }
        }
    };
    (usart $Type:ty, $pid:literal) => {
        impl sealed::Sealed for $Type {}
        impl Instance for $Type {
            const PID: u8 = $pid;

            fn state() -> &'static State {
                static STATE: State = State::new();
                &STATE
            }

            fn reset() {
                let r = unsafe { &*<$Type>::ptr() };
                unsafe {
                    r.idr().write_with_zero(|w| w.bits(0xFFFF_FFFF));
                    r.ptcr()
                        .write_with_zero(|w| w.rxtdis().set_bit().txtdis().set_bit());
                    r.cr().write_with_zero(|w| {
                        w.rstrx().set_bit();
                        w.rsttx().set_bit();
                        w.rststa().set_bit();
                        w.rxdis().set_bit();
                        w.txdis().set_bit()
                    });
                }
            }

            fn configure(config: &Config, clocks: &Clocks) -> Result<(), ConfigError> {
                let x8 = divisor(config.baud, clocks.mck_hz)?;
                let r = unsafe { &*<$Type>::ptr() };
                // SAFETY: MR has no reset value in the SVD; every field
                // that matters is written here.
                unsafe {
                    r.mr().write_with_zero(|w| {
                        w.usart_mode().normal();
                        w.usclks().mck();
                        match config.data_bits {
                            DataBits::Five => w.chrl()._5_bit(),
                            DataBits::Six => w.chrl()._6_bit(),
                            DataBits::Seven => w.chrl()._7_bit(),
                            DataBits::Eight => w.chrl()._8_bit(),
                        };
                        match config.parity {
                            Parity::None => w.par().no(),
                            Parity::Even => w.par().even(),
                            Parity::Odd => w.par().odd(),
                            Parity::Mark => w.par().mark(),
                            Parity::Space => w.par().space(),
                        };
                        match config.stop_bits {
                            StopBits::One => w.nbstop()._1_bit(),
                            StopBits::OneAndHalf => w.nbstop()._1_5_bit(),
                            StopBits::Two => w.nbstop()._2_bit(),
                        };
                        w.chmode().normal()
                    });
                }
                r.brgr().write(|w| unsafe {
                    w.cd().bits((x8 / 8) as u16);
                    w.fp().bits((x8 % 8) as u8)
                });
                Ok(())
            }

            fn enable() {
                let r = unsafe { &*<$Type>::ptr() };
                unsafe {
                    r.cr()
                        .write_with_zero(|w| w.rxen().set_bit().txen().set_bit());
                }
            }

            fn disable() {
                let r = unsafe { &*<$Type>::ptr() };
                unsafe {
                    r.cr()
                        .write_with_zero(|w| w.rxdis().set_bit().txdis().set_bit());
                }
            }

            fn tx_ready() -> bool {
                unsafe { &*<$Type>::ptr() }
                    .csr()
                    .read()
                    .txrdy()
                    .bit_is_set()
            }

            fn tx_empty() -> bool {
                unsafe { &*<$Type>::ptr() }
                    .csr()
                    .read()
                    .txempty()
                    .bit_is_set()
            }

            fn rx_ready() -> bool {
                unsafe { &*<$Type>::ptr() }
                    .csr()
                    .read()
                    .rxrdy()
                    .bit_is_set()
            }

            fn write(byte: u8) {
                let r = unsafe { &*<$Type>::ptr() };
                unsafe { r.thr().write_with_zero(|w| w.txchr().bits(byte as u16)) };
            }

            fn read() -> u8 {
                unsafe { &*<$Type>::ptr() }.rhr().read().rxchr().bits() as u8
            }

            fn take_error() -> Option<Error> {
                let r = unsafe { &*<$Type>::ptr() };
                let sr = r.csr().read();
                let error = if sr.ovre().bit_is_set() {
                    Some(Error::Overrun)
                } else if sr.frame().bit_is_set() {
                    Some(Error::Framing)
                } else if sr.pare().bit_is_set() {
                    Some(Error::Parity)
                } else {
                    None
                };
                if error.is_some() {
                    unsafe { r.cr().write_with_zero(|w| w.rststa().set_bit()) };
                }
                error
            }

            fn set_rx_interrupt(enabled: bool) {
                let r = unsafe { &*<$Type>::ptr() };
                unsafe {
                    if enabled {
                        r.ier().write_with_zero(|w| w.rxrdy().set_bit());
                    } else {
                        r.idr().write_with_zero(|w| w.rxrdy().set_bit());
                    }
                }
            }

            fn set_tx_interrupt(enabled: bool) {
                let r = unsafe { &*<$Type>::ptr() };
                unsafe {
                    if enabled {
                        r.ier().write_with_zero(|w| w.txrdy().set_bit());
                    } else {
                        r.idr().write_with_zero(|w| w.txrdy().set_bit());
                    }
                }
            }

            fn pending() -> Pending {
                let r = unsafe { &*<$Type>::ptr() };
                let sr = r.csr().read();
                let imr = r.imr().read();
                Pending {
                    rx_ready: sr.rxrdy().bit_is_set() && imr.rxrdy().bit_is_set(),
                    tx_ready: sr.txrdy().bit_is_set() && imr.txrdy().bit_is_set(),
                    rx_done: sr.endrx().bit_is_set() && imr.endrx().bit_is_set(),
                    tx_done: sr.endtx().bit_is_set() && imr.endtx().bit_is_set(),
                }
            }
        }
    };
}

instance!(uart pac::Uart, 8);
instance!(usart pac::Usart0, 17);
instance!(usart pac::Usart1, 18);
instance!(usart pac::Usart2, 19);
instance!(usart pac::Usart3, 20);

/// Interrupt handler for async use: disable the interrupts that fired and
/// wake the waiting half. Does nothing when no task waits.
pub fn on_interrupt<I: Instance>() {
    let pending = I::pending();
    let state = I::state();
    if pending.rx_ready {
        I::set_rx_interrupt(false);
        state.rx.wake();
    }
    if pending.tx_ready {
        I::set_tx_interrupt(false);
        state.tx.wake();
    }
    if pending.rx_done {
        <I as pdc::Instance>::set_rx_done_interrupt(false);
        state.rx.wake();
    }
    if pending.tx_done {
        <I as pdc::Instance>::set_tx_done_interrupt(false);
        state.tx.wake();
    }
}

/// A pin an instance receives on.
pub trait RxPin<I: Instance>: sealed::Sealed {}
/// A pin an instance transmits on.
pub trait TxPin<I: Instance>: sealed::Sealed {}

macro_rules! pins {
    ($( $I:ty : rx $Rx:ident $RxF:ident, tx $Tx:ident $TxF:ident; )*) => {
        $(
            impl sealed::Sealed for Pin<$Rx, Peripheral<$RxF>> {}
            impl RxPin<$I> for Pin<$Rx, Peripheral<$RxF>> {}
            impl sealed::Sealed for Pin<$Tx, Peripheral<$TxF>> {}
            impl TxPin<$I> for Pin<$Tx, Peripheral<$TxF>> {}
        )*
    };
}

pins! {
    pac::Uart: rx PA8 A, tx PA9 A;
    pac::Usart0: rx PA10 A, tx PA11 A;
    pac::Usart1: rx PA12 A, tx PA13 A;
    pac::Usart2: rx PB21 A, tx PB20 A;
    pac::Usart3: rx PD5 B, tx PD4 B;
}

/// The UART's pins, PA8 (RX) and PA9 (TX) on peripheral A.
pub type UartPins = (Pin<PA8, Peripheral<A>>, Pin<PA9, Peripheral<A>>);

fn enable_clock(pmc: &pac::Pmc, pid: u8) {
    // SAFETY: a write-only enable register, one bit per peripheral; every
    // serial PID is below 32.
    unsafe { pmc.pmc_pcer0().write_with_zero(|w| w.bits(1 << pid)) };
}

/// The receiving half.
pub struct Rx<I: Instance> {
    _instance: PhantomData<I>,
}

/// The transmitting half.
pub struct Tx<I: Instance> {
    _instance: PhantomData<I>,
}

/// A serial port: an instance, its two pins, both directions enabled.
pub struct Serial<I: Instance, RX: RxPin<I>, TX: TxPin<I>> {
    instance: I,
    pins: (RX, TX),
    rx: Rx<I>,
    tx: Tx<I>,
}

impl<I: Instance, RX: RxPin<I>, TX: TxPin<I>> Serial<I, RX, TX> {
    /// Enable the peripheral clock, apply `config` and enable both
    /// directions. The pins must already be on the port's peripheral
    /// function.
    pub fn new(
        instance: I,
        pins: (RX, TX),
        config: &Config,
        clocks: &Clocks,
        pmc: &pac::Pmc,
    ) -> Result<Self, ConfigError> {
        enable_clock(pmc, I::PID);
        I::reset();
        I::configure(config, clocks)?;
        I::enable();
        Ok(Serial {
            instance,
            pins,
            rx: Rx {
                _instance: PhantomData,
            },
            tx: Tx {
                _instance: PhantomData,
            },
        })
    }

    /// The port's NVIC interrupt.
    pub const fn interrupt(&self) -> pac::Interrupt {
        I::INTERRUPT
    }

    /// Split into receiver and transmitter. The port cannot be
    /// [`free`](Self::free)d afterwards.
    pub fn split(self) -> (Rx<I>, Tx<I>) {
        (self.rx, self.tx)
    }

    /// Disable the port and return the instance and the pins.
    pub fn free(self) -> (I, (RX, TX)) {
        I::reset();
        (self.instance, self.pins)
    }

    /// Wait until the transmitter is ready, then send `byte`.
    pub fn write_byte(&mut self, byte: u8) {
        self.tx.write_byte(byte);
    }

    /// Wait until every written byte has been sent.
    pub fn flush(&mut self) {
        self.tx.flush();
    }

    /// The received byte, if any.
    pub fn read_byte(&mut self) -> Option<u8> {
        self.rx.read_byte()
    }

    /// Read and clear the receiver's last error.
    pub fn take_error(&mut self) -> Option<Error> {
        self.rx.take_error()
    }

    /// Enable the receive interrupt.
    pub fn enable_rx_interrupt(&mut self) {
        self.rx.enable_interrupt();
    }

    /// Disable the receive interrupt.
    pub fn disable_rx_interrupt(&mut self) {
        self.rx.disable_interrupt();
    }
}

impl<I: Instance> Tx<I> {
    /// The port's NVIC interrupt.
    pub const fn interrupt(&self) -> pac::Interrupt {
        I::INTERRUPT
    }

    /// Wait until the transmitter is ready, then send `byte`.
    pub fn write_byte(&mut self, byte: u8) {
        while !I::tx_ready() {}
        I::write(byte);
    }

    /// Wait until every written byte has been sent.
    pub fn flush(&mut self) {
        while !I::tx_empty() {}
    }

    /// Enable the transmit interrupt.
    pub fn enable_interrupt(&mut self) {
        I::set_tx_interrupt(true);
    }

    /// Disable the transmit interrupt.
    pub fn disable_interrupt(&mut self) {
        I::set_tx_interrupt(false);
    }

    /// Wait for the transmitter to be ready, then send as many bytes as it
    /// accepts without waiting again (one or two). Needs the interrupt
    /// vector to call [`on_interrupt`] and the NVIC line unmasked.
    pub async fn write_async(&mut self, buf: &[u8]) -> usize {
        if buf.is_empty() {
            return 0;
        }
        poll_fn(|cx| {
            if I::tx_ready() {
                return Poll::Ready(());
            }
            I::state().tx.register(cx.waker());
            I::set_tx_interrupt(true);
            // The transmitter may have become ready before the interrupt
            // was enabled.
            if I::tx_ready() {
                I::set_tx_interrupt(false);
                return Poll::Ready(());
            }
            Poll::Pending
        })
        .await;
        let mut n = 0;
        while n < buf.len() && I::tx_ready() {
            I::write(buf[n]);
            n += 1;
        }
        n
    }

    /// Send every byte, waiting on the interrupt when the transmitter is
    /// busy.
    pub async fn write_all_async(&mut self, mut buf: &[u8]) {
        while !buf.is_empty() {
            let n = self.write_async(buf).await;
            buf = &buf[n..];
        }
    }
}

impl<I: Instance> Rx<I> {
    /// The port's NVIC interrupt.
    pub const fn interrupt(&self) -> pac::Interrupt {
        I::INTERRUPT
    }

    /// The received byte, if any.
    pub fn read_byte(&mut self) -> Option<u8> {
        if I::rx_ready() {
            Some(I::read())
        } else {
            None
        }
    }

    /// Read and clear the receiver's last error.
    pub fn take_error(&mut self) -> Option<Error> {
        I::take_error()
    }

    /// Enable the receive interrupt.
    pub fn enable_interrupt(&mut self) {
        I::set_rx_interrupt(true);
    }

    /// Disable the receive interrupt.
    pub fn disable_interrupt(&mut self) {
        I::set_rx_interrupt(false);
    }

    /// Read the received bytes into `buf`, up to its length.
    fn drain(buf: &mut [u8]) -> usize {
        let mut n = 0;
        while n < buf.len() && I::rx_ready() {
            buf[n] = I::read();
            n += 1;
        }
        n
    }

    /// Wait for the first byte, then read the received bytes into `buf`, up
    /// to its length. A receive error is returned once, instead of bytes.
    /// Needs the interrupt vector to call [`on_interrupt`] and the NVIC line
    /// unmasked.
    pub async fn read_async(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        poll_fn(|cx| {
            if let Some(error) = I::take_error() {
                return Poll::Ready(Err(error));
            }
            if I::rx_ready() {
                return Poll::Ready(Ok(Self::drain(buf)));
            }
            I::state().rx.register(cx.waker());
            I::set_rx_interrupt(true);
            // A byte may have arrived before the interrupt was enabled.
            if I::rx_ready() {
                I::set_rx_interrupt(false);
                return Poll::Ready(Ok(Self::drain(buf)));
            }
            Poll::Pending
        })
        .await
    }
}

impl<I: Instance> embedded_io::ErrorType for Tx<I> {
    type Error = Infallible;
}

impl<I: Instance> embedded_io::Write for Tx<I> {
    /// Send every byte unchanged, `\n` included.
    fn write(&mut self, buf: &[u8]) -> Result<usize, Infallible> {
        for &byte in buf {
            self.write_byte(byte);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Infallible> {
        Tx::flush(self);
        Ok(())
    }
}

impl<I: Instance> embedded_io_async::Write for Tx<I> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Infallible> {
        Ok(self.write_async(buf).await)
    }

    /// Busy-waits: the last byte is sent within one character time.
    async fn flush(&mut self) -> Result<(), Infallible> {
        Tx::flush(self);
        Ok(())
    }
}

impl<I: Instance> core::fmt::Write for Tx<I> {
    /// Send text for a terminal: each `\n` is sent as CR+LF. The
    /// `embedded_io::Write` implementation sends bytes unchanged.
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &byte in s.as_bytes() {
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
        Ok(())
    }
}

impl<I: Instance> embedded_io::ErrorType for Rx<I> {
    type Error = Error;
}

impl<I: Instance> embedded_io::ReadReady for Rx<I> {
    fn read_ready(&mut self) -> Result<bool, Error> {
        Ok(I::rx_ready())
    }
}

impl<I: Instance> embedded_io::Read for Rx<I> {
    /// Block until the first byte, then read the received bytes into `buf`,
    /// up to its length. A receive error is returned once, instead of
    /// bytes.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        while !I::rx_ready() {
            if let Some(error) = I::take_error() {
                return Err(error);
            }
        }
        Ok(Self::drain(buf))
    }
}

impl<I: Instance> embedded_io_async::Read for Rx<I> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.read_async(buf).await
    }
}

impl<I: Instance, RX: RxPin<I>, TX: TxPin<I>> embedded_io::ErrorType for Serial<I, RX, TX> {
    type Error = Error;
}

impl<I: Instance, RX: RxPin<I>, TX: TxPin<I>> embedded_io::Write for Serial<I, RX, TX> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        embedded_io::Write::write(&mut self.tx, buf).map_err(|e| match e {})
    }

    fn flush(&mut self) -> Result<(), Error> {
        Tx::flush(&mut self.tx);
        Ok(())
    }
}

impl<I: Instance, RX: RxPin<I>, TX: TxPin<I>> embedded_io::Read for Serial<I, RX, TX> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        embedded_io::Read::read(&mut self.rx, buf)
    }
}

impl<I: Instance, RX: RxPin<I>, TX: TxPin<I>> embedded_io::ReadReady for Serial<I, RX, TX> {
    fn read_ready(&mut self) -> Result<bool, Error> {
        Ok(I::rx_ready())
    }
}

impl<I: Instance, RX: RxPin<I>, TX: TxPin<I>> core::fmt::Write for Serial<I, RX, TX> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.tx.write_str(s)
    }
}

// PDC targets: a transfer owns the half while it runs, and the
// end-of-transfer interrupt wakes the half's waker through `on_interrupt`.
impl<I: Instance> pdc::sealed::Sealed for Tx<I> {}
impl<I: Instance> pdc::Target for Tx<I> {
    type Instance = I;
    type Word = u8;
    fn waker() -> &'static AtomicWaker {
        &I::state().tx
    }
}
impl<I: Instance> pdc::TxTarget for Tx<I> {}

impl<I: Instance> pdc::sealed::Sealed for Rx<I> {}
impl<I: Instance> pdc::Target for Rx<I> {
    type Instance = I;
    type Word = u8;
    fn waker() -> &'static AtomicWaker {
        &I::state().rx
    }
}
impl<I: Instance> pdc::RxTarget for Rx<I> {}
