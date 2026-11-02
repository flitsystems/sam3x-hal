#[repr(C)]
#[doc = "One of the three channels"]
#[doc(alias = "CHANNEL")]
pub struct Channel {
    ccr: Ccr,
    _reserved_1_cmr: [u8; 0x04],
    smmr: Smmr,
    _reserved3: [u8; 0x04],
    cv: Cv,
    ra: Ra,
    rb: Rb,
    rc: Rc,
    sr: Sr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
}
impl Channel {
    #[doc = "0x00 - Channel Control Register (channel = 0)"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn wave_eq_1_cmr_wave_eq_1(&self) -> &WaveEq1CmrWaveEq1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn cmr(&self) -> &Cmr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - Stepper Motor Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn smmr(&self) -> &Smmr {
        &self.smmr
    }
    #[doc = "0x10 - Counter Value (channel = 0)"]
    #[inline(always)]
    pub const fn cv(&self) -> &Cv {
        &self.cv
    }
    #[doc = "0x14 - Register A (channel = 0)"]
    #[inline(always)]
    pub const fn ra(&self) -> &Ra {
        &self.ra
    }
    #[doc = "0x18 - Register B (channel = 0)"]
    #[inline(always)]
    pub const fn rb(&self) -> &Rb {
        &self.rb
    }
    #[doc = "0x1c - Register C (channel = 0)"]
    #[inline(always)]
    pub const fn rc(&self) -> &Rc {
        &self.rc
    }
    #[doc = "0x20 - Status Register (channel = 0)"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x24 - Interrupt Enable Register (channel = 0)"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x28 - Interrupt Disable Register (channel = 0)"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x2c - Interrupt Mask Register (channel = 0)"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
}
#[doc = "CCR (w) register accessor: Channel Control Register (channel = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "Channel Control Register (channel = 0)"]
pub mod ccr;
#[doc = "CMR (rw) register accessor: Channel Mode Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`cmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr`] module"]
#[doc(alias = "CMR")]
pub type Cmr = crate::Reg<cmr::CmrSpec>;
#[doc = "Channel Mode Register (channel = 0)"]
pub mod cmr;
#[doc = "WAVE_EQ_1_CMR_WAVE_EQ_1 (rw) register accessor: Channel Mode Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`wave_eq_1_cmr_wave_eq_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wave_eq_1_cmr_wave_eq_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave_eq_1_cmr_wave_eq_1`] module"]
#[doc(alias = "WAVE_EQ_1_CMR_WAVE_EQ_1")]
pub type WaveEq1CmrWaveEq1 = crate::Reg<wave_eq_1_cmr_wave_eq_1::WaveEq1CmrWaveEq1Spec>;
#[doc = "Channel Mode Register (channel = 0)"]
pub mod wave_eq_1_cmr_wave_eq_1;
#[doc = "SMMR (rw) register accessor: Stepper Motor Mode Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`smmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smmr`] module"]
#[doc(alias = "SMMR")]
pub type Smmr = crate::Reg<smmr::SmmrSpec>;
#[doc = "Stepper Motor Mode Register (channel = 0)"]
pub mod smmr;
#[doc = "CV (r) register accessor: Counter Value (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`cv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cv`] module"]
#[doc(alias = "CV")]
pub type Cv = crate::Reg<cv::CvSpec>;
#[doc = "Counter Value (channel = 0)"]
pub mod cv;
#[doc = "RA (rw) register accessor: Register A (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`ra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ra`] module"]
#[doc(alias = "RA")]
pub type Ra = crate::Reg<ra::RaSpec>;
#[doc = "Register A (channel = 0)"]
pub mod ra;
#[doc = "RB (rw) register accessor: Register B (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`rb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rb`] module"]
#[doc(alias = "RB")]
pub type Rb = crate::Reg<rb::RbSpec>;
#[doc = "Register B (channel = 0)"]
pub mod rb;
#[doc = "RC (rw) register accessor: Register C (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`rc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc`] module"]
#[doc(alias = "RC")]
pub type Rc = crate::Reg<rc::RcSpec>;
#[doc = "Register C (channel = 0)"]
pub mod rc;
#[doc = "SR (r) register accessor: Status Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register (channel = 0)"]
pub mod sr;
#[doc = "IER (w) register accessor: Interrupt Enable Register (channel = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register (channel = 0)"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register (channel = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Interrupt Disable Register (channel = 0)"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register (channel = 0)"]
pub mod imr;
