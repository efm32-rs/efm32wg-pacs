#[doc = "Register `CHALTS` writer"]
pub struct W(crate::W<CHALTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHALTS_SPEC>;
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
impl From<crate::W<CHALTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHALTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0ALTS` writer - Channel 0 Alternate Structure Set"]
pub type CH0ALTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHALTS_SPEC, bool, O>;
#[doc = "Field `CH1ALTS` writer - Channel 1 Alternate Structure Set"]
pub type CH1ALTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHALTS_SPEC, bool, O>;
#[doc = "Field `CH2ALTS` writer - Channel 2 Alternate Structure Set"]
pub type CH2ALTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHALTS_SPEC, bool, O>;
#[doc = "Field `CH3ALTS` writer - Channel 3 Alternate Structure Set"]
pub type CH3ALTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHALTS_SPEC, bool, O>;
#[doc = "Field `CH4ALTS` writer - Channel 4 Alternate Structure Set"]
pub type CH4ALTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHALTS_SPEC, bool, O>;
#[doc = "Field `CH5ALTS` writer - Channel 5 Alternate Structure Set"]
pub type CH5ALTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHALTS_SPEC, bool, O>;
#[doc = "Field `CH6ALTS` writer - Channel 6 Alternate Structure Set"]
pub type CH6ALTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHALTS_SPEC, bool, O>;
#[doc = "Field `CH7ALTS` writer - Channel 7 Alternate Structure Set"]
pub type CH7ALTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHALTS_SPEC, bool, O>;
#[doc = "Field `CH8ALTS` writer - Channel 8 Alternate Structure Set"]
pub type CH8ALTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHALTS_SPEC, bool, O>;
#[doc = "Field `CH9ALTS` writer - Channel 9 Alternate Structure Set"]
pub type CH9ALTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHALTS_SPEC, bool, O>;
#[doc = "Field `CH10ALTS` writer - Channel 10 Alternate Structure Set"]
pub type CH10ALTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHALTS_SPEC, bool, O>;
#[doc = "Field `CH11ALTS` writer - Channel 11 Alternate Structure Set"]
pub type CH11ALTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHALTS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch0alts(&mut self) -> CH0ALTS_W<0> {
        CH0ALTS_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch1alts(&mut self) -> CH1ALTS_W<1> {
        CH1ALTS_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch2alts(&mut self) -> CH2ALTS_W<2> {
        CH2ALTS_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch3alts(&mut self) -> CH3ALTS_W<3> {
        CH3ALTS_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch4alts(&mut self) -> CH4ALTS_W<4> {
        CH4ALTS_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch5alts(&mut self) -> CH5ALTS_W<5> {
        CH5ALTS_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch6alts(&mut self) -> CH6ALTS_W<6> {
        CH6ALTS_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch7alts(&mut self) -> CH7ALTS_W<7> {
        CH7ALTS_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch8alts(&mut self) -> CH8ALTS_W<8> {
        CH8ALTS_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch9alts(&mut self) -> CH9ALTS_W<9> {
        CH9ALTS_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch10alts(&mut self) -> CH10ALTS_W<10> {
        CH10ALTS_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch11alts(&mut self) -> CH11ALTS_W<11> {
        CH11ALTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Alternate Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chalts](index.html) module"]
pub struct CHALTS_SPEC;
impl crate::RegisterSpec for CHALTS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chalts::W](W) writer structure"]
impl crate::Writable for CHALTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHALTS to value 0"]
impl crate::Resettable for CHALTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
