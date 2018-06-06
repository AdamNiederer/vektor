#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Read a hardware generated 16-bit random value and store the result in val.
/// Return 1 if a random value was generated, and 0 otherwise.
#[inline]
#[target_feature(enable = "rdrand")]
#[cfg_attr(test, assert_instr(rdrand))]
#[cfg_attr(feature = "cargo-clippy", allow(stutter))]
pub unsafe fn _rdrand16_step(val: &mut u16) -> i32 {
    ::mem::transmute(::myarch::_rdrand16_step(::mem::transmute(val)))
}

/// Read a hardware generated 32-bit random value and store the result in val.
/// Return 1 if a random value was generated, and 0 otherwise.
#[inline]
#[target_feature(enable = "rdrand")]
#[cfg_attr(test, assert_instr(rdrand))]
#[cfg_attr(feature = "cargo-clippy", allow(stutter))]
pub unsafe fn _rdrand32_step(val: &mut u32) -> i32 {
    ::mem::transmute(::myarch::_rdrand32_step(::mem::transmute(val)))
}

/// Read a 16-bit NIST SP800-90B and SP800-90C compliant random value and store
/// in val. Return 1 if a random value was generated, and 0 otherwise.
#[inline]
#[target_feature(enable = "rdseed")]
#[cfg_attr(test, assert_instr(rdseed))]
pub unsafe fn _rdseed16_step(val: &mut u16) -> i32 {
    ::mem::transmute(::myarch::_rdseed16_step(::mem::transmute(val)))
}

/// Read a 32-bit NIST SP800-90B and SP800-90C compliant random value and store
/// in val. Return 1 if a random value was generated, and 0 otherwise.
#[inline]
#[target_feature(enable = "rdseed")]
#[cfg_attr(test, assert_instr(rdseed))]
pub unsafe fn _rdseed32_step(val: &mut u32) -> i32 {
    ::mem::transmute(::myarch::_rdseed32_step(::mem::transmute(val)))
}

