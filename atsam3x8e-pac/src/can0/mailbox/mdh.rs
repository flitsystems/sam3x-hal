#[doc = "Register `MDH` reader"]
pub type R = crate::R<MdhSpec>;
#[doc = "Register `MDH` writer"]
pub type W = crate::W<MdhSpec>;
#[doc = "Field `MDH` reader - Message Data High Value"]
pub type MdhR = crate::FieldReader<u32>;
#[doc = "Field `MDH` writer - Message Data High Value"]
pub type MdhW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Data High Value"]
    #[inline(always)]
    pub fn mdh(&self) -> MdhR {
        MdhR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Data High Value"]
    #[inline(always)]
    pub fn mdh(&mut self) -> MdhW<'_, MdhSpec> {
        MdhW::new(self, 0)
    }
}
#[doc = "Mailbox Data High Register (MB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdhSpec;
impl crate::RegisterSpec for MdhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdh::R`](R) reader structure"]
impl crate::Readable for MdhSpec {}
#[doc = "`write(|w| ..)` method takes [`mdh::W`](W) writer structure"]
impl crate::Writable for MdhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDH to value 0"]
impl crate::Resettable for MdhSpec {}
