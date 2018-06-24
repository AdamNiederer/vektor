#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Alternatively add and subtract packed single-precision (32-bit)
/// floating-point elements in `a` to/from packed elements in `b`.
#[inline]
#[target_feature(enable = "sse3")]
#[cfg_attr(test, assert_instr(addsubps))]
pub unsafe fn _mm_addsub_ps(a: f32x4, b: f32x4) -> f32x4 {
    crate::mem::transmute(crate::myarch::_mm_addsub_ps(crate::mem::transmute(a), crate::mem::transmute(b)))
}

/// Alternatively add and subtract packed double-precision (64-bit)
/// floating-point elements in `a` to/from packed elements in `b`.
#[inline]
#[target_feature(enable = "sse3")]
#[cfg_attr(test, assert_instr(addsubpd))]
pub unsafe fn _mm_addsub_pd(a: f64x2, b: f64x2) -> f64x2 {
    crate::mem::transmute(crate::myarch::_mm_addsub_pd(crate::mem::transmute(a), crate::mem::transmute(b)))
}

/// Horizontally add adjacent pairs of double-precision (64-bit)
/// floating-point elements in `a` and `b`, and pack the results.
#[inline]
#[target_feature(enable = "sse3")]
#[cfg_attr(test, assert_instr(haddpd))]
pub unsafe fn _mm_hadd_pd(a: f64x2, b: f64x2) -> f64x2 {
    crate::mem::transmute(crate::myarch::_mm_hadd_pd(crate::mem::transmute(a), crate::mem::transmute(b)))
}

/// Horizontally add adjacent pairs of single-precision (32-bit)
/// floating-point elements in `a` and `b`, and pack the results.
#[inline]
#[target_feature(enable = "sse3")]
#[cfg_attr(test, assert_instr(haddps))]
pub unsafe fn _mm_hadd_ps(a: f32x4, b: f32x4) -> f32x4 {
    crate::mem::transmute(crate::myarch::_mm_hadd_ps(crate::mem::transmute(a), crate::mem::transmute(b)))
}

/// Horizontally subtract adjacent pairs of double-precision (64-bit)
/// floating-point elements in `a` and `b`, and pack the results.
#[inline]
#[target_feature(enable = "sse3")]
#[cfg_attr(test, assert_instr(hsubpd))]
pub unsafe fn _mm_hsub_pd(a: f64x2, b: f64x2) -> f64x2 {
    crate::mem::transmute(crate::myarch::_mm_hsub_pd(crate::mem::transmute(a), crate::mem::transmute(b)))
}

/// Horizontally add adjacent pairs of single-precision (32-bit)
/// floating-point elements in `a` and `b`, and pack the results.
#[inline]
#[target_feature(enable = "sse3")]
#[cfg_attr(test, assert_instr(hsubps))]
pub unsafe fn _mm_hsub_ps(a: f32x4, b: f32x4) -> f32x4 {
    crate::mem::transmute(crate::myarch::_mm_hsub_ps(crate::mem::transmute(a), crate::mem::transmute(b)))
}

/// Load 128-bits of integer data from unaligned memory.
/// This intrinsic may perform better than `_mm_loadu_si128`
/// when the data crosses a cache line boundary.
#[inline]
#[target_feature(enable = "sse3")]
#[cfg_attr(test, assert_instr(lddqu))]
pub unsafe fn _mm_lddqu_si128(mem_addr: *const __m128i) -> __m128i {
    crate::mem::transmute(crate::myarch::_mm_lddqu_si128(crate::mem::transmute(mem_addr)))
}

/// Duplicate the low double-precision (64-bit) floating-point element
/// from `a`.
#[inline]
#[target_feature(enable = "sse3")]
#[cfg_attr(test, assert_instr(movddup))]
pub unsafe fn _mm_movedup_pd(a: f64x2) -> f64x2 {
    crate::mem::transmute(crate::myarch::_mm_movedup_pd(crate::mem::transmute(a)))
}

/// Load a double-precision (64-bit) floating-point element from memory
/// into both elements of return vector.
#[inline]
#[target_feature(enable = "sse3")]
#[cfg_attr(test, assert_instr(movddup))]
pub unsafe fn _mm_loaddup_pd(mem_addr: *const f64) -> f64x2 {
    crate::mem::transmute(crate::myarch::_mm_loaddup_pd(crate::mem::transmute(mem_addr)))
}

/// Duplicate odd-indexed single-precision (32-bit) floating-point elements
/// from `a`.
#[inline]
#[target_feature(enable = "sse3")]
#[cfg_attr(test, assert_instr(movshdup))]
pub unsafe fn _mm_movehdup_ps(a: f32x4) -> f32x4 {
    crate::mem::transmute(crate::myarch::_mm_movehdup_ps(crate::mem::transmute(a)))
}

/// Duplicate even-indexed single-precision (32-bit) floating-point elements
/// from `a`.
#[inline]
#[target_feature(enable = "sse3")]
#[cfg_attr(test, assert_instr(movsldup))]
pub unsafe fn _mm_moveldup_ps(a: f32x4) -> f32x4 {
    crate::mem::transmute(crate::myarch::_mm_moveldup_ps(crate::mem::transmute(a)))
}

