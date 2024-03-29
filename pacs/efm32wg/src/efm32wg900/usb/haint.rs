#[doc = "Register `HAINT` reader"]
pub struct R(crate::R<HAINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HAINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HAINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HAINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HAINT` reader - Channel Interrupt for channel 0 - 13."]
pub type HAINT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - Channel Interrupt for channel 0 - 13."]
    #[inline(always)]
    pub fn haint(&self) -> HAINT_R {
        HAINT_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "Host All Channels Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [haint](index.html) module"]
pub struct HAINT_SPEC;
impl crate::RegisterSpec for HAINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [haint::R](R) reader structure"]
impl crate::Readable for HAINT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HAINT to value 0"]
impl crate::Resettable for HAINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
