//! Parallel Input/Output Controller (PIO)
//!
//! The SAM3X8E's I/O pins are grouped in four ports, PIOA to PIOD, each run
//! by its own controller. A pin is either driven by software, as an input or
//! an output, or handed over to one of the two on-chip peripherals it is
//! wired to, called A and B, such as a UART's transmit line.
//!
//! [`Pins::new`] takes the four controllers and returns one value per pin,
//! named after it: `pa8`, `pb27`, and so on. A pin's mode is part of its
//! type, and each `into_*` method reconfigures the pin and returns it with
//! its new type. A peripheral driver asks for its pins in the mode it needs,
//! so passing the wrong pin, or a pin in the wrong mode, does not compile.
//! [`DynPin`] carries the pin and its mode as values instead, for an array
//! of pins from different ports or a pin chosen at run time.
//!
//! The pins implement the embedded-hal digital traits. An input can be
//! pulled up but not down: the controller has no pull-down. It can raise an
//! interrupt on an edge or a level, with an optional glitch or debouncing
//! filter. A port's interrupt flags are cleared when read, so a handler
//! reads them once with [`Port::interrupt_status`] and checks every pin it
//! cares about in that value.
//!
//! Pins of the same port can be used from different contexts, such as the
//! main loop and an interrupt handler, without a lock: each operation
//! touches only its own pin.
//!
//! See chapter 31 of the SAM3X/SAM3A datasheet for more details.

use core::convert::Infallible;
use core::marker::PhantomData;

use embedded_hal::digital::{
    ErrorKind, ErrorType, InputPin, OutputPin, PinState, StatefulOutputPin,
};

use crate::pac;
use crate::pac::pioa::RegisterBlock;

/// Write a mask to a set/clear register.
macro_rules! wo {
    ($reg:expr, $mask:expr) => {
        // SAFETY: a set/clear register, one bit per pin.
        unsafe {
            $reg.write_with_zero(|w| w.bits($mask));
        }
    };
}

/// One of the four controllers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port {
    /// PIOA, peripheral 11.
    A,
    /// PIOB, peripheral 12.
    B,
    /// PIOC, peripheral 13.
    C,
    /// PIOD, peripheral 14.
    D,
}

impl Port {
    fn regs(self) -> &'static RegisterBlock {
        // One register block type for the four controllers: the PAC's SVD
        // patch derives PIOB, PIOC and PIOD from PIOA, so every `ptr()` here
        // is already a `*const pioa::RegisterBlock`.
        let ptr = match self {
            Port::A => pac::Pioa::ptr(),
            Port::B => pac::Piob::ptr(),
            Port::C => pac::Pioc::ptr(),
            Port::D => pac::Piod::ptr(),
        };
        // SAFETY: a fixed MMIO address, mapped for the whole run.
        unsafe { &*ptr }
    }

    /// The controller's NVIC interrupt.
    pub const fn interrupt(self) -> pac::Interrupt {
        match self {
            Port::A => pac::Interrupt::PIOA,
            Port::B => pac::Interrupt::PIOB,
            Port::C => pac::Interrupt::PIOC,
            Port::D => pac::Interrupt::PIOD,
        }
    }

    /// Read the interrupt status of every pin of the controller, as a mask.
    /// Reading clears it, so read it once per handler run.
    pub fn interrupt_status(self) -> u32 {
        self.regs().isr().read().bits()
    }

    /// The level of every pin of the controller, as a mask.
    pub fn levels(self) -> u32 {
        self.regs().pdsr().read().bits()
    }

    /// Period of the debouncing filter, shared by every pin of the
    /// controller that enables it: `2 * (div + 1)` slow-clock cycles, 32 kHz
    /// on the Due, so `div` of 0x3FFF is about one second.
    pub fn set_debounce_divider(self, div: u16) {
        self.regs()
            .scdr()
            .write(|w| unsafe { w.div().bits(div & 0x3FFF) });
    }
}

mod sealed {
    pub trait Sealed {}
}

/// A pin's configuration, carried in its type.
pub trait PinMode: sealed::Sealed {}

/// The pin's state at reset, before configuration.
pub enum Reset {}
impl sealed::Sealed for Reset {}
impl PinMode for Reset {}

/// Input, with the pull-up on or off.
pub struct Input<PULL: Pull> {
    _pull: PhantomData<PULL>,
}
impl<PULL: Pull> sealed::Sealed for Input<PULL> {}
impl<PULL: Pull> PinMode for Input<PULL> {}

/// An input's pull resistor.
pub trait Pull: sealed::Sealed {}
/// No pull resistor.
pub enum Floating {}
impl sealed::Sealed for Floating {}
impl Pull for Floating {}
/// The internal pull-up.
pub enum PullUp {}
impl sealed::Sealed for PullUp {}
impl Pull for PullUp {}

/// Output, push-pull or open-drain.
pub struct Output<KIND: OutputKind> {
    _kind: PhantomData<KIND>,
}
impl<KIND: OutputKind> sealed::Sealed for Output<KIND> {}
impl<KIND: OutputKind> PinMode for Output<KIND> {}

/// How an output drives its line.
pub trait OutputKind: sealed::Sealed {
    /// The same mode, as a value, for [`DynPin`].
    const DYN: DynMode;
}
/// Drives both high and low.
pub enum PushPull {}
impl sealed::Sealed for PushPull {}
impl OutputKind for PushPull {
    const DYN: DynMode = DynMode::PushPullOutput;
}
/// Drives low only, released when high (multi-drive).
pub enum OpenDrain {}
impl sealed::Sealed for OpenDrain {}
impl OutputKind for OpenDrain {
    const DYN: DynMode = DynMode::OpenDrainOutput;
}

/// Assigned to a peripheral: function A or B of the pin's multiplexer.
pub struct Peripheral<F: Function> {
    _function: PhantomData<F>,
}
impl<F: Function> sealed::Sealed for Peripheral<F> {}
impl<F: Function> PinMode for Peripheral<F> {}

/// One of the two peripheral functions of a pin.
pub trait Function: sealed::Sealed {
    /// The `ABSR` bit: clear for A, set for B.
    const B: bool;
}
/// Peripheral function A.
pub enum A {}
impl sealed::Sealed for A {}
impl Function for A {
    const B: bool = false;
}
/// Peripheral function B.
pub enum B {}
impl sealed::Sealed for B {}
impl Function for B {
    const B: bool = true;
}

/// A pin's identity: its controller and bit.
pub trait PinId: sealed::Sealed {
    /// The controller.
    const PORT: Port;
    /// The bit, 0 to 31.
    const NUM: u8;
}

/// A pin, owned, in mode `M`.
pub struct Pin<I: PinId, M: PinMode> {
    _id: PhantomData<I>,
    _mode: PhantomData<M>,
}

impl<I: PinId, M: PinMode> Pin<I, M> {
    const MASK: u32 = 1 << I::NUM;

    fn regs() -> &'static RegisterBlock {
        I::PORT.regs()
    }

    fn change<N: PinMode>(self) -> Pin<I, N> {
        Pin {
            _id: PhantomData,
            _mode: PhantomData,
        }
    }

    /// The controller and the bit.
    pub const fn id(&self) -> (Port, u8) {
        (I::PORT, I::NUM)
    }

    /// The pin's bit in the controller's registers.
    pub const fn mask(&self) -> u32 {
        Self::MASK
    }

    /// An input with no pull.
    pub fn into_floating_input(self) -> Pin<I, Input<Floating>> {
        let r = Self::regs();
        wo!(r.pudr(), Self::MASK);
        wo!(r.odr(), Self::MASK);
        wo!(r.per(), Self::MASK);
        self.change()
    }

    /// An input with the internal pull-up.
    pub fn into_pull_up_input(self) -> Pin<I, Input<PullUp>> {
        let r = Self::regs();
        wo!(r.puer(), Self::MASK);
        wo!(r.odr(), Self::MASK);
        wo!(r.per(), Self::MASK);
        self.change()
    }

    /// A push-pull output, starting low.
    pub fn into_push_pull_output(self) -> Pin<I, Output<PushPull>> {
        self.into_push_pull_output_in_state(PinState::Low)
    }

    /// A push-pull output starting at `state`. The level is written before
    /// the output is enabled, so the pin does not glitch.
    pub fn into_push_pull_output_in_state(self, state: PinState) -> Pin<I, Output<PushPull>> {
        let r = Self::regs();
        wo!(r.mddr(), Self::MASK);
        wo!(r.pudr(), Self::MASK);
        match state {
            PinState::High => wo!(r.sodr(), Self::MASK),
            PinState::Low => wo!(r.codr(), Self::MASK),
        }
        wo!(r.oer(), Self::MASK);
        wo!(r.per(), Self::MASK);
        self.change()
    }

    /// An open-drain output, released, with the internal pull-up enabled.
    pub fn into_open_drain_output(self) -> Pin<I, Output<OpenDrain>> {
        let r = Self::regs();
        wo!(r.mder(), Self::MASK);
        wo!(r.puer(), Self::MASK);
        wo!(r.sodr(), Self::MASK);
        wo!(r.oer(), Self::MASK);
        wo!(r.per(), Self::MASK);
        self.change()
    }

    /// Assign the pin to peripheral function A.
    pub fn into_peripheral_a(self) -> Pin<I, Peripheral<A>> {
        self.into_peripheral()
    }

    /// Assign the pin to peripheral function B.
    pub fn into_peripheral_b(self) -> Pin<I, Peripheral<B>> {
        self.into_peripheral()
    }

    fn into_peripheral<F: Function>(self) -> Pin<I, Peripheral<F>> {
        let r = Self::regs();
        // Select the function before handing the pin to the peripheral.
        critical_section::with(|_| {
            r.absr().modify(|rd, w| {
                let bits = if F::B {
                    rd.bits() | Self::MASK
                } else {
                    rd.bits() & !Self::MASK
                };
                // SAFETY: every bit of ABSR is one pin's A/B select; the
                // other pins' bits are written back unchanged.
                unsafe { w.bits(bits) }
            });
        });
        wo!(r.pdr(), Self::MASK);
        self.change()
    }
}

/// What triggers an input's interrupt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InterruptMode {
    /// Either edge, the controller's default.
    AnyEdge,
    /// Low to high.
    RisingEdge,
    /// High to low.
    FallingEdge,
    /// While high.
    HighLevel,
    /// While low.
    LowLevel,
}

impl<I: PinId, PULL: Pull> Pin<I, Input<PULL>> {
    /// The pin is high.
    pub fn is_high(&self) -> bool {
        Self::regs().pdsr().read().bits() & Self::MASK != 0
    }

    /// The pin is low.
    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    /// Enable the pin's interrupt on `mode`. The caller unmasks the NVIC
    /// line ([`Port::interrupt`]), and the handler reads
    /// [`Port::interrupt_status`] to find which pins fired.
    ///
    /// The status flag is set on any change, even with the interrupt
    /// disabled, so a change before this call can fire once right after it.
    /// The status is not cleared here: it is shared by the whole controller,
    /// and reading it would clear the other pins' pending events.
    pub fn enable_interrupt(&mut self, mode: InterruptMode) {
        let r = Self::regs();
        match mode {
            InterruptMode::AnyEdge => wo!(r.aimdr(), Self::MASK),
            InterruptMode::RisingEdge => {
                wo!(r.esr(), Self::MASK);
                wo!(r.rehlsr(), Self::MASK);
                wo!(r.aimer(), Self::MASK);
            }
            InterruptMode::FallingEdge => {
                wo!(r.esr(), Self::MASK);
                wo!(r.fellsr(), Self::MASK);
                wo!(r.aimer(), Self::MASK);
            }
            InterruptMode::HighLevel => {
                wo!(r.lsr(), Self::MASK);
                wo!(r.rehlsr(), Self::MASK);
                wo!(r.aimer(), Self::MASK);
            }
            InterruptMode::LowLevel => {
                wo!(r.lsr(), Self::MASK);
                wo!(r.fellsr(), Self::MASK);
                wo!(r.aimer(), Self::MASK);
            }
        }
        wo!(r.ier(), Self::MASK);
    }

    /// Disable the pin's interrupt.
    pub fn disable_interrupt(&mut self) {
        wo!(Self::regs().idr(), Self::MASK);
    }

    /// Enable the glitch filter: ignore pulses shorter than half a master
    /// clock cycle.
    pub fn enable_glitch_filter(&mut self) {
        let r = Self::regs();
        wo!(r.scifsr(), Self::MASK);
        wo!(r.ifer(), Self::MASK);
    }

    /// Enable the debounce filter: ignore pulses shorter than the
    /// controller's debounce period ([`Port::set_debounce_divider`]).
    pub fn enable_debounce_filter(&mut self) {
        let r = Self::regs();
        wo!(r.difsr(), Self::MASK);
        wo!(r.ifer(), Self::MASK);
    }

    /// Disable the input filter.
    pub fn disable_input_filter(&mut self) {
        wo!(Self::regs().ifdr(), Self::MASK);
    }

    /// Convert to a [`DynPin`], whose mode is checked at run time.
    pub fn downgrade(self) -> DynPin {
        DynPin {
            port: I::PORT,
            num: I::NUM,
            mode: DynMode::Input,
        }
    }
}

impl<I: PinId, KIND: OutputKind> Pin<I, Output<KIND>> {
    /// Drive high (or release, for open drain).
    pub fn set_high(&mut self) {
        wo!(Self::regs().sodr(), Self::MASK);
    }

    /// Drive low.
    pub fn set_low(&mut self) {
        wo!(Self::regs().codr(), Self::MASK);
    }

    /// The level last written. For open drain, it may differ from the pin
    /// level.
    pub fn is_set_high(&self) -> bool {
        Self::regs().odsr().read().bits() & Self::MASK != 0
    }

    /// Invert the level last written.
    pub fn toggle(&mut self) {
        if Self::is_set_high(self) {
            self.set_low();
        } else {
            self.set_high();
        }
    }

    /// Convert to a [`DynPin`], whose mode is checked at run time.
    pub fn downgrade(self) -> DynPin {
        DynPin {
            port: I::PORT,
            num: I::NUM,
            mode: KIND::DYN,
        }
    }
}

impl<I: PinId> Pin<I, Output<OpenDrain>> {
    /// The pin level, which another device may hold low.
    pub fn is_high(&self) -> bool {
        Self::regs().pdsr().read().bits() & Self::MASK != 0
    }

    /// The pin is low.
    pub fn is_low(&self) -> bool {
        !self.is_high()
    }
}

impl<I: PinId, M: PinMode> ErrorType for Pin<I, M> {
    type Error = Infallible;
}

impl<I: PinId, KIND: OutputKind> OutputPin for Pin<I, Output<KIND>> {
    fn set_low(&mut self) -> Result<(), Infallible> {
        Pin::set_low(self);
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Infallible> {
        Pin::set_high(self);
        Ok(())
    }
}

impl<I: PinId, KIND: OutputKind> StatefulOutputPin for Pin<I, Output<KIND>> {
    fn is_set_high(&mut self) -> Result<bool, Infallible> {
        Ok(Pin::is_set_high(self))
    }

    fn is_set_low(&mut self) -> Result<bool, Infallible> {
        Ok(!Pin::is_set_high(self))
    }
}

impl<I: PinId, PULL: Pull> InputPin for Pin<I, Input<PULL>> {
    fn is_high(&mut self) -> Result<bool, Infallible> {
        Ok(Self::is_high(self))
    }

    fn is_low(&mut self) -> Result<bool, Infallible> {
        Ok(Self::is_low(self))
    }
}

impl<I: PinId> InputPin for Pin<I, Output<OpenDrain>> {
    fn is_high(&mut self) -> Result<bool, Infallible> {
        Ok(Self::is_high(self))
    }

    fn is_low(&mut self) -> Result<bool, Infallible> {
        Ok(Self::is_low(self))
    }
}

/// A [`DynPin`]'s mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DynMode {
    /// Input, whatever the pull.
    Input,
    /// Push-pull output.
    PushPullOutput,
    /// Open-drain output.
    OpenDrainOutput,
}

/// A pin whose identity and mode are run-time values rather than types, for
/// arrays of pins or pins chosen at run time.
///
/// Created from a typed pin with `downgrade`. The mode is checked on every
/// write; writing to an input returns [`InvalidMode`].
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DynPin {
    port: Port,
    num: u8,
    mode: DynMode,
}

/// The operation does not apply to the pin's current mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidMode;

impl embedded_hal::digital::Error for InvalidMode {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

impl DynPin {
    fn regs(&self) -> &'static RegisterBlock {
        self.port.regs()
    }

    fn mask(&self) -> u32 {
        1 << self.num
    }

    /// The controller and the bit.
    pub fn id(&self) -> (Port, u8) {
        (self.port, self.num)
    }

    /// The current mode.
    pub fn mode(&self) -> DynMode {
        self.mode
    }

    fn check_output(&self) -> Result<(), InvalidMode> {
        match self.mode {
            DynMode::Input => Err(InvalidMode),
            DynMode::PushPullOutput | DynMode::OpenDrainOutput => Ok(()),
        }
    }

    /// Drive high (or release, for open drain).
    pub fn set_high(&mut self) -> Result<(), InvalidMode> {
        self.check_output()?;
        wo!(self.regs().sodr(), self.mask());
        Ok(())
    }

    /// Drive low.
    pub fn set_low(&mut self) -> Result<(), InvalidMode> {
        self.check_output()?;
        wo!(self.regs().codr(), self.mask());
        Ok(())
    }

    /// The level last written.
    pub fn is_set_high(&self) -> Result<bool, InvalidMode> {
        self.check_output()?;
        Ok(self.regs().odsr().read().bits() & self.mask() != 0)
    }

    /// The pin level, readable in every mode.
    pub fn is_high(&self) -> bool {
        self.regs().pdsr().read().bits() & self.mask() != 0
    }
}

impl ErrorType for DynPin {
    type Error = InvalidMode;
}

impl OutputPin for DynPin {
    fn set_low(&mut self) -> Result<(), InvalidMode> {
        DynPin::set_low(self)
    }

    fn set_high(&mut self) -> Result<(), InvalidMode> {
        DynPin::set_high(self)
    }
}

impl StatefulOutputPin for DynPin {
    fn is_set_high(&mut self) -> Result<bool, InvalidMode> {
        DynPin::is_set_high(self)
    }

    fn is_set_low(&mut self) -> Result<bool, InvalidMode> {
        DynPin::is_set_high(self).map(|high| !high)
    }
}

impl InputPin for DynPin {
    fn is_high(&mut self) -> Result<bool, InvalidMode> {
        Ok(DynPin::is_high(self))
    }

    fn is_low(&mut self) -> Result<bool, InvalidMode> {
        Ok(!DynPin::is_high(self))
    }
}

/// A placeholder for a pin the hardware does not have.
///
/// Pass it to a driver that needs an `OutputPin`, such as a reset or a chip
/// select, when the board has none. Writes do nothing.
pub struct NoPin;

impl ErrorType for NoPin {
    type Error = Infallible;
}

impl OutputPin for NoPin {
    fn set_low(&mut self) -> Result<(), Infallible> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Infallible> {
        Ok(())
    }
}

macro_rules! pins {
    ($( $port:expr => [ $( ($Id:ident, $field:ident, $n:literal) ),* $(,)? ] ),* $(,)?) => {
        $( $(
            #[doc = concat!("Pin ", stringify!($Id), ".")]
            pub enum $Id {}
            impl sealed::Sealed for $Id {}
            impl PinId for $Id {
                const PORT: Port = $port;
                const NUM: u8 = $n;
            }
        )* )*

        /// Every pin of the SAM3X8E package, each in its reset state.
        pub struct Pins {
            $( $(
                #[doc = concat!("Pin ", stringify!($Id), ".")]
                pub $field: Pin<$Id, Reset>,
            )* )*
        }

        impl Pins {
            /// Enable the clocks of the four controllers, needed to read
            /// inputs, and split them into pins.
            pub fn new(
                _pioa: pac::Pioa,
                _piob: pac::Piob,
                _pioc: pac::Pioc,
                _piod: pac::Piod,
                pmc: &pac::Pmc,
            ) -> Self {
                // SAFETY: PCER0 is a write-only enable register; setting the
                // four PIO bits touches nothing else.
                unsafe {
                    pmc.pmc_pcer0().write_with_zero(|w| {
                        w.pid11().set_bit();
                        w.pid12().set_bit();
                        w.pid13().set_bit();
                        w.pid14().set_bit()
                    });
                }
                Pins {
                    $( $(
                        $field: Pin { _id: PhantomData, _mode: PhantomData },
                    )* )*
                }
            }
        }
    };
}

pins! {
    Port::A => [
        (PA0, pa0, 0), (PA1, pa1, 1), (PA2, pa2, 2), (PA3, pa3, 3),
        (PA4, pa4, 4), (PA5, pa5, 5), (PA6, pa6, 6), (PA7, pa7, 7),
        (PA8, pa8, 8), (PA9, pa9, 9), (PA10, pa10, 10), (PA11, pa11, 11),
        (PA12, pa12, 12), (PA13, pa13, 13), (PA14, pa14, 14), (PA15, pa15, 15),
        (PA16, pa16, 16), (PA17, pa17, 17), (PA18, pa18, 18), (PA19, pa19, 19),
        (PA20, pa20, 20), (PA21, pa21, 21), (PA22, pa22, 22), (PA23, pa23, 23),
        (PA24, pa24, 24), (PA25, pa25, 25), (PA26, pa26, 26), (PA27, pa27, 27),
        (PA28, pa28, 28), (PA29, pa29, 29),
    ],
    Port::B => [
        (PB0, pb0, 0), (PB1, pb1, 1), (PB2, pb2, 2), (PB3, pb3, 3),
        (PB4, pb4, 4), (PB5, pb5, 5), (PB6, pb6, 6), (PB7, pb7, 7),
        (PB8, pb8, 8), (PB9, pb9, 9), (PB10, pb10, 10), (PB11, pb11, 11),
        (PB12, pb12, 12), (PB13, pb13, 13), (PB14, pb14, 14), (PB15, pb15, 15),
        (PB16, pb16, 16), (PB17, pb17, 17), (PB18, pb18, 18), (PB19, pb19, 19),
        (PB20, pb20, 20), (PB21, pb21, 21), (PB22, pb22, 22), (PB23, pb23, 23),
        (PB24, pb24, 24), (PB25, pb25, 25), (PB26, pb26, 26), (PB27, pb27, 27),
        (PB28, pb28, 28), (PB29, pb29, 29), (PB30, pb30, 30), (PB31, pb31, 31),
    ],
    Port::C => [
        (PC0, pc0, 0), (PC1, pc1, 1), (PC2, pc2, 2), (PC3, pc3, 3),
        (PC4, pc4, 4), (PC5, pc5, 5), (PC6, pc6, 6), (PC7, pc7, 7),
        (PC8, pc8, 8), (PC9, pc9, 9), (PC10, pc10, 10), (PC11, pc11, 11),
        (PC12, pc12, 12), (PC13, pc13, 13), (PC14, pc14, 14), (PC15, pc15, 15),
        (PC16, pc16, 16), (PC17, pc17, 17), (PC18, pc18, 18), (PC19, pc19, 19),
        (PC20, pc20, 20), (PC21, pc21, 21), (PC22, pc22, 22), (PC23, pc23, 23),
        (PC24, pc24, 24), (PC25, pc25, 25), (PC26, pc26, 26), (PC27, pc27, 27),
        (PC28, pc28, 28), (PC29, pc29, 29), (PC30, pc30, 30),
    ],
    Port::D => [
        (PD0, pd0, 0), (PD1, pd1, 1), (PD2, pd2, 2), (PD3, pd3, 3),
        (PD4, pd4, 4), (PD5, pd5, 5), (PD6, pd6, 6), (PD7, pd7, 7),
        (PD8, pd8, 8), (PD9, pd9, 9), (PD10, pd10, 10),
    ],
}
