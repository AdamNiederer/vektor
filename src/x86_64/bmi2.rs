#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

 // calls an intrinsic
pub unsafe fn _mulx_u64(a: u64, b: u64, hi: &mut u64) -> u64 {
    ::mem::transmute(::myarch::_mulx_u64(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(hi)))
}

/// Zero higher bits of `a` >= `index`.
#[inline]
#[target_feature(enable = "bmi2")]
#[cfg_attr(test, assert_instr(bzhi))]
#[cfg(not(target_arch = "x86"))]
pub unsafe fn _bzhi_u64(a: u64, index: u32) -> u64 {
    ::mem::transmute(::myarch::_bzhi_u64(::mem::transmute(a), ::mem::transmute(index)))
}

/// Scatter contiguous low order bits of `a` to the result at the positions
/// specified by the `mask`.
#[inline]
#[target_feature(enable = "bmi2")]
#[cfg_attr(test, assert_instr(pdep))]
#[cfg(not(target_arch = "x86"))]
pub unsafe fn _pdep_u64(a: u64, mask: u64) -> u64 {
    ::mem::transmute(::myarch::_pdep_u64(::mem::transmute(a), ::mem::transmute(mask)))
}

/// Gathers the bits of `x` specified by the `mask` into the contiguous low
/// order bit positions of the result.
#[inline]
#[target_feature(enable = "bmi2")]
#[cfg_attr(test, assert_instr(pext))]
#[cfg(not(target_arch = "x86"))]
pub unsafe fn _pext_u64(a: u64, mask: u64) -> u64 {
    ::mem::transmute(::myarch::_pext_u64(::mem::transmute(a), ::mem::transmute(mask)))
}

