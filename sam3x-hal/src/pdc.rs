//! Peripheral DMA Controller (PDC)
//!
//! The PDC moves data between memory and a peripheral without the core. A
//! transfer is a memory pointer and a count; when the count reaches zero the
//! peripheral raises its end-of-transfer flag, and its interrupt if enabled.
//!
//! On the SAM3X the PDC is not a central controller but a channel pair built
//! into each peripheral that has one, with its registers in that peripheral's
//! register map. This module drives the PDC of the UART, the four USARTs, the
//! two TWIs, the ADC (receive only) and the DACC (transmit only). The SPI has
//! no PDC on the SAM3X and goes through the DMA controller instead. The count
//! is 16 bits wide, so one transfer moves at most 65535 words.
//!
//! [`write()`] and [`read()`] take a buffer and the owned half of a
//! peripheral, start the transfer and return a [`Transfer`] that owns both
//! until [`Transfer::wait`], [`Transfer::wait_async`] or [`Transfer::abort`]
//! hands them back. Buffers use the `embedded-dma` traits, which only accept
//! memory that stays in place during the transfer.
//!
//! The PDC has no access to the internal flash (bus matrix, table 21-3): a
//! buffer in flash, such as a `static` byte string, must be copied to RAM
//! first. [`write()`] rejects it with [`Error::InFlash`].
//!
//! See chapter 26 of the SAM3X/SAM3A datasheet for more details.
//!

use core::fmt;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::{compiler_fence, Ordering};
use core::task::Poll;

use embedded_dma::{ReadBuffer, WriteBuffer};

use crate::pac;
use crate::waker::AtomicWaker;

pub(crate) mod sealed {
    pub trait Sealed {}
}

/// A peripheral with a PDC block.
pub trait Instance: sealed::Sealed {
    /// The peripheral's NVIC interrupt, which signals the end of transfers.
    const INTERRUPT: pac::Interrupt;
    /// Whether the peripheral has a PDC transmit channel. The ADC has none.
    const HAS_TX: bool;
    /// Point the receive channel at `ptr` for `count` bytes.
    fn set_rx(ptr: u32, count: u16);
    /// Point the transmit channel at `ptr` for `count` bytes.
    fn set_tx(ptr: u32, count: u16);
    /// Bytes left on the receive channel.
    fn rx_remaining() -> u16;
    /// Bytes left on the transmit channel.
    fn tx_remaining() -> u16;
    /// Start or stop the receive channel.
    fn set_rx_enabled(enabled: bool);
    /// Start or stop the transmit channel.
    fn set_tx_enabled(enabled: bool);
    /// The receive count reached zero (`ENDRX`).
    fn rx_done() -> bool;
    /// The transmit count reached zero (`ENDTX`).
    fn tx_done() -> bool;
    /// Enable or disable the `ENDRX` interrupt.
    fn set_rx_done_interrupt(enabled: bool);
    /// Enable or disable the `ENDTX` interrupt.
    fn set_tx_done_interrupt(enabled: bool);
}

macro_rules! instance {
    ($Type:ty, $status:ident, $irq:ident, tx) => {
        impl sealed::Sealed for $Type {}
        impl Instance for $Type {
            const INTERRUPT: pac::Interrupt = pac::Interrupt::$irq;
            const HAS_TX: bool = true;

            fn set_rx(ptr: u32, count: u16) {
                // SAFETY: a fixed MMIO address, mapped for the whole run.
                let r = unsafe { &*<$Type>::ptr() };
                r.rpr().write(|w| unsafe { w.rxptr().bits(ptr) });
                r.rcr().write(|w| unsafe { w.rxctr().bits(count.into()) });
            }

            fn set_tx(ptr: u32, count: u16) {
                let r = unsafe { &*<$Type>::ptr() };
                r.tpr().write(|w| unsafe { w.txptr().bits(ptr) });
                r.tcr().write(|w| unsafe { w.txctr().bits(count.into()) });
            }

            fn rx_remaining() -> u16 {
                unsafe { &*<$Type>::ptr() }.rcr().read().rxctr().bits() as u16
            }

            fn tx_remaining() -> u16 {
                unsafe { &*<$Type>::ptr() }.tcr().read().txctr().bits() as u16
            }

            fn set_rx_enabled(enabled: bool) {
                let r = unsafe { &*<$Type>::ptr() };
                // SAFETY: PTCR is a write-only command register.
                unsafe {
                    if enabled {
                        r.ptcr().write_with_zero(|w| w.rxten().set_bit());
                    } else {
                        r.ptcr().write_with_zero(|w| w.rxtdis().set_bit());
                    }
                }
            }

            fn set_tx_enabled(enabled: bool) {
                let r = unsafe { &*<$Type>::ptr() };
                unsafe {
                    if enabled {
                        r.ptcr().write_with_zero(|w| w.txten().set_bit());
                    } else {
                        r.ptcr().write_with_zero(|w| w.txtdis().set_bit());
                    }
                }
            }

            fn rx_done() -> bool {
                unsafe { &*<$Type>::ptr() }
                    .$status()
                    .read()
                    .endrx()
                    .bit_is_set()
            }

            fn tx_done() -> bool {
                unsafe { &*<$Type>::ptr() }
                    .$status()
                    .read()
                    .endtx()
                    .bit_is_set()
            }

            fn set_rx_done_interrupt(enabled: bool) {
                let r = unsafe { &*<$Type>::ptr() };
                // SAFETY: IER and IDR are write-only enable registers.
                unsafe {
                    if enabled {
                        r.ier().write_with_zero(|w| w.endrx().set_bit());
                    } else {
                        r.idr().write_with_zero(|w| w.endrx().set_bit());
                    }
                }
            }

            fn set_tx_done_interrupt(enabled: bool) {
                let r = unsafe { &*<$Type>::ptr() };
                unsafe {
                    if enabled {
                        r.ier().write_with_zero(|w| w.endtx().set_bit());
                    } else {
                        r.idr().write_with_zero(|w| w.endtx().set_bit());
                    }
                }
            }
        }
    };
}

macro_rules! rx_only_instance {
    ($Type:ty, $status:ident, $irq:ident) => {
        impl sealed::Sealed for $Type {}
        impl Instance for $Type {
            const INTERRUPT: pac::Interrupt = pac::Interrupt::$irq;
            const HAS_TX: bool = false;

            fn set_rx(ptr: u32, count: u16) {
                // SAFETY: a fixed MMIO address, mapped for the whole run.
                let r = unsafe { &*<$Type>::ptr() };
                r.rpr().write(|w| unsafe { w.rxptr().bits(ptr) });
                r.rcr().write(|w| unsafe { w.rxctr().bits(count.into()) });
            }

            fn set_tx(_ptr: u32, _count: u16) {}

            fn rx_remaining() -> u16 {
                unsafe { &*<$Type>::ptr() }.rcr().read().rxctr().bits() as u16
            }

            fn tx_remaining() -> u16 {
                0
            }

            fn set_rx_enabled(enabled: bool) {
                let r = unsafe { &*<$Type>::ptr() };
                // SAFETY: PTCR is a write-only command register.
                unsafe {
                    if enabled {
                        r.ptcr().write_with_zero(|w| w.rxten().set_bit());
                    } else {
                        r.ptcr().write_with_zero(|w| w.rxtdis().set_bit());
                    }
                }
            }

            fn set_tx_enabled(_enabled: bool) {}

            fn rx_done() -> bool {
                unsafe { &*<$Type>::ptr() }
                    .$status()
                    .read()
                    .endrx()
                    .bit_is_set()
            }

            fn tx_done() -> bool {
                false
            }

            fn set_rx_done_interrupt(enabled: bool) {
                let r = unsafe { &*<$Type>::ptr() };
                // SAFETY: IER and IDR are write-only enable registers.
                unsafe {
                    if enabled {
                        r.ier().write_with_zero(|w| w.endrx().set_bit());
                    } else {
                        r.idr().write_with_zero(|w| w.endrx().set_bit());
                    }
                }
            }

            fn set_tx_done_interrupt(_enabled: bool) {}
        }
    };
}

macro_rules! tx_only_instance {
    ($Type:ty, $status:ident, $irq:ident) => {
        impl sealed::Sealed for $Type {}
        impl Instance for $Type {
            const INTERRUPT: pac::Interrupt = pac::Interrupt::$irq;
            const HAS_TX: bool = true;

            fn set_rx(_ptr: u32, _count: u16) {}

            fn set_tx(ptr: u32, count: u16) {
                // SAFETY: a fixed MMIO address, mapped for the whole run.
                let r = unsafe { &*<$Type>::ptr() };
                r.tpr().write(|w| unsafe { w.txptr().bits(ptr) });
                r.tcr().write(|w| unsafe { w.txctr().bits(count.into()) });
            }

            fn rx_remaining() -> u16 {
                0
            }

            fn tx_remaining() -> u16 {
                unsafe { &*<$Type>::ptr() }.tcr().read().txctr().bits() as u16
            }

            fn set_rx_enabled(_enabled: bool) {}

            fn set_tx_enabled(enabled: bool) {
                let r = unsafe { &*<$Type>::ptr() };
                // SAFETY: PTCR is a write-only command register.
                unsafe {
                    if enabled {
                        r.ptcr().write_with_zero(|w| w.txten().set_bit());
                    } else {
                        r.ptcr().write_with_zero(|w| w.txtdis().set_bit());
                    }
                }
            }

            fn rx_done() -> bool {
                false
            }

            fn tx_done() -> bool {
                unsafe { &*<$Type>::ptr() }
                    .$status()
                    .read()
                    .endtx()
                    .bit_is_set()
            }

            fn set_rx_done_interrupt(_enabled: bool) {}

            fn set_tx_done_interrupt(enabled: bool) {
                let r = unsafe { &*<$Type>::ptr() };
                // SAFETY: IER and IDR are write-only enable registers.
                unsafe {
                    if enabled {
                        r.ier().write_with_zero(|w| w.endtx().set_bit());
                    } else {
                        r.idr().write_with_zero(|w| w.endtx().set_bit());
                    }
                }
            }
        }
    };
}

instance!(pac::Uart, sr, UART, tx);
instance!(pac::Usart0, csr, USART0, tx);
instance!(pac::Usart1, csr, USART1, tx);
instance!(pac::Usart2, csr, USART2, tx);
instance!(pac::Usart3, csr, USART3, tx);
instance!(pac::Twi0, sr, TWI0, tx);
instance!(pac::Twi1, sr, TWI1, tx);
rx_only_instance!(pac::Adc, isr, ADC);
tx_only_instance!(pac::Dacc, isr, DACC);

/// An owned half of a peripheral the PDC can drive.
pub trait Target: sealed::Sealed {
    /// The peripheral behind it.
    type Instance: Instance;
    /// What one transfer moves: a byte for a serial port, a tagged sample
    /// for the ADC. The count is in words.
    type Word: Copy;
    /// The waker the peripheral's interrupt handler wakes for this half,
    /// used by [`Transfer::wait_async`].
    fn waker() -> &'static AtomicWaker;
}

/// The transmit half, used by [`write()`].
pub trait TxTarget: Target {}

/// The receive half, used by [`read()`].
pub trait RxTarget: Target {}

/// Which channel a [`Transfer`] runs on.
pub trait Direction: sealed::Sealed {
    #[doc(hidden)]
    fn done<I: Instance>() -> bool;
    #[doc(hidden)]
    fn remaining<I: Instance>() -> u16;
    #[doc(hidden)]
    fn set_enabled<I: Instance>(enabled: bool);
    #[doc(hidden)]
    fn set_interrupt<I: Instance>(enabled: bool);
}

/// Memory to the peripheral.
pub enum Tx {}
impl sealed::Sealed for Tx {}
impl Direction for Tx {
    fn done<I: Instance>() -> bool {
        I::tx_done()
    }
    fn remaining<I: Instance>() -> u16 {
        I::tx_remaining()
    }
    fn set_enabled<I: Instance>(enabled: bool) {
        I::set_tx_enabled(enabled)
    }
    fn set_interrupt<I: Instance>(enabled: bool) {
        I::set_tx_done_interrupt(enabled)
    }
}

/// The peripheral to memory.
pub enum Rx {}
impl sealed::Sealed for Rx {}
impl Direction for Rx {
    fn done<I: Instance>() -> bool {
        I::rx_done()
    }
    fn remaining<I: Instance>() -> u16 {
        I::rx_remaining()
    }
    fn set_enabled<I: Instance>(enabled: bool) {
        I::set_rx_enabled(enabled)
    }
    fn set_interrupt<I: Instance>(enabled: bool) {
        I::set_rx_done_interrupt(enabled)
    }
}

/// Why a transfer was not started.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The buffer is empty.
    Empty,
    /// The buffer is longer than the 16-bit count holds, 65535 bytes.
    TooLong,
    /// The buffer is in the code region (flash or ROM, below 0x2000_0000),
    /// which the PDC cannot read.
    InFlash,
}

/// A transfer that did not start, with the buffer and the peripheral half.
pub struct Rejected<B, T> {
    /// The buffer.
    pub buf: B,
    /// The peripheral half.
    pub target: T,
    /// The reason.
    pub error: Error,
}

impl<B, T> fmt::Debug for Rejected<B, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rejected")
            .field("error", &self.error)
            .finish()
    }
}

/// A running transfer. It owns the buffer and the peripheral half until it
/// completes or is aborted.
pub struct Transfer<B, T, D: Direction> {
    buf: B,
    target: T,
    len: u16,
    _direction: PhantomData<D>,
}

/// Start of the SRAM. Below it, the code region holds the boot memory, the
/// ROM and the flash.
const SRAM_START: usize = 0x2000_0000;

fn check(ptr: usize, len: usize) -> Result<u16, Error> {
    if len == 0 {
        Err(Error::Empty)
    } else if ptr < SRAM_START {
        Err(Error::InFlash)
    } else if len > u16::MAX as usize {
        Err(Error::TooLong)
    } else {
        Ok(len as u16)
    }
}

/// Send `buf` through `target`.
pub fn write<B: ReadBuffer<Word = T::Word>, T: TxTarget>(
    buf: B,
    target: T,
) -> Result<Transfer<B, T, Tx>, Rejected<B, T>> {
    // SAFETY: the transfer owns the buffer until the PDC is done with it, so
    // nothing else moves or accesses it meanwhile.
    let (ptr, len) = unsafe { buf.read_buffer() };
    let len = match check(ptr as usize, len) {
        Ok(len) => len,
        Err(error) => return Err(Rejected { buf, target, error }),
    };
    // Complete all writes to the buffer before the PDC starts reading.
    compiler_fence(Ordering::SeqCst);
    T::Instance::set_tx(ptr as u32, len);
    T::Instance::set_tx_enabled(true);
    Ok(Transfer {
        buf,
        target,
        len,
        _direction: PhantomData,
    })
}

/// Fill `buf` from `target`. The transfer ends only when the buffer is
/// full: use a shorter buffer to receive less.
pub fn read<B: WriteBuffer<Word = T::Word>, T: RxTarget>(
    mut buf: B,
    target: T,
) -> Result<Transfer<B, T, Rx>, Rejected<B, T>> {
    // SAFETY: as for `write`, with the PDC writing instead of reading.
    let (ptr, len) = unsafe { buf.write_buffer() };
    let len = match check(ptr as usize, len) {
        Ok(len) => len,
        Err(error) => return Err(Rejected { buf, target, error }),
    };
    compiler_fence(Ordering::SeqCst);
    T::Instance::set_rx(ptr as u32, len);
    T::Instance::set_rx_enabled(true);
    Ok(Transfer {
        buf,
        target,
        len,
        _direction: PhantomData,
    })
}

impl<B, T: Target, D: Direction> Transfer<B, T, D> {
    /// The count reached zero.
    pub fn is_done(&self) -> bool {
        D::done::<T::Instance>()
    }

    /// Bytes still to move.
    pub fn remaining(&self) -> u16 {
        D::remaining::<T::Instance>()
    }

    /// Bytes moved so far.
    pub fn transferred(&self) -> u16 {
        self.len - self.remaining()
    }

    /// The peripheral's NVIC interrupt.
    pub const fn interrupt(&self) -> pac::Interrupt {
        T::Instance::INTERRUPT
    }

    /// Enable the end-of-transfer interrupt. The application's handler then
    /// checks [`is_done`](Self::is_done).
    pub fn enable_interrupt(&mut self) {
        D::set_interrupt::<T::Instance>(true);
    }

    /// Disable the end-of-transfer interrupt.
    pub fn disable_interrupt(&mut self) {
        D::set_interrupt::<T::Instance>(false);
    }

    /// Busy-wait until done, then return the buffer and the peripheral half.
    pub fn wait(self) -> (B, T) {
        while !self.is_done() {}
        self.stop()
    }

    /// Wait for the end-of-transfer interrupt, then return the buffer and
    /// the peripheral half. Needs the peripheral's interrupt vector to call
    /// its `on_interrupt` and the NVIC line unmasked.
    pub async fn wait_async(self) -> (B, T) {
        poll_fn(|cx| {
            if self.is_done() {
                return Poll::Ready(());
            }
            T::waker().register(cx.waker());
            D::set_interrupt::<T::Instance>(true);
            // The transfer may have ended before the interrupt was enabled.
            if self.is_done() {
                D::set_interrupt::<T::Instance>(false);
                return Poll::Ready(());
            }
            Poll::Pending
        })
        .await;
        self.stop()
    }

    /// Stop the transfer and return the buffer, the peripheral half and the
    /// number of bytes moved.
    pub fn abort(self) -> (B, T, u16) {
        let moved = self.transferred();
        let (buf, target) = self.stop();
        (buf, target, moved)
    }

    fn stop(self) -> (B, T) {
        D::set_interrupt::<T::Instance>(false);
        D::set_enabled::<T::Instance>(false);
        // Do not access the buffer before the PDC has stopped.
        compiler_fence(Ordering::SeqCst);
        (self.buf, self.target)
    }
}
