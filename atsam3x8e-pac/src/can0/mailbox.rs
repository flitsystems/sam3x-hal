#[repr(C)]
#[doc = "One of the eight mailboxes"]
#[doc(alias = "MAILBOX")]
pub struct Mailbox {
    mmr: Mmr,
    mam: Mam,
    mid: Mid,
    mfid: Mfid,
    msr: Msr,
    mdl: Mdl,
    mdh: Mdh,
    mcr: Mcr,
}
impl Mailbox {
    #[doc = "0x00 - Mailbox Mode Register (MB = 0)"]
    #[inline(always)]
    pub const fn mmr(&self) -> &Mmr {
        &self.mmr
    }
    #[doc = "0x04 - Mailbox Acceptance Mask Register (MB = 0)"]
    #[inline(always)]
    pub const fn mam(&self) -> &Mam {
        &self.mam
    }
    #[doc = "0x08 - Mailbox ID Register (MB = 0)"]
    #[inline(always)]
    pub const fn mid(&self) -> &Mid {
        &self.mid
    }
    #[doc = "0x0c - Mailbox Family ID Register (MB = 0)"]
    #[inline(always)]
    pub const fn mfid(&self) -> &Mfid {
        &self.mfid
    }
    #[doc = "0x10 - Mailbox Status Register (MB = 0)"]
    #[inline(always)]
    pub const fn msr(&self) -> &Msr {
        &self.msr
    }
    #[doc = "0x14 - Mailbox Data Low Register (MB = 0)"]
    #[inline(always)]
    pub const fn mdl(&self) -> &Mdl {
        &self.mdl
    }
    #[doc = "0x18 - Mailbox Data High Register (MB = 0)"]
    #[inline(always)]
    pub const fn mdh(&self) -> &Mdh {
        &self.mdh
    }
    #[doc = "0x1c - Mailbox Control Register (MB = 0)"]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
}
#[doc = "MMR (rw) register accessor: Mailbox Mode Register (MB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr`] module"]
#[doc(alias = "MMR")]
pub type Mmr = crate::Reg<mmr::MmrSpec>;
#[doc = "Mailbox Mode Register (MB = 0)"]
pub mod mmr;
#[doc = "MAM (rw) register accessor: Mailbox Acceptance Mask Register (MB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mam`] module"]
#[doc(alias = "MAM")]
pub type Mam = crate::Reg<mam::MamSpec>;
#[doc = "Mailbox Acceptance Mask Register (MB = 0)"]
pub mod mam;
#[doc = "MID (rw) register accessor: Mailbox ID Register (MB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid`] module"]
#[doc(alias = "MID")]
pub type Mid = crate::Reg<mid::MidSpec>;
#[doc = "Mailbox ID Register (MB = 0)"]
pub mod mid;
#[doc = "MFID (r) register accessor: Mailbox Family ID Register (MB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mfid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfid`] module"]
#[doc(alias = "MFID")]
pub type Mfid = crate::Reg<mfid::MfidSpec>;
#[doc = "Mailbox Family ID Register (MB = 0)"]
pub mod mfid;
#[doc = "MSR (r) register accessor: Mailbox Status Register (MB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`] module"]
#[doc(alias = "MSR")]
pub type Msr = crate::Reg<msr::MsrSpec>;
#[doc = "Mailbox Status Register (MB = 0)"]
pub mod msr;
#[doc = "MDL (rw) register accessor: Mailbox Data Low Register (MB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdl`] module"]
#[doc(alias = "MDL")]
pub type Mdl = crate::Reg<mdl::MdlSpec>;
#[doc = "Mailbox Data Low Register (MB = 0)"]
pub mod mdl;
#[doc = "MDH (rw) register accessor: Mailbox Data High Register (MB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdh`] module"]
#[doc(alias = "MDH")]
pub type Mdh = crate::Reg<mdh::MdhSpec>;
#[doc = "Mailbox Data High Register (MB = 0)"]
pub mod mdh;
#[doc = "MCR (w) register accessor: Mailbox Control Register (MB = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`] module"]
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "Mailbox Control Register (MB = 0)"]
pub mod mcr;
