use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "rdrand")]
pub unsafe fn _rdrand16_step(val: &mut u16) -> i32 {
    ::mem::transmute(::arch::x86::_rdrand16_step(::mem::transmute(val)))
}

#[inline]
#[target_feature(enable = "rdrand")]
pub unsafe fn _rdrand32_step(val: &mut u32) -> i32 {
    ::mem::transmute(::arch::x86::_rdrand32_step(::mem::transmute(val)))
}

#[inline]
#[target_feature(enable = "rdseed")]
pub unsafe fn _rdseed16_step(val: &mut u16) -> i32 {
    ::mem::transmute(::arch::x86::_rdseed16_step(::mem::transmute(val)))
}

#[inline]
#[target_feature(enable = "rdseed")]
pub unsafe fn _rdseed32_step(val: &mut u32) -> i32 {
    ::mem::transmute(::arch::x86::_rdseed32_step(::mem::transmute(val)))
}

