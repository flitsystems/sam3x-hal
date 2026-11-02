#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mr: Mr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    sr: Sr,
    br: Br,
    tim: Tim,
    timestp: Timestp,
    ecr: Ecr,
    tcr: Tcr,
    acr: Acr,
    _reserved11: [u8; 0xb8],
    wpmr: Wpmr,
    wpsr: Wpsr,
    _reserved13: [u8; 0x0114],
    mailbox: [Mailbox; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
    }
    #[doc = "0x04 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x08 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x0c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x10 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x14 - Baudrate Register"]
    #[inline(always)]
    pub const fn br(&self) -> &Br {
        &self.br
    }
    #[doc = "0x18 - Timer Register"]
    #[inline(always)]
    pub const fn tim(&self) -> &Tim {
        &self.tim
    }
    #[doc = "0x1c - Timestamp Register"]
    #[inline(always)]
    pub const fn timestp(&self) -> &Timestp {
        &self.timestp
    }
    #[doc = "0x20 - Error Counter Register"]
    #[inline(always)]
    pub const fn ecr(&self) -> &Ecr {
        &self.ecr
    }
    #[doc = "0x24 - Transfer Command Register"]
    #[inline(always)]
    pub const fn tcr(&self) -> &Tcr {
        &self.tcr
    }
    #[doc = "0x28 - Abort Command Register"]
    #[inline(always)]
    pub const fn acr(&self) -> &Acr {
        &self.acr
    }
    #[doc = "0xe4 - Write Protect Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0xe8 - Write Protect Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
    #[doc = "0x200..0x300 - One of the eight mailboxes"]
    #[inline(always)]
    pub const fn mailbox(&self, n: usize) -> &Mailbox {
        &self.mailbox[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x300 - One of the eight mailboxes"]
    #[inline(always)]
    pub fn mailbox_iter(&self) -> impl Iterator<Item = &Mailbox> {
        self.mailbox.iter()
    }
}
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`] module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "BR (rw) register accessor: Baudrate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@br`] module"]
#[doc(alias = "BR")]
pub type Br = crate::Reg<br::BrSpec>;
#[doc = "Baudrate Register"]
pub mod br;
#[doc = "TIM (r) register accessor: Timer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim`] module"]
#[doc(alias = "TIM")]
pub type Tim = crate::Reg<tim::TimSpec>;
#[doc = "Timer Register"]
pub mod tim;
#[doc = "TIMESTP (r) register accessor: Timestamp Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timestp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestp`] module"]
#[doc(alias = "TIMESTP")]
pub type Timestp = crate::Reg<timestp::TimestpSpec>;
#[doc = "Timestamp Register"]
pub mod timestp;
#[doc = "ECR (r) register accessor: Error Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`] module"]
#[doc(alias = "ECR")]
pub type Ecr = crate::Reg<ecr::EcrSpec>;
#[doc = "Error Counter Register"]
pub mod ecr;
#[doc = "TCR (w) register accessor: Transfer Command Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`] module"]
#[doc(alias = "TCR")]
pub type Tcr = crate::Reg<tcr::TcrSpec>;
#[doc = "Transfer Command Register"]
pub mod tcr;
#[doc = "ACR (w) register accessor: Abort Command Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`] module"]
#[doc(alias = "ACR")]
pub type Acr = crate::Reg<acr::AcrSpec>;
#[doc = "Abort Command Register"]
pub mod acr;
#[doc = "WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`] module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`] module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
#[doc = "One of the eight mailboxes"]
pub use self::mailbox::Mailbox;
#[doc = r"Cluster"]
#[doc = "One of the eight mailboxes"]
pub mod mailbox;
