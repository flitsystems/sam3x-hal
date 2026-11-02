//! Waker storage for async drivers
//!
//! An async driver's future stores its task's waker in an [`AtomicWaker`]
//! before it waits for the peripheral. The interrupt handler calls
//! [`AtomicWaker::wake`] when the event arrives, and the executor polls the
//! future again.
//!
//! The slot is guarded by a `critical-section` mutex, so it works with any
//! executor and pulls no async runtime into the HAL.

use core::cell::Cell;
use core::task::Waker;

use critical_section::Mutex;

/// A slot for one waker.
pub struct AtomicWaker {
    waker: Mutex<Cell<Option<Waker>>>,
}

impl AtomicWaker {
    /// Create an empty slot.
    pub const fn new() -> Self {
        AtomicWaker {
            waker: Mutex::new(Cell::new(None)),
        }
    }

    /// Store `waker`, replacing any earlier one unless both wake the same
    /// task.
    pub fn register(&self, waker: &Waker) {
        critical_section::with(|cs| {
            let slot = self.waker.borrow(cs);
            match slot.take() {
                Some(old) if old.will_wake(waker) => slot.set(Some(old)),
                _ => slot.set(Some(waker.clone())),
            }
        });
    }

    /// Wake the stored task, if any, and empty the slot.
    pub fn wake(&self) {
        if let Some(waker) = critical_section::with(|cs| self.waker.borrow(cs).take()) {
            waker.wake();
        }
    }
}

impl Default for AtomicWaker {
    fn default() -> Self {
        Self::new()
    }
}
