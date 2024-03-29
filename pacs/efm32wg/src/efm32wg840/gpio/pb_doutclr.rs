#[doc = "Register `PB_DOUTCLR` writer"]
pub struct W(crate::W<PB_DOUTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB_DOUTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PB_DOUTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PB_DOUTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUTCLR` writer - Data Out Clear"]
pub type DOUTCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PB_DOUTCLR_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - Data Out Clear"]
    #[inline(always)]
    #[must_use]
    pub fn doutclr(&mut self) -> DOUTCLR_W<0> {
        DOUTCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Data Out Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_doutclr](index.html) module"]
pub struct PB_DOUTCLR_SPEC;
impl crate::RegisterSpec for PB_DOUTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pb_doutclr::W](W) writer structure"]
impl crate::Writable for PB_DOUTCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PB_DOUTCLR to value 0"]
impl crate::Resettable for PB_DOUTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
