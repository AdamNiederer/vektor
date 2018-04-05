use ::arch::x86_64::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "bmi1")]
pub unsafe fn _bextr_u64(a: u64, start: u32, len: u32) -> u64 {
    ::mem::transmute(::arch::x86_64::_bextr_u64(::mem::transmute(a), ::mem::transmute(start), ::mem::transmute(len)))
}

#[inline]
#[target_feature(enable = "bmi1")]
pub unsafe fn _bextr2_u64(a: u64, control: u64) -> u64 {
    ::mem::transmute(::arch::x86_64::_bextr2_u64(::mem::transmute(a), ::mem::transmute(control)))
}

#[inline]
#[target_feature(enable = "bmi1")]
pub unsafe fn _andn_u64(a: u64, b: u64) -> u64 {
    ::mem::transmute(::arch::x86_64::_andn_u64(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "bmi1")]
pub unsafe fn _blsi_u64(x: u64) -> u64 {
    ::mem::transmute(::arch::x86_64::_blsi_u64(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "bmi1")]
pub unsafe fn _blsmsk_u64(x: u64) -> u64 {
    ::mem::transmute(::arch::x86_64::_blsmsk_u64(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "bmi1")]
pub unsafe fn _blsr_u64(x: u64) -> u64 {
    ::mem::transmute(::arch::x86_64::_blsr_u64(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "bmi1")]
pub unsafe fn _tzcnt_u64(x: u64) -> u64 {
    ::mem::transmute(::arch::x86_64::_tzcnt_u64(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "bmi1")]
pub unsafe fn _mm_tzcnt_64(x: u64) -> i64 {
    ::mem::transmute(::arch::x86_64::_mm_tzcnt_64(::mem::transmute(x)))
}

