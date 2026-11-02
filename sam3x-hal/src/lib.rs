//! Hardware abstraction layer for the Microchip SAM3X family.

#![no_std]
#![deny(missing_docs)]

pub use atsam3x8e_pac as pac;

pub mod clocks;
pub mod efc;
pub mod watchdog;
