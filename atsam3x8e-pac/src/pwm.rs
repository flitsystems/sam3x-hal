#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk: Clk,
    ena: Ena,
    dis: Dis,
    sr: Sr,
    ier1: Ier1,
    idr1: Idr1,
    imr1: Imr1,
    isr1: Isr1,
    scm: Scm,
    _reserved9: [u8; 0x04],
    scuc: Scuc,
    scup: Scup,
    scupupd: Scupupd,
    ier2: Ier2,
    idr2: Idr2,
    imr2: Imr2,
    isr2: Isr2,
    oov: Oov,
    os: Os,
    oss: Oss,
    osc: Osc,
    ossupd: Ossupd,
    oscupd: Oscupd,
    fmr: Fmr,
    fsr: Fsr,
    fcr: Fcr,
    fpv: Fpv,
    fpe1: Fpe1,
    fpe2: Fpe2,
    _reserved28: [u8; 0x08],
    elmr: [Elmr; 2],
    _reserved29: [u8; 0x2c],
    smmr: Smmr,
    _reserved30: [u8; 0x30],
    wpcr: Wpcr,
    wpsr: Wpsr,
    _reserved32: [u8; 0x1c],
    tpr: Tpr,
    tcr: Tcr,
    _reserved34: [u8; 0x08],
    tnpr: Tnpr,
    tncr: Tncr,
    ptcr: Ptcr,
    ptsr: Ptsr,
    _reserved38: [u8; 0x08],
    compare: [Compare; 8],
    _reserved39: [u8; 0x50],
    channel: [Channel; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - PWM Clock Register"]
    #[inline(always)]
    pub const fn clk(&self) -> &Clk {
        &self.clk
    }
    #[doc = "0x04 - PWM Enable Register"]
    #[inline(always)]
    pub const fn ena(&self) -> &Ena {
        &self.ena
    }
    #[doc = "0x08 - PWM Disable Register"]
    #[inline(always)]
    pub const fn dis(&self) -> &Dis {
        &self.dis
    }
    #[doc = "0x0c - PWM Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - PWM Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn ier1(&self) -> &Ier1 {
        &self.ier1
    }
    #[doc = "0x14 - PWM Interrupt Disable Register 1"]
    #[inline(always)]
    pub const fn idr1(&self) -> &Idr1 {
        &self.idr1
    }
    #[doc = "0x18 - PWM Interrupt Mask Register 1"]
    #[inline(always)]
    pub const fn imr1(&self) -> &Imr1 {
        &self.imr1
    }
    #[doc = "0x1c - PWM Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn isr1(&self) -> &Isr1 {
        &self.isr1
    }
    #[doc = "0x20 - PWM Sync Channels Mode Register"]
    #[inline(always)]
    pub const fn scm(&self) -> &Scm {
        &self.scm
    }
    #[doc = "0x28 - PWM Sync Channels Update Control Register"]
    #[inline(always)]
    pub const fn scuc(&self) -> &Scuc {
        &self.scuc
    }
    #[doc = "0x2c - PWM Sync Channels Update Period Register"]
    #[inline(always)]
    pub const fn scup(&self) -> &Scup {
        &self.scup
    }
    #[doc = "0x30 - PWM Sync Channels Update Period Update Register"]
    #[inline(always)]
    pub const fn scupupd(&self) -> &Scupupd {
        &self.scupupd
    }
    #[doc = "0x34 - PWM Interrupt Enable Register 2"]
    #[inline(always)]
    pub const fn ier2(&self) -> &Ier2 {
        &self.ier2
    }
    #[doc = "0x38 - PWM Interrupt Disable Register 2"]
    #[inline(always)]
    pub const fn idr2(&self) -> &Idr2 {
        &self.idr2
    }
    #[doc = "0x3c - PWM Interrupt Mask Register 2"]
    #[inline(always)]
    pub const fn imr2(&self) -> &Imr2 {
        &self.imr2
    }
    #[doc = "0x40 - PWM Interrupt Status Register 2"]
    #[inline(always)]
    pub const fn isr2(&self) -> &Isr2 {
        &self.isr2
    }
    #[doc = "0x44 - PWM Output Override Value Register"]
    #[inline(always)]
    pub const fn oov(&self) -> &Oov {
        &self.oov
    }
    #[doc = "0x48 - PWM Output Selection Register"]
    #[inline(always)]
    pub const fn os(&self) -> &Os {
        &self.os
    }
    #[doc = "0x4c - PWM Output Selection Set Register"]
    #[inline(always)]
    pub const fn oss(&self) -> &Oss {
        &self.oss
    }
    #[doc = "0x50 - PWM Output Selection Clear Register"]
    #[inline(always)]
    pub const fn osc(&self) -> &Osc {
        &self.osc
    }
    #[doc = "0x54 - PWM Output Selection Set Update Register"]
    #[inline(always)]
    pub const fn ossupd(&self) -> &Ossupd {
        &self.ossupd
    }
    #[doc = "0x58 - PWM Output Selection Clear Update Register"]
    #[inline(always)]
    pub const fn oscupd(&self) -> &Oscupd {
        &self.oscupd
    }
    #[doc = "0x5c - PWM Fault Mode Register"]
    #[inline(always)]
    pub const fn fmr(&self) -> &Fmr {
        &self.fmr
    }
    #[doc = "0x60 - PWM Fault Status Register"]
    #[inline(always)]
    pub const fn fsr(&self) -> &Fsr {
        &self.fsr
    }
    #[doc = "0x64 - PWM Fault Clear Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        &self.fcr
    }
    #[doc = "0x68 - PWM Fault Protection Value Register"]
    #[inline(always)]
    pub const fn fpv(&self) -> &Fpv {
        &self.fpv
    }
    #[doc = "0x6c - PWM Fault Protection Enable Register 1"]
    #[inline(always)]
    pub const fn fpe1(&self) -> &Fpe1 {
        &self.fpe1
    }
    #[doc = "0x70 - PWM Fault Protection Enable Register 2"]
    #[inline(always)]
    pub const fn fpe2(&self) -> &Fpe2 {
        &self.fpe2
    }
    #[doc = "0x7c..0x84 - PWM Event Line 0 Mode Register"]
    #[inline(always)]
    pub const fn elmr(&self, n: usize) -> &Elmr {
        &self.elmr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x7c..0x84 - PWM Event Line 0 Mode Register"]
    #[inline(always)]
    pub fn elmr_iter(&self) -> impl Iterator<Item = &Elmr> {
        self.elmr.iter()
    }
    #[doc = "0xb0 - PWM Stepper Motor Mode Register"]
    #[inline(always)]
    pub const fn smmr(&self) -> &Smmr {
        &self.smmr
    }
    #[doc = "0xe4 - PWM Write Protect Control Register"]
    #[inline(always)]
    pub const fn wpcr(&self) -> &Wpcr {
        &self.wpcr
    }
    #[doc = "0xe8 - PWM Write Protect Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
    #[doc = "0x108 - Transmit Pointer Register"]
    #[inline(always)]
    pub const fn tpr(&self) -> &Tpr {
        &self.tpr
    }
    #[doc = "0x10c - Transmit Counter Register"]
    #[inline(always)]
    pub const fn tcr(&self) -> &Tcr {
        &self.tcr
    }
    #[doc = "0x118 - Transmit Next Pointer Register"]
    #[inline(always)]
    pub const fn tnpr(&self) -> &Tnpr {
        &self.tnpr
    }
    #[doc = "0x11c - Transmit Next Counter Register"]
    #[inline(always)]
    pub const fn tncr(&self) -> &Tncr {
        &self.tncr
    }
    #[doc = "0x120 - Transfer Control Register"]
    #[inline(always)]
    pub const fn ptcr(&self) -> &Ptcr {
        &self.ptcr
    }
    #[doc = "0x124 - Transfer Status Register"]
    #[inline(always)]
    pub const fn ptsr(&self) -> &Ptsr {
        &self.ptsr
    }
    #[doc = "0x130..0x1b0 - One of the eight comparison units"]
    #[inline(always)]
    pub const fn compare(&self, n: usize) -> &Compare {
        &self.compare[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x130..0x1b0 - One of the eight comparison units"]
    #[inline(always)]
    pub fn compare_iter(&self) -> impl Iterator<Item = &Compare> {
        self.compare.iter()
    }
    #[doc = "0x200..0x300 - One of the eight channels"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &Channel {
        &self.channel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x300 - One of the eight channels"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &Channel> {
        self.channel.iter()
    }
}
#[doc = "CLK (rw) register accessor: PWM Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`] module"]
#[doc(alias = "CLK")]
pub type Clk = crate::Reg<clk::ClkSpec>;
#[doc = "PWM Clock Register"]
pub mod clk;
#[doc = "ENA (w) register accessor: PWM Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ena::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ena`] module"]
#[doc(alias = "ENA")]
pub type Ena = crate::Reg<ena::EnaSpec>;
#[doc = "PWM Enable Register"]
pub mod ena;
#[doc = "DIS (w) register accessor: PWM Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dis`] module"]
#[doc(alias = "DIS")]
pub type Dis = crate::Reg<dis::DisSpec>;
#[doc = "PWM Disable Register"]
pub mod dis;
#[doc = "SR (r) register accessor: PWM Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "PWM Status Register"]
pub mod sr;
#[doc = "IER1 (w) register accessor: PWM Interrupt Enable Register 1\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier1`] module"]
#[doc(alias = "IER1")]
pub type Ier1 = crate::Reg<ier1::Ier1Spec>;
#[doc = "PWM Interrupt Enable Register 1"]
pub mod ier1;
#[doc = "IDR1 (w) register accessor: PWM Interrupt Disable Register 1\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr1`] module"]
#[doc(alias = "IDR1")]
pub type Idr1 = crate::Reg<idr1::Idr1Spec>;
#[doc = "PWM Interrupt Disable Register 1"]
pub mod idr1;
#[doc = "IMR1 (r) register accessor: PWM Interrupt Mask Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`imr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr1`] module"]
#[doc(alias = "IMR1")]
pub type Imr1 = crate::Reg<imr1::Imr1Spec>;
#[doc = "PWM Interrupt Mask Register 1"]
pub mod imr1;
#[doc = "ISR1 (r) register accessor: PWM Interrupt Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`isr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr1`] module"]
#[doc(alias = "ISR1")]
pub type Isr1 = crate::Reg<isr1::Isr1Spec>;
#[doc = "PWM Interrupt Status Register 1"]
pub mod isr1;
#[doc = "SCM (rw) register accessor: PWM Sync Channels Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scm`] module"]
#[doc(alias = "SCM")]
pub type Scm = crate::Reg<scm::ScmSpec>;
#[doc = "PWM Sync Channels Mode Register"]
pub mod scm;
#[doc = "SCUC (rw) register accessor: PWM Sync Channels Update Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc`] module"]
#[doc(alias = "SCUC")]
pub type Scuc = crate::Reg<scuc::ScucSpec>;
#[doc = "PWM Sync Channels Update Control Register"]
pub mod scuc;
#[doc = "SCUP (rw) register accessor: PWM Sync Channels Update Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scup`] module"]
#[doc(alias = "SCUP")]
pub type Scup = crate::Reg<scup::ScupSpec>;
#[doc = "PWM Sync Channels Update Period Register"]
pub mod scup;
#[doc = "SCUPUPD (w) register accessor: PWM Sync Channels Update Period Update Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scupupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scupupd`] module"]
#[doc(alias = "SCUPUPD")]
pub type Scupupd = crate::Reg<scupupd::ScupupdSpec>;
#[doc = "PWM Sync Channels Update Period Update Register"]
pub mod scupupd;
#[doc = "IER2 (w) register accessor: PWM Interrupt Enable Register 2\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier2`] module"]
#[doc(alias = "IER2")]
pub type Ier2 = crate::Reg<ier2::Ier2Spec>;
#[doc = "PWM Interrupt Enable Register 2"]
pub mod ier2;
#[doc = "IDR2 (w) register accessor: PWM Interrupt Disable Register 2\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr2`] module"]
#[doc(alias = "IDR2")]
pub type Idr2 = crate::Reg<idr2::Idr2Spec>;
#[doc = "PWM Interrupt Disable Register 2"]
pub mod idr2;
#[doc = "IMR2 (r) register accessor: PWM Interrupt Mask Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`imr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr2`] module"]
#[doc(alias = "IMR2")]
pub type Imr2 = crate::Reg<imr2::Imr2Spec>;
#[doc = "PWM Interrupt Mask Register 2"]
pub mod imr2;
#[doc = "ISR2 (r) register accessor: PWM Interrupt Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`isr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr2`] module"]
#[doc(alias = "ISR2")]
pub type Isr2 = crate::Reg<isr2::Isr2Spec>;
#[doc = "PWM Interrupt Status Register 2"]
pub mod isr2;
#[doc = "OOV (rw) register accessor: PWM Output Override Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`oov::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oov::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oov`] module"]
#[doc(alias = "OOV")]
pub type Oov = crate::Reg<oov::OovSpec>;
#[doc = "PWM Output Override Value Register"]
pub mod oov;
#[doc = "OS (rw) register accessor: PWM Output Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`os::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`os::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@os`] module"]
#[doc(alias = "OS")]
pub type Os = crate::Reg<os::OsSpec>;
#[doc = "PWM Output Selection Register"]
pub mod os;
#[doc = "OSS (w) register accessor: PWM Output Selection Set Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oss::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oss`] module"]
#[doc(alias = "OSS")]
pub type Oss = crate::Reg<oss::OssSpec>;
#[doc = "PWM Output Selection Set Register"]
pub mod oss;
#[doc = "OSC (w) register accessor: PWM Output Selection Clear Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc`] module"]
#[doc(alias = "OSC")]
pub type Osc = crate::Reg<osc::OscSpec>;
#[doc = "PWM Output Selection Clear Register"]
pub mod osc;
#[doc = "OSSUPD (w) register accessor: PWM Output Selection Set Update Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ossupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ossupd`] module"]
#[doc(alias = "OSSUPD")]
pub type Ossupd = crate::Reg<ossupd::OssupdSpec>;
#[doc = "PWM Output Selection Set Update Register"]
pub mod ossupd;
#[doc = "OSCUPD (w) register accessor: PWM Output Selection Clear Update Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscupd`] module"]
#[doc(alias = "OSCUPD")]
pub type Oscupd = crate::Reg<oscupd::OscupdSpec>;
#[doc = "PWM Output Selection Clear Update Register"]
pub mod oscupd;
#[doc = "FMR (rw) register accessor: PWM Fault Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmr`] module"]
#[doc(alias = "FMR")]
pub type Fmr = crate::Reg<fmr::FmrSpec>;
#[doc = "PWM Fault Mode Register"]
pub mod fmr;
#[doc = "FSR (r) register accessor: PWM Fault Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsr`] module"]
#[doc(alias = "FSR")]
pub type Fsr = crate::Reg<fsr::FsrSpec>;
#[doc = "PWM Fault Status Register"]
pub mod fsr;
#[doc = "FCR (w) register accessor: PWM Fault Clear Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`] module"]
#[doc(alias = "FCR")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "PWM Fault Clear Register"]
pub mod fcr;
#[doc = "FPV (rw) register accessor: PWM Fault Protection Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpv`] module"]
#[doc(alias = "FPV")]
pub type Fpv = crate::Reg<fpv::FpvSpec>;
#[doc = "PWM Fault Protection Value Register"]
pub mod fpv;
#[doc = "FPE1 (rw) register accessor: PWM Fault Protection Enable Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpe1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpe1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpe1`] module"]
#[doc(alias = "FPE1")]
pub type Fpe1 = crate::Reg<fpe1::Fpe1Spec>;
#[doc = "PWM Fault Protection Enable Register 1"]
pub mod fpe1;
#[doc = "FPE2 (rw) register accessor: PWM Fault Protection Enable Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fpe2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpe2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpe2`] module"]
#[doc(alias = "FPE2")]
pub type Fpe2 = crate::Reg<fpe2::Fpe2Spec>;
#[doc = "PWM Fault Protection Enable Register 2"]
pub mod fpe2;
#[doc = "ELMR (rw) register accessor: PWM Event Line 0 Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`elmr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@elmr`] module"]
#[doc(alias = "ELMR")]
pub type Elmr = crate::Reg<elmr::ElmrSpec>;
#[doc = "PWM Event Line 0 Mode Register"]
pub mod elmr;
#[doc = "SMMR (rw) register accessor: PWM Stepper Motor Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smmr`] module"]
#[doc(alias = "SMMR")]
pub type Smmr = crate::Reg<smmr::SmmrSpec>;
#[doc = "PWM Stepper Motor Mode Register"]
pub mod smmr;
#[doc = "WPCR (w) register accessor: PWM Write Protect Control Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpcr`] module"]
#[doc(alias = "WPCR")]
pub type Wpcr = crate::Reg<wpcr::WpcrSpec>;
#[doc = "PWM Write Protect Control Register"]
pub mod wpcr;
#[doc = "WPSR (r) register accessor: PWM Write Protect Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`] module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "PWM Write Protect Status Register"]
pub mod wpsr;
#[doc = "One of the eight comparison units"]
pub use self::compare::Compare;
#[doc = r"Cluster"]
#[doc = "One of the eight comparison units"]
pub mod compare;
#[doc = "One of the eight channels"]
pub use self::channel::Channel;
#[doc = r"Cluster"]
#[doc = "One of the eight channels"]
pub mod channel;
#[doc = "TPR (rw) register accessor: Transmit Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpr`] module"]
#[doc(alias = "TPR")]
pub type Tpr = crate::Reg<tpr::TprSpec>;
#[doc = "Transmit Pointer Register"]
pub mod tpr;
#[doc = "TCR (rw) register accessor: Transmit Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`] module"]
#[doc(alias = "TCR")]
pub type Tcr = crate::Reg<tcr::TcrSpec>;
#[doc = "Transmit Counter Register"]
pub mod tcr;
#[doc = "TNPR (rw) register accessor: Transmit Next Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tnpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tnpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tnpr`] module"]
#[doc(alias = "TNPR")]
pub type Tnpr = crate::Reg<tnpr::TnprSpec>;
#[doc = "Transmit Next Pointer Register"]
pub mod tnpr;
#[doc = "TNCR (rw) register accessor: Transmit Next Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tncr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tncr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tncr`] module"]
#[doc(alias = "TNCR")]
pub type Tncr = crate::Reg<tncr::TncrSpec>;
#[doc = "Transmit Next Counter Register"]
pub mod tncr;
#[doc = "PTCR (w) register accessor: Transfer Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptcr`] module"]
#[doc(alias = "PTCR")]
pub type Ptcr = crate::Reg<ptcr::PtcrSpec>;
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "PTSR (r) register accessor: Transfer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptsr`] module"]
#[doc(alias = "PTSR")]
pub type Ptsr = crate::Reg<ptsr::PtsrSpec>;
#[doc = "Transfer Status Register"]
pub mod ptsr;
