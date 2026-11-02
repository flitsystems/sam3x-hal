#[doc = "Register `DEVEPTICR0_ISOENPT` writer"]
pub type W = crate::W<IsoenptDevepticr0IsoenptSpec>;
#[doc = "Field `TXINIC` writer - Transmitted IN Data Interrupt Clear"]
pub type TxinicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTIC` writer - Received OUT Data Interrupt Clear"]
pub type RxouticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFIC` writer - Underflow Interrupt Clear"]
pub type UnderficW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOINERRIC` writer - High bandwidth isochronous IN Underflow Error Interrupt Clear"]
pub type HbisoinerricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOFLUSHIC` writer - High Bandwidth Isochronous IN Flush Interrupt Clear"]
pub type HbisoflushicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIC` writer - Overflow Interrupt Clear"]
pub type OverficW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERRIC` writer - CRC Error Interrupt Clear"]
pub type CrcerricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETC` writer - Short Packet Interrupt Clear"]
pub type ShortpacketcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Clear"]
    #[inline(always)]
    pub fn txinic(&mut self) -> TxinicW<'_, IsoenptDevepticr0IsoenptSpec> {
        TxinicW::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    pub fn rxoutic(&mut self) -> RxouticW<'_, IsoenptDevepticr0IsoenptSpec> {
        RxouticW::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Clear"]
    #[inline(always)]
    pub fn underfic(&mut self) -> UnderficW<'_, IsoenptDevepticr0IsoenptSpec> {
        UnderficW::new(self, 2)
    }
    #[doc = "Bit 3 - High bandwidth isochronous IN Underflow Error Interrupt Clear"]
    #[inline(always)]
    pub fn hbisoinerric(&mut self) -> HbisoinerricW<'_, IsoenptDevepticr0IsoenptSpec> {
        HbisoinerricW::new(self, 3)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Clear"]
    #[inline(always)]
    pub fn hbisoflushic(&mut self) -> HbisoflushicW<'_, IsoenptDevepticr0IsoenptSpec> {
        HbisoflushicW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn overfic(&mut self) -> OverficW<'_, IsoenptDevepticr0IsoenptSpec> {
        OverficW::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Clear"]
    #[inline(always)]
    pub fn crcerric(&mut self) -> CrcerricW<'_, IsoenptDevepticr0IsoenptSpec> {
        CrcerricW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    pub fn shortpacketc(&mut self) -> ShortpacketcW<'_, IsoenptDevepticr0IsoenptSpec> {
        ShortpacketcW::new(self, 7)
    }
}
#[doc = "Device Endpoint Clear Register (n = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoenpt_devepticr0_isoenpt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsoenptDevepticr0IsoenptSpec;
impl crate::RegisterSpec for IsoenptDevepticr0IsoenptSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isoenpt_devepticr0_isoenpt::W`](W) writer structure"]
impl crate::Writable for IsoenptDevepticr0IsoenptSpec {
    type Safety = crate::Unsafe;
}
