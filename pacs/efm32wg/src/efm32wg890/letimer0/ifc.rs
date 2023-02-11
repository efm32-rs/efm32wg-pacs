#[doc = "Register `IFC` writer"]
pub struct W(crate::W<IFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFC_SPEC>;
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
impl From<crate::W<IFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP0` writer - Clear Compare Match 0 Interrupt Flag"]
pub type COMP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `COMP1` writer - Clear Compare Match 1 Interrupt Flag"]
pub type COMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `UF` writer - Clear Underflow Interrupt Flag"]
pub type UF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `REP0` writer - Clear Repeat Counter 0 Interrupt Flag"]
pub type REP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `REP1` writer - Clear Repeat Counter 1 Interrupt Flag"]
pub type REP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear Compare Match 0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn comp0(&mut self) -> COMP0_W<0> {
        COMP0_W::new(self)
    }
    #[doc = "Bit 1 - Clear Compare Match 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> COMP1_W<1> {
        COMP1_W::new(self)
    }
    #[doc = "Bit 2 - Clear Underflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UF_W<2> {
        UF_W::new(self)
    }
    #[doc = "Bit 3 - Clear Repeat Counter 0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rep0(&mut self) -> REP0_W<3> {
        REP0_W::new(self)
    }
    #[doc = "Bit 4 - Clear Repeat Counter 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rep1(&mut self) -> REP1_W<4> {
        REP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifc](index.html) module"]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifc::W](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
