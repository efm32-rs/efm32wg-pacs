#[doc = "Register `IFS` writer"]
pub struct W(crate::W<IFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFS_SPEC>;
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
impl From<crate::W<IFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPIOC` writer - Set FPIOC Interrupt Flag"]
pub type FPIOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `FPDZC` writer - Set FPDZC Interrupt Flag"]
pub type FPDZC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `FPUFC` writer - Set FPUFC Interrupt Flag"]
pub type FPUFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `FPOFC` writer - Set FPOFC Interrupt Flag"]
pub type FPOFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `FPIDC` writer - Set FPIDC Interrupt Flag"]
pub type FPIDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `FPIXC` writer - Set FPIXC Interrupt Flag"]
pub type FPIXC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Set FPIOC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpioc(&mut self) -> FPIOC_W<0> {
        FPIOC_W::new(self)
    }
    #[doc = "Bit 1 - Set FPDZC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpdzc(&mut self) -> FPDZC_W<1> {
        FPDZC_W::new(self)
    }
    #[doc = "Bit 2 - Set FPUFC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpufc(&mut self) -> FPUFC_W<2> {
        FPUFC_W::new(self)
    }
    #[doc = "Bit 3 - Set FPOFC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpofc(&mut self) -> FPOFC_W<3> {
        FPOFC_W::new(self)
    }
    #[doc = "Bit 4 - Set FPIDC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpidc(&mut self) -> FPIDC_W<4> {
        FPIDC_W::new(self)
    }
    #[doc = "Bit 5 - Set FPIXC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpixc(&mut self) -> FPIXC_W<5> {
        FPIXC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](index.html) module"]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifs::W](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
