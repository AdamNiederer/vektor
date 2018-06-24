#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Unsigned multiply without affecting flags.
///
/// Unsigned multiplication of `a` with `b` returning a pair `(lo, hi)` with
/// the low half and the high half of the result.
#[inline]
// LLVM BUG (should be mulxl): https://bugs.llvm.org/show_bug.cgi?id=34232
#[cfg_attr(all(test, target_arch = "x86_64"), assert_instr(imul))]
#[cfg_attr(all(test, target_arch = "x86"), assert_instr(mulx))]
#[target_feature(enable = "bmi2")]
pub unsafe fn _mulx_u32(a: u32, b: u32, hi: &mut u32) -> u32 {
    crate::mem::transmute(crate::myarch::_mulx_u32(crate::mem::transmute(a), crate::mem::transmute(b), crate::mem::transmute(hi)))
}

/// Zero higher bits of `a` >= `index`.
#[inline]
#[target_feature(enable = "bmi2")]
#[cfg_attr(test, assert_instr(bzhi))]
pub unsafe fn _bzhi_u32(a: u32, index: u32) -> u32 {
    crate::mem::transmute(crate::myarch::_bzhi_u32(crate::mem::transmute(a), crate::mem::transmute(index)))
}

/// Scatter contiguous low order bits of `a` to the result at the positions
/// specified by the `mask`.
#[inline]
#[target_feature(enable = "bmi2")]
#[cfg_attr(test, assert_instr(pdep))]
pub unsafe fn _pdep_u32(a: u32, mask: u32) -> u32 {
    crate::mem::transmute(crate::myarch::_pdep_u32(crate::mem::transmute(a), crate::mem::transmute(mask)))
}

/// Gathers the bits of `x` specified by the `mask` into the contiguous low
/// order bit positions of the result.
#[inline]
#[target_feature(enable = "bmi2")]
#[cfg_attr(test, assert_instr(pext))]
pub unsafe fn _pext_u32(a: u32, mask: u32) -> u32 {
    crate::mem::transmute(crate::myarch::_pext_u32(crate::mem::transmute(a), crate::mem::transmute(mask)))
}

