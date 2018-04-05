use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "xsave")]
pub unsafe fn _xgetbv(xcr_no: u32) -> u64 {
    ::mem::transmute(::arch::x86::_xgetbv(::mem::transmute(xcr_no)))
}

