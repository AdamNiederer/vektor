#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Convert the lower double-precision (64-bit) floating-point element in a to
/// a 64-bit integer.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtsd2si))]
pub unsafe fn _mm_cvtsd_si64(a: __m128d) -> i64 {
    ::mem::transmute(::myarch::_mm_cvtsd_si64(::mem::transmute(a)))
}

/// Alias for `_mm_cvtsd_si64`
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtsd2si))]
pub unsafe fn _mm_cvtsd_si64x(a: __m128d) -> i64 {
    ::mem::transmute(::myarch::_mm_cvtsd_si64x(::mem::transmute(a)))
}

/// Convert the lower double-precision (64-bit) floating-point element in `a`
/// to a 64-bit integer with truncation.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvttsd2si))]
pub unsafe fn _mm_cvttsd_si64(a: __m128d) -> i64 {
    ::mem::transmute(::myarch::_mm_cvttsd_si64(::mem::transmute(a)))
}

/// Alias for `_mm_cvttsd_si64`
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvttsd2si))]
pub unsafe fn _mm_cvttsd_si64x(a: __m128d) -> i64 {
    ::mem::transmute(::myarch::_mm_cvttsd_si64x(::mem::transmute(a)))
}

/// Return a vector whose lowest element is `a` and all higher elements are
/// `0`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(all(test, not(windows)), assert_instr(movq))]
pub unsafe fn _mm_cvtsi64_si128(a: i64) -> __m128i {
    ::mem::transmute(::myarch::_mm_cvtsi64_si128(::mem::transmute(a)))
}

/// Return a vector whose lowest element is `a` and all higher elements are
/// `0`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(all(test, not(windows)), assert_instr(movq))]
pub unsafe fn _mm_cvtsi64x_si128(a: i64) -> __m128i {
    ::mem::transmute(::myarch::_mm_cvtsi64x_si128(::mem::transmute(a)))
}

/// Return the lowest element of `a`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(all(test, not(windows)), assert_instr(movq))]
pub unsafe fn _mm_cvtsi128_si64(a: __m128i) -> i64 {
    ::mem::transmute(::myarch::_mm_cvtsi128_si64(::mem::transmute(a)))
}

/// Return the lowest element of `a`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(all(test, not(windows)), assert_instr(movq))]
pub unsafe fn _mm_cvtsi128_si64x(a: __m128i) -> i64 {
    ::mem::transmute(::myarch::_mm_cvtsi128_si64x(::mem::transmute(a)))
}

/// Return `a` with its lower element replaced by `b` after converting it to
/// an `f64`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtsi2sd))]
pub unsafe fn _mm_cvtsi64_sd(a: __m128d, b: i64) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cvtsi64_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return `a` with its lower element replaced by `b` after converting it to
/// an `f64`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtsi2sd))]
pub unsafe fn _mm_cvtsi64x_sd(a: __m128d, b: i64) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cvtsi64x_sd(::mem::transmute(a), ::mem::transmute(b)))
}

