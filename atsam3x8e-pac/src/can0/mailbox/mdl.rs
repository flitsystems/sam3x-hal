#[doc = "Register `MDL` reader"]
pub type R = crate::R<MdlSpec>;
#[doc = "Register `MDL` writer"]
pub type W = crate::W<MdlSpec>;
#[doc = "Field `MDL` reader - Message Data Low Value"]
pub type MdlR = crate::FieldReader<u32>;
#[doc = "Field `MDL` writer - Message Data Low Value"]
pub type MdlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Data Low Value"]
    #[inline(always)]
    pub fn mdl(&self) -> MdlR {
        MdlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Data Low Value"]
    #[inline(always)]
    pub fn mdl(&mut self) -> MdlW<'_, MdlSpec> {
        MdlW::new(self, 0)
    }
}
#[doc = "Mailbox Data Low Register (MB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdlSpec;
impl crate::RegisterSpec for MdlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdl::R`](R) reader structure"]
impl crate::Readable for MdlSpec {}
#[doc = "`write(|w| ..)` method takes [`mdl::W`](W) writer structure"]
impl crate::Writable for MdlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDL to value 0"]
impl crate::Resettable for MdlSpec {}
