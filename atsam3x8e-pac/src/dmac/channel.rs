#[repr(C)]
#[doc = "One of the six channels"]
#[doc(alias = "CHANNEL")]
pub struct Channel {
    saddr: Saddr,
    daddr: Daddr,
    dscr: Dscr,
    ctrla: Ctrla,
    ctrlb: Ctrlb,
    cfg: Cfg,
}
impl Channel {
    #[doc = "0x00 - DMAC Channel Source Address Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn saddr(&self) -> &Saddr {
        &self.saddr
    }
    #[doc = "0x04 - DMAC Channel Destination Address Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn daddr(&self) -> &Daddr {
        &self.daddr
    }
    #[doc = "0x08 - DMAC Channel Descriptor Address Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn dscr(&self) -> &Dscr {
        &self.dscr
    }
    #[doc = "0x0c - DMAC Channel Control A Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x10 - DMAC Channel Control B Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x14 - DMAC Channel Configuration Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
}
#[doc = "SADDR (rw) register accessor: DMAC Channel Source Address Register (ch_num = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`saddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr`] module"]
#[doc(alias = "SADDR")]
pub type Saddr = crate::Reg<saddr::SaddrSpec>;
#[doc = "DMAC Channel Source Address Register (ch_num = 0)"]
pub mod saddr;
#[doc = "DADDR (rw) register accessor: DMAC Channel Destination Address Register (ch_num = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`daddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr`] module"]
#[doc(alias = "DADDR")]
pub type Daddr = crate::Reg<daddr::DaddrSpec>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 0)"]
pub mod daddr;
#[doc = "DSCR (rw) register accessor: DMAC Channel Descriptor Address Register (ch_num = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`dscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr`] module"]
#[doc(alias = "DSCR")]
pub type Dscr = crate::Reg<dscr::DscrSpec>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 0)"]
pub mod dscr;
#[doc = "CTRLA (rw) register accessor: DMAC Channel Control A Register (ch_num = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "DMAC Channel Control A Register (ch_num = 0)"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: DMAC Channel Control B Register (ch_num = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`] module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "DMAC Channel Control B Register (ch_num = 0)"]
pub mod ctrlb;
#[doc = "CFG (rw) register accessor: DMAC Channel Configuration Register (ch_num = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "DMAC Channel Configuration Register (ch_num = 0)"]
pub mod cfg;
