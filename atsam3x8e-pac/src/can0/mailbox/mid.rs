#[doc = "Register `MID` reader"]
pub type R = crate::R<MidSpec>;
#[doc = "Register `MID` writer"]
pub type W = crate::W<MidSpec>;
#[doc = "Field `MIDvB` reader - Complementary bits for identifier in extended frame mode"]
pub type MidvBR = crate::FieldReader<u32>;
#[doc = "Field `MIDvB` writer - Complementary bits for identifier in extended frame mode"]
pub type MidvBW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `MIDvA` reader - Identifier for standard frame mode"]
pub type MidvAR = crate::FieldReader<u16>;
#[doc = "Field `MIDvA` writer - Identifier for standard frame mode"]
pub type MidvAW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `MIDE` reader - Identifier Version"]
pub type MideR = crate::BitReader;
#[doc = "Field `MIDE` writer - Identifier Version"]
pub type MideW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:17 - Complementary bits for identifier in extended frame mode"]
    #[inline(always)]
    pub fn midv_b(&self) -> MidvBR {
        MidvBR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:28 - Identifier for standard frame mode"]
    #[inline(always)]
    pub fn midv_a(&self) -> MidvAR {
        MidvAR::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - Identifier Version"]
    #[inline(always)]
    pub fn mide(&self) -> MideR {
        MideR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - Complementary bits for identifier in extended frame mode"]
    #[inline(always)]
    pub fn midv_b(&mut self) -> MidvBW<'_, MidSpec> {
        MidvBW::new(self, 0)
    }
    #[doc = "Bits 18:28 - Identifier for standard frame mode"]
    #[inline(always)]
    pub fn midv_a(&mut self) -> MidvAW<'_, MidSpec> {
        MidvAW::new(self, 18)
    }
    #[doc = "Bit 29 - Identifier Version"]
    #[inline(always)]
    pub fn mide(&mut self) -> MideW<'_, MidSpec> {
        MideW::new(self, 29)
    }
}
#[doc = "Mailbox ID Register (MB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MidSpec;
impl crate::RegisterSpec for MidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mid::R`](R) reader structure"]
impl crate::Readable for MidSpec {}
#[doc = "`write(|w| ..)` method takes [`mid::W`](W) writer structure"]
impl crate::Writable for MidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MID to value 0"]
impl crate::Resettable for MidSpec {}
