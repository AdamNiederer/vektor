use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "bmi2")]
pub unsafe fn _mulx_u32(a: u32, b: u32, hi: &mut u32) -> u32 {
    ::mem::transmute(::arch::x86::_mulx_u32(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(hi)))
}

#[inline]
#[target_feature(enable = "bmi2")]
pub unsafe fn _bzhi_u32(a: u32, index: u32) -> u32 {
    ::mem::transmute(::arch::x86::_bzhi_u32(::mem::transmute(a), ::mem::transmute(index)))
}

#[inline]
#[target_feature(enable = "bmi2")]
pub unsafe fn _pdep_u32(a: u32, mask: u32) -> u32 {
    ::mem::transmute(::arch::x86::_pdep_u32(::mem::transmute(a), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "bmi2")]
pub unsafe fn _pext_u32(a: u32, mask: u32) -> u32 {
    ::mem::transmute(::arch::x86::_pext_u32(::mem::transmute(a), ::mem::transmute(mask)))
}

