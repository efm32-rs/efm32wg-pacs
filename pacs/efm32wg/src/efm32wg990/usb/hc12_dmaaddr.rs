#[doc = "Register `HC12_DMAADDR` reader"]
pub struct R(crate::R<HC12_DMAADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC12_DMAADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC12_DMAADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC12_DMAADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HC12_DMAADDR` writer"]
pub struct W(crate::W<HC12_DMAADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC12_DMAADDR_SPEC>;
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
impl From<crate::W<HC12_DMAADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC12_DMAADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAADDR` reader - DMA Address"]
pub type DMAADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMAADDR` writer - DMA Address"]
pub type DMAADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HC12_DMAADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<0> {
        DMAADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Channel x DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc12_dmaaddr](index.html) module"]
pub struct HC12_DMAADDR_SPEC;
impl crate::RegisterSpec for HC12_DMAADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc12_dmaaddr::R](R) reader structure"]
impl crate::Readable for HC12_DMAADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc12_dmaaddr::W](W) writer structure"]
impl crate::Writable for HC12_DMAADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HC12_DMAADDR to value 0"]
impl crate::Resettable for HC12_DMAADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
