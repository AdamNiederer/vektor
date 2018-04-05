use ::arch::x86_64::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "lzcnt")]
pub unsafe fn _lzcnt_u64(x: u64) -> u64 {
    ::mem::transmute(::arch::x86_64::_lzcnt_u64(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "popcnt")]
pub unsafe fn _popcnt64(x: i64) -> i32 {
    ::mem::transmute(::arch::x86_64::_popcnt64(::mem::transmute(x)))
}

