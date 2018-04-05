use ::arch::x86_64::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "bmi2")]
pub unsafe fn _mulx_u64(a: u64, b: u64, hi: &mut u64) -> u64 {
    ::mem::transmute(::arch::x86_64::_mulx_u64(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(hi)))
}

#[inline]
#[target_feature(enable = "bmi2")]
pub unsafe fn _bzhi_u64(a: u64, index: u32) -> u64 {
    ::mem::transmute(::arch::x86_64::_bzhi_u64(::mem::transmute(a), ::mem::transmute(index)))
}

#[inline]
#[target_feature(enable = "bmi2")]
pub unsafe fn _pdep_u64(a: u64, mask: u64) -> u64 {
    ::mem::transmute(::arch::x86_64::_pdep_u64(::mem::transmute(a), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "bmi2")]
pub unsafe fn _pext_u64(a: u64, mask: u64) -> u64 {
    ::mem::transmute(::arch::x86_64::_pext_u64(::mem::transmute(a), ::mem::transmute(mask)))
}

