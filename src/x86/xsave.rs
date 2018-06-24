#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Reads the contents of the extended control register `XCR`
/// specified in `xcr_no`.
#[inline]
#[target_feature(enable = "xsave")]
#[cfg_attr(test, assert_instr(xgetbv))]
pub unsafe fn _xgetbv(xcr_no: u32) -> u64 {
    crate::mem::transmute(crate::myarch::_xgetbv(crate::mem::transmute(xcr_no)))
}

