#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gcfg: Gcfg,
    en: En,
    sreq: Sreq,
    creq: Creq,
    last: Last,
    _reserved5: [u8; 0x04],
    ebcier: Ebcier,
    ebcidr: Ebcidr,
    ebcimr: Ebcimr,
    ebcisr: Ebcisr,
    cher: Cher,
    chdr: Chdr,
    chsr: Chsr,
    _reserved12: [u8; 0x08],
    channel: (),
    _reserved13: [u8; 0x01a8],
    wpmr: Wpmr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - DMAC Global Configuration Register"]
    #[inline(always)]
    pub const fn gcfg(&self) -> &Gcfg {
        &self.gcfg
    }
    #[doc = "0x04 - DMAC Enable Register"]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
    #[doc = "0x08 - DMAC Software Single Request Register"]
    #[inline(always)]
    pub const fn sreq(&self) -> &Sreq {
        &self.sreq
    }
    #[doc = "0x0c - DMAC Software Chunk Transfer Request Register"]
    #[inline(always)]
    pub const fn creq(&self) -> &Creq {
        &self.creq
    }
    #[doc = "0x10 - DMAC Software Last Transfer Flag Register"]
    #[inline(always)]
    pub const fn last(&self) -> &Last {
        &self.last
    }
    #[doc = "0x18 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register."]
    #[inline(always)]
    pub const fn ebcier(&self) -> &Ebcier {
        &self.ebcier
    }
    #[doc = "0x1c - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register."]
    #[inline(always)]
    pub const fn ebcidr(&self) -> &Ebcidr {
        &self.ebcidr
    }
    #[doc = "0x20 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register."]
    #[inline(always)]
    pub const fn ebcimr(&self) -> &Ebcimr {
        &self.ebcimr
    }
    #[doc = "0x24 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register."]
    #[inline(always)]
    pub const fn ebcisr(&self) -> &Ebcisr {
        &self.ebcisr
    }
    #[doc = "0x28 - DMAC Channel Handler Enable Register"]
    #[inline(always)]
    pub const fn cher(&self) -> &Cher {
        &self.cher
    }
    #[doc = "0x2c - DMAC Channel Handler Disable Register"]
    #[inline(always)]
    pub const fn chdr(&self) -> &Chdr {
        &self.chdr
    }
    #[doc = "0x30 - DMAC Channel Handler Status Register"]
    #[inline(always)]
    pub const fn chsr(&self) -> &Chsr {
        &self.chsr
    }
    #[doc = "0x3c..0xcc - One of the six channels"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &Channel {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(60)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3c..0xcc - One of the six channels"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &Channel> {
        (0..6).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(60)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x1e4 - DMAC Write Protect Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0x1e8 - DMAC Write Protect Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
}
#[doc = "GCFG (rw) register accessor: DMAC Global Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcfg`] module"]
#[doc(alias = "GCFG")]
pub type Gcfg = crate::Reg<gcfg::GcfgSpec>;
#[doc = "DMAC Global Configuration Register"]
pub mod gcfg;
#[doc = "EN (rw) register accessor: DMAC Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`] module"]
#[doc(alias = "EN")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = "DMAC Enable Register"]
pub mod en;
#[doc = "SREQ (rw) register accessor: DMAC Software Single Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sreq`] module"]
#[doc(alias = "SREQ")]
pub type Sreq = crate::Reg<sreq::SreqSpec>;
#[doc = "DMAC Software Single Request Register"]
pub mod sreq;
#[doc = "CREQ (rw) register accessor: DMAC Software Chunk Transfer Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`creq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`creq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@creq`] module"]
#[doc(alias = "CREQ")]
pub type Creq = crate::Reg<creq::CreqSpec>;
#[doc = "DMAC Software Chunk Transfer Request Register"]
pub mod creq;
#[doc = "LAST (rw) register accessor: DMAC Software Last Transfer Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`last::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`last::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@last`] module"]
#[doc(alias = "LAST")]
pub type Last = crate::Reg<last::LastSpec>;
#[doc = "DMAC Software Last Transfer Flag Register"]
pub mod last;
#[doc = "EBCIER (w) register accessor: DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register.\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ebcier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ebcier`] module"]
#[doc(alias = "EBCIER")]
pub type Ebcier = crate::Reg<ebcier::EbcierSpec>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register."]
pub mod ebcier;
#[doc = "EBCIDR (w) register accessor: DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register.\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ebcidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ebcidr`] module"]
#[doc(alias = "EBCIDR")]
pub type Ebcidr = crate::Reg<ebcidr::EbcidrSpec>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register."]
pub mod ebcidr;
#[doc = "EBCIMR (r) register accessor: DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ebcimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ebcimr`] module"]
#[doc(alias = "EBCIMR")]
pub type Ebcimr = crate::Reg<ebcimr::EbcimrSpec>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register."]
pub mod ebcimr;
#[doc = "EBCISR (r) register accessor: DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ebcisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ebcisr`] module"]
#[doc(alias = "EBCISR")]
pub type Ebcisr = crate::Reg<ebcisr::EbcisrSpec>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register."]
pub mod ebcisr;
#[doc = "CHER (w) register accessor: DMAC Channel Handler Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cher::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cher`] module"]
#[doc(alias = "CHER")]
pub type Cher = crate::Reg<cher::CherSpec>;
#[doc = "DMAC Channel Handler Enable Register"]
pub mod cher;
#[doc = "CHDR (w) register accessor: DMAC Channel Handler Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdr`] module"]
#[doc(alias = "CHDR")]
pub type Chdr = crate::Reg<chdr::ChdrSpec>;
#[doc = "DMAC Channel Handler Disable Register"]
pub mod chdr;
#[doc = "CHSR (r) register accessor: DMAC Channel Handler Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsr`] module"]
#[doc(alias = "CHSR")]
pub type Chsr = crate::Reg<chsr::ChsrSpec>;
#[doc = "DMAC Channel Handler Status Register"]
pub mod chsr;
#[doc = "One of the six channels"]
pub use self::channel::Channel;
#[doc = r"Cluster"]
#[doc = "One of the six channels"]
pub mod channel;
#[doc = "WPMR (rw) register accessor: DMAC Write Protect Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`] module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "DMAC Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: DMAC Write Protect Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`] module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "DMAC Write Protect Status Register"]
pub mod wpsr;
