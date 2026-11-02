#[doc = "Register `HSTPIPICR0_INTPIPES` writer"]
pub type W = crate::W<IntpipesHstpipicr0IntpipesSpec>;
#[doc = "Field `RXINIC` writer - Received IN Data Interrupt Clear"]
pub type RxinicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTIC` writer - Transmitted OUT Data Interrupt Clear"]
pub type TxouticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFIC` writer - Underflow Interrupt Clear"]
pub type UnderficW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDIC` writer - NAKed Interrupt Clear"]
pub type NakedicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIC` writer - Overflow Interrupt Clear"]
pub type OverficW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTALLDIC` writer - Received STALLed Interrupt Clear"]
pub type RxstalldicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETIC` writer - Short Packet Interrupt Clear"]
pub type ShortpacketicW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Clear"]
    #[inline(always)]
    pub fn rxinic(&mut self) -> RxinicW<'_, IntpipesHstpipicr0IntpipesSpec> {
        RxinicW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Clear"]
    #[inline(always)]
    pub fn txoutic(&mut self) -> TxouticW<'_, IntpipesHstpipicr0IntpipesSpec> {
        TxouticW::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Clear"]
    #[inline(always)]
    pub fn underfic(&mut self) -> UnderficW<'_, IntpipesHstpipicr0IntpipesSpec> {
        UnderficW::new(self, 2)
    }
    #[doc = "Bit 4 - NAKed Interrupt Clear"]
    #[inline(always)]
    pub fn nakedic(&mut self) -> NakedicW<'_, IntpipesHstpipicr0IntpipesSpec> {
        NakedicW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn overfic(&mut self) -> OverficW<'_, IntpipesHstpipicr0IntpipesSpec> {
        OverficW::new(self, 5)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Clear"]
    #[inline(always)]
    pub fn rxstalldic(&mut self) -> RxstalldicW<'_, IntpipesHstpipicr0IntpipesSpec> {
        RxstalldicW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    pub fn shortpacketic(&mut self) -> ShortpacketicW<'_, IntpipesHstpipicr0IntpipesSpec> {
        ShortpacketicW::new(self, 7)
    }
}
#[doc = "Host Pipe Clear Register (n = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpipes_hstpipicr0_intpipes::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntpipesHstpipicr0IntpipesSpec;
impl crate::RegisterSpec for IntpipesHstpipicr0IntpipesSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intpipes_hstpipicr0_intpipes::W`](W) writer structure"]
impl crate::Writable for IntpipesHstpipicr0IntpipesSpec {
    type Safety = crate::Unsafe;
}
