#[doc = "Register `MFID` reader"]
pub type R = crate::R<MfidSpec>;
#[doc = "Field `MFID` reader - Family ID"]
pub type MfidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:28 - Family ID"]
    #[inline(always)]
    pub fn mfid(&self) -> MfidR {
        MfidR::new(self.bits & 0x1fff_ffff)
    }
}
#[doc = "Mailbox Family ID Register (MB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mfid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MfidSpec;
impl crate::RegisterSpec for MfidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mfid::R`](R) reader structure"]
impl crate::Readable for MfidSpec {}
#[doc = "`reset()` method sets MFID to value 0"]
impl crate::Resettable for MfidSpec {}
