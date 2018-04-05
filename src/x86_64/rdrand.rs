use ::arch::x86_64::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "rdrand")]
pub unsafe fn _rdrand64_step(val: &mut u64) -> i32 {
    ::mem::transmute(::arch::x86_64::_rdrand64_step(::mem::transmute(val)))
}

#[inline]
#[target_feature(enable = "rdseed")]
pub unsafe fn _rdseed64_step(val: &mut u64) -> i32 {
    ::mem::transmute(::arch::x86_64::_rdseed64_step(::mem::transmute(val)))
}

