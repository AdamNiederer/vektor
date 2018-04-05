use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "bmi1")]
pub unsafe fn _bextr_u32(a: u32, start: u32, len: u32) -> u32 {
    ::mem::transmute(::arch::x86::_bextr_u32(::mem::transmute(a), ::mem::transmute(start), ::mem::transmute(len)))
}

#[inline]
#[target_feature(enable = "bmi1")]
pub unsafe fn _bextr2_u32(a: u32, control: u32) -> u32 {
    ::mem::transmute(::arch::x86::_bextr2_u32(::mem::transmute(a), ::mem::transmute(control)))
}

#[inline]
#[target_feature(enable = "bmi1")]
pub unsafe fn _andn_u32(a: u32, b: u32) -> u32 {
    ::mem::transmute(::arch::x86::_andn_u32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "bmi1")]
pub unsafe fn _blsi_u32(x: u32) -> u32 {
    ::mem::transmute(::arch::x86::_blsi_u32(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "bmi1")]
pub unsafe fn _blsmsk_u32(x: u32) -> u32 {
    ::mem::transmute(::arch::x86::_blsmsk_u32(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "bmi1")]
pub unsafe fn _blsr_u32(x: u32) -> u32 {
    ::mem::transmute(::arch::x86::_blsr_u32(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "bmi1")]
pub unsafe fn _tzcnt_u32(x: u32) -> u32 {
    ::mem::transmute(::arch::x86::_tzcnt_u32(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "bmi1")]
pub unsafe fn _mm_tzcnt_32(x: u32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_tzcnt_32(::mem::transmute(x)))
}

