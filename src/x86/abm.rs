use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "lzcnt")]
pub unsafe fn _lzcnt_u32(x: u32) -> u32 {
    ::mem::transmute(::arch::x86::_lzcnt_u32(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "popcnt")]
pub unsafe fn _popcnt32(x: i32) -> i32 {
    ::mem::transmute(::arch::x86::_popcnt32(::mem::transmute(x)))
}

