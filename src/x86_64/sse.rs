#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Convert the lowest 32 bit float in the input vector to a 64 bit integer.
///
/// The result is rounded according to the current rounding mode. If the result
/// cannot be represented as a 64 bit integer the result will be
/// `0x8000_0000_0000_0000` (`std::i64::MIN`) or trigger an invalid operation
/// floating point exception if unmasked (see
/// [`_mm_setcsr`](fn._mm_setcsr.html)).
///
/// This corresponds to the `CVTSS2SI` instruction (with 64 bit output).
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cvtss2si))]
pub unsafe fn _mm_cvtss_si64(a: f32x4) -> i64 {
    ::mem::transmute(::myarch::_mm_cvtss_si64(::mem::transmute(a)))
}

/// Convert the lowest 32 bit float in the input vector to a 64 bit integer
/// with truncation.
///
/// The result is rounded always using truncation (round towards zero). If the
/// result cannot be represented as a 64 bit integer the result will be
/// `0x8000_0000_0000_0000` (`std::i64::MIN`) or an invalid operation floating
/// point exception if unmasked (see [`_mm_setcsr`](fn._mm_setcsr.html)).
///
/// This corresponds to the `CVTTSS2SI` instruction (with 64 bit output).
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cvttss2si))]
pub unsafe fn _mm_cvttss_si64(a: f32x4) -> i64 {
    ::mem::transmute(::myarch::_mm_cvttss_si64(::mem::transmute(a)))
}

/// Convert a 64 bit integer to a 32 bit float. The result vector is the input
/// vector `a` with the lowest 32 bit float replaced by the converted integer.
///
/// This intrinsic corresponds to the `CVTSI2SS` instruction (with 64 bit
/// input).
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cvtsi2ss))]
pub unsafe fn _mm_cvtsi64_ss(a: f32x4, b: i64) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cvtsi64_ss(::mem::transmute(a), ::mem::transmute(b)))
}

