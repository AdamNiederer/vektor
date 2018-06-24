#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Adds the first component of `a` and `b`, the other components are copied
/// from `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(addss))]
pub unsafe fn _mm_add_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_add_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Adds __m128 vectors.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(addps))]
pub unsafe fn _mm_add_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_add_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtracts the first component of `b` from `a`, the other components are
/// copied from `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(subss))]
pub unsafe fn _mm_sub_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_sub_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtracts __m128 vectors.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(subps))]
pub unsafe fn _mm_sub_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_sub_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiplies the first component of `a` and `b`, the other components are
/// copied from `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(mulss))]
pub unsafe fn _mm_mul_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_mul_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiplies __m128 vectors.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(mulps))]
pub unsafe fn _mm_mul_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_mul_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Divides the first component of `b` by `a`, the other components are
/// copied from `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(divss))]
pub unsafe fn _mm_div_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_div_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Divides __m128 vectors.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(divps))]
pub unsafe fn _mm_div_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_div_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return the square root of the first single-precision (32-bit)
/// floating-point element in `a`, the other elements are unchanged.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(sqrtss))]
pub unsafe fn _mm_sqrt_ss(a: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_sqrt_ss(::mem::transmute(a)))
}

/// Return the square root of packed single-precision (32-bit) floating-point
/// elements in `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(sqrtps))]
pub unsafe fn _mm_sqrt_ps(a: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_sqrt_ps(::mem::transmute(a)))
}

/// Return the approximate reciprocal of the first single-precision
/// (32-bit) floating-point element in `a`, the other elements are unchanged.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(rcpss))]
pub unsafe fn _mm_rcp_ss(a: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_rcp_ss(::mem::transmute(a)))
}

/// Return the approximate reciprocal of packed single-precision (32-bit)
/// floating-point elements in `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(rcpps))]
pub unsafe fn _mm_rcp_ps(a: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_rcp_ps(::mem::transmute(a)))
}

/// Return the approximate reciprocal square root of the fist single-precision
/// (32-bit) floating-point elements in `a`, the other elements are unchanged.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(rsqrtss))]
pub unsafe fn _mm_rsqrt_ss(a: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_rsqrt_ss(::mem::transmute(a)))
}

/// Return the approximate reciprocal square root of packed single-precision
/// (32-bit) floating-point elements in `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(rsqrtps))]
pub unsafe fn _mm_rsqrt_ps(a: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_rsqrt_ps(::mem::transmute(a)))
}

/// Compare the first single-precision (32-bit) floating-point element of `a`
/// and `b`, and return the minimum value in the first element of the return
/// value, the other elements are copied from `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(minss))]
pub unsafe fn _mm_min_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_min_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed single-precision (32-bit) floating-point elements in `a` and
/// `b`, and return the corresponding minimum values.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(minps))]
pub unsafe fn _mm_min_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_min_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the first single-precision (32-bit) floating-point element of `a`
/// and `b`, and return the maximum value in the first element of the return
/// value, the other elements are copied from `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(maxss))]
pub unsafe fn _mm_max_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_max_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed single-precision (32-bit) floating-point elements in `a` and
/// `b`, and return the corresponding maximum values.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(maxps))]
pub unsafe fn _mm_max_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_max_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Bitwise AND of packed single-precision (32-bit) floating-point elements.
#[inline]
#[target_feature(enable = "sse")]
// i586 only seems to generate plain `and` instructions, so ignore it.
#[cfg_attr(all(test, any(target_arch = "x86_64", target_feature = "sse2")),
           assert_instr(andps))]
pub unsafe fn _mm_and_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_and_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Bitwise AND-NOT of packed single-precision (32-bit) floating-point
/// elements.
///
/// Computes `!a & b` for each bit in `a` and `b`.
#[inline]
#[target_feature(enable = "sse")]
// i586 only seems to generate plain `not` and `and` instructions, so ignore
// it.
#[cfg_attr(all(test, any(target_arch = "x86_64", target_feature = "sse2")),
           assert_instr(andnps))]
pub unsafe fn _mm_andnot_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_andnot_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Bitwise OR of packed single-precision (32-bit) floating-point elements.
#[inline]
#[target_feature(enable = "sse")]
// i586 only seems to generate plain `or` instructions, so we ignore it.
#[cfg_attr(all(test, any(target_arch = "x86_64", target_feature = "sse2")),
           assert_instr(orps))]
pub unsafe fn _mm_or_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_or_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Bitwise exclusive OR of packed single-precision (32-bit) floating-point
/// elements.
#[inline]
#[target_feature(enable = "sse")]
// i586 only seems to generate plain `xor` instructions, so we ignore it.
#[cfg_attr(all(test, any(target_arch = "x86_64", target_feature = "sse2")),
           assert_instr(xorps))]
pub unsafe fn _mm_xor_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_xor_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lowest `f32` of both inputs for equality. The lowest 32 bits of
/// the result will be `0xffffffff` if the two inputs are equal, or `0`
/// otherwise. The upper 96 bits of the result are the upper 96 bits of `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpeqss))]
pub unsafe fn _mm_cmpeq_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpeq_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lowest `f32` of both inputs for less than. The lowest 32 bits
/// of the result will be `0xffffffff` if `a.extract(0)` is less than
/// `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result are the
/// upper 96 bits of `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpltss))]
pub unsafe fn _mm_cmplt_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmplt_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lowest `f32` of both inputs for less than or equal. The lowest
/// 32 bits of the result will be `0xffffffff` if `a.extract(0)` is less than
/// or equal `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result
/// are the upper 96 bits of `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpless))]
pub unsafe fn _mm_cmple_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmple_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lowest `f32` of both inputs for greater than. The lowest 32
/// bits of the result will be `0xffffffff` if `a.extract(0)` is greater
/// than `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result
/// are the upper 96 bits of `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpltss))]
pub unsafe fn _mm_cmpgt_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpgt_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lowest `f32` of both inputs for greater than or equal. The
/// lowest 32 bits of the result will be `0xffffffff` if `a.extract(0)` is
/// greater than or equal `b.extract(0)`, or `0` otherwise. The upper 96 bits
/// of the result are the upper 96 bits of `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpless))]
pub unsafe fn _mm_cmpge_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpge_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lowest `f32` of both inputs for inequality. The lowest 32 bits
/// of the result will be `0xffffffff` if `a.extract(0)` is not equal to
/// `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result are the
/// upper 96 bits of `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpneqss))]
pub unsafe fn _mm_cmpneq_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpneq_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lowest `f32` of both inputs for not-less-than. The lowest 32
/// bits of the result will be `0xffffffff` if `a.extract(0)` is not less than
/// `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result are the
/// upper 96 bits of `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpnltss))]
pub unsafe fn _mm_cmpnlt_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpnlt_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lowest `f32` of both inputs for not-less-than-or-equal. The
/// lowest 32 bits of the result will be `0xffffffff` if `a.extract(0)` is not
/// less than or equal to `b.extract(0)`, or `0` otherwise. The upper 96 bits
/// of the result are the upper 96 bits of `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpnless))]
pub unsafe fn _mm_cmpnle_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpnle_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lowest `f32` of both inputs for not-greater-than. The lowest 32
/// bits of the result will be `0xffffffff` if `a.extract(0)` is not greater
/// than `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result are
/// the upper 96 bits of `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpnltss))]
pub unsafe fn _mm_cmpngt_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpngt_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lowest `f32` of both inputs for not-greater-than-or-equal. The
/// lowest 32 bits of the result will be `0xffffffff` if `a.extract(0)` is not
/// greater than or equal to `b.extract(0)`, or `0` otherwise. The upper 96
/// bits of the result are the upper 96 bits of `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpnless))]
pub unsafe fn _mm_cmpnge_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpnge_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Check if the lowest `f32` of both inputs are ordered. The lowest 32 bits of
/// the result will be `0xffffffff` if neither of `a.extract(0)` or
/// `b.extract(0)` is a NaN, or `0` otherwise. The upper 96 bits of the result
/// are the upper 96 bits of `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpordss))]
pub unsafe fn _mm_cmpord_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpord_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Check if the lowest `f32` of both inputs are unordered. The lowest 32 bits
/// of the result will be `0xffffffff` if any of `a.extract(0)` or
/// `b.extract(0)` is a NaN, or `0` otherwise. The upper 96 bits of the result
/// are the upper 96 bits of `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpunordss))]
pub unsafe fn _mm_cmpunord_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpunord_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input elements
/// were equal, or `0` otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpeqps))]
pub unsafe fn _mm_cmpeq_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpeq_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input element
/// in `a` is less than the corresponding element in `b`, or `0` otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpltps))]
pub unsafe fn _mm_cmplt_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmplt_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input element
/// in `a` is less than or equal to the corresponding element in `b`, or `0`
/// otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpleps))]
pub unsafe fn _mm_cmple_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmple_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input element
/// in `a` is greater than the corresponding element in `b`, or `0` otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpltps))]
pub unsafe fn _mm_cmpgt_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpgt_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input element
/// in `a` is greater than or equal to the corresponding element in `b`, or `0`
/// otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpleps))]
pub unsafe fn _mm_cmpge_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpge_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input elements
/// are *not* equal, or `0` otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpneqps))]
pub unsafe fn _mm_cmpneq_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpneq_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input element
/// in `a` is *not* less than the corresponding element in `b`, or `0`
/// otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpnltps))]
pub unsafe fn _mm_cmpnlt_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpnlt_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input element
/// in `a` is *not* less than or equal to the corresponding element in `b`, or
/// `0` otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpnleps))]
pub unsafe fn _mm_cmpnle_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpnle_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input element
/// in `a` is *not* greater than the corresponding element in `b`, or `0`
/// otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpnltps))]
pub unsafe fn _mm_cmpngt_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpngt_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input element
/// in `a` is *not* greater than or equal to the corresponding element in `b`,
/// or `0` otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpnleps))]
pub unsafe fn _mm_cmpnge_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpnge_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// Returns four floats that have one of two possible bit patterns. The element
/// in the output vector will be `0xffffffff` if the input elements in `a` and
/// `b` are ordered (i.e., neither of them is a NaN), or 0 otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpordps))]
pub unsafe fn _mm_cmpord_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpord_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// Returns four floats that have one of two possible bit patterns. The element
/// in the output vector will be `0xffffffff` if the input elements in `a` and
/// `b` are unordered (i.e., at least on of them is a NaN), or 0 otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cmpunordps))]
pub unsafe fn _mm_cmpunord_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cmpunord_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if they are equal, or `0` otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(comiss))]
pub unsafe fn _mm_comieq_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_comieq_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if the value from `a` is less than the one from `b`, or `0` otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(comiss))]
pub unsafe fn _mm_comilt_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_comilt_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if the value from `a` is less than or equal to the one from `b`, or `0`
/// otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(comiss))]
pub unsafe fn _mm_comile_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_comile_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if the value from `a` is greater than the one from `b`, or `0`
/// otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(comiss))]
pub unsafe fn _mm_comigt_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_comigt_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if the value from `a` is greater than or equal to the one from `b`, or
/// `0` otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(comiss))]
pub unsafe fn _mm_comige_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_comige_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if they are *not* equal, or `0` otherwise.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(comiss))]
pub unsafe fn _mm_comineq_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_comineq_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if they are equal, or `0` otherwise. This instruction will not signal
/// an exception if either argument is a quiet NaN.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(ucomiss))]
pub unsafe fn _mm_ucomieq_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_ucomieq_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if the value from `a` is less than the one from `b`, or `0` otherwise.
/// This instruction will not signal an exception if either argument is a quiet
/// NaN.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(ucomiss))]
pub unsafe fn _mm_ucomilt_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_ucomilt_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if the value from `a` is less than or equal to the one from `b`, or `0`
/// otherwise. This instruction will not signal an exception if either argument
/// is a quiet NaN.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(ucomiss))]
pub unsafe fn _mm_ucomile_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_ucomile_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if the value from `a` is greater than the one from `b`, or `0`
/// otherwise. This instruction will not signal an exception if either argument
/// is a quiet NaN.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(ucomiss))]
pub unsafe fn _mm_ucomigt_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_ucomigt_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if the value from `a` is greater than or equal to the one from `b`, or
/// `0` otherwise. This instruction will not signal an exception if either
/// argument is a quiet NaN.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(ucomiss))]
pub unsafe fn _mm_ucomige_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_ucomige_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if they are *not* equal, or `0` otherwise. This instruction will not
/// signal an exception if either argument is a quiet NaN.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(ucomiss))]
pub unsafe fn _mm_ucomineq_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_ucomineq_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Convert the lowest 32 bit float in the input vector to a 32 bit integer.
///
/// The result is rounded according to the current rounding mode. If the result
/// cannot be represented as a 32 bit integer the result will be `0x8000_0000`
/// (`std::i32::MIN`) or an invalid operation floating point exception if
/// unmasked (see [`_mm_setcsr`](fn._mm_setcsr.html)).
///
/// This corresponds to the `CVTSS2SI` instruction (with 32 bit output).
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cvtss2si))]
pub unsafe fn _mm_cvtss_si32(a: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_cvtss_si32(::mem::transmute(a)))
}

/// Alias for [`_mm_cvtss_si32`](fn._mm_cvtss_si32.html).
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cvtss2si))]
pub unsafe fn _mm_cvt_ss2si(a: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_cvt_ss2si(::mem::transmute(a)))
}

/// Convert the lowest 32 bit float in the input vector to a 32 bit integer
/// with
/// truncation.
///
/// The result is rounded always using truncation (round towards zero). If the
/// result cannot be represented as a 32 bit integer the result will be
/// `0x8000_0000` (`std::i32::MIN`) or an invalid operation floating point
/// exception if unmasked (see [`_mm_setcsr`](fn._mm_setcsr.html)).
///
/// This corresponds to the `CVTTSS2SI` instruction (with 32 bit output).
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cvttss2si))]
pub unsafe fn _mm_cvttss_si32(a: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_cvttss_si32(::mem::transmute(a)))
}

/// Alias for [`_mm_cvttss_si32`](fn._mm_cvttss_si32.html).
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cvttss2si))]
pub unsafe fn _mm_cvtt_ss2si(a: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_cvtt_ss2si(::mem::transmute(a)))
}

/// Extract the lowest 32 bit float from the input vector.
#[inline]
#[target_feature(enable = "sse")]
// No point in using assert_instrs. In Unix x86_64 calling convention this is a
// no-op, and on Windows it's just a `mov`.
pub unsafe fn _mm_cvtss_f32(a: f32x4) -> f32 {
    ::mem::transmute(::myarch::_mm_cvtss_f32(::mem::transmute(a)))
}

/// Convert a 32 bit integer to a 32 bit float. The result vector is the input
/// vector `a` with the lowest 32 bit float replaced by the converted integer.
///
/// This intrinsic corresponds to the `CVTSI2SS` instruction (with 32 bit
/// input).
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cvtsi2ss))]
pub unsafe fn _mm_cvtsi32_ss(a: f32x4, b: i32) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cvtsi32_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Alias for [`_mm_cvtsi32_ss`](fn._mm_cvtsi32_ss.html).
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(cvtsi2ss))]
pub unsafe fn _mm_cvt_si2ss(a: f32x4, b: i32) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cvt_si2ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Construct a `__m128` with the lowest element set to `a` and the rest set to
/// zero.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(movss))]
pub unsafe fn _mm_set_ss(a: f32) -> f32x4 {
    ::mem::transmute(::myarch::_mm_set_ss(::mem::transmute(a)))
}

/// Construct a `__m128` with all element set to `a`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(shufps))]
pub unsafe fn _mm_set1_ps(a: f32) -> f32x4 {
    ::mem::transmute(::myarch::_mm_set1_ps(::mem::transmute(a)))
}

/// Alias for [`_mm_set1_ps`](fn._mm_set1_ps.html)
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(shufps))]
pub unsafe fn _mm_set_ps1(a: f32) -> f32x4 {
    ::mem::transmute(::myarch::_mm_set_ps1(::mem::transmute(a)))
}

/// Construct a `__m128` from four floating point values highest to lowest.
///
/// Note that `a` will be the highest 32 bits of the result, and `d` the
/// lowest. This matches the standard way of writing bit patterns on x86:
///
/// ```text
///  bit    127 .. 96  95 .. 64  63 .. 32  31 .. 0
///        +---------+---------+---------+---------+
///        |    a    |    b    |    c    |    d    |   result
///        +---------+---------+---------+---------+
/// ```
///
/// Alternatively:
///
/// ```text
/// let v = _mm_set_ps(d, c, b, a);
/// ```
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(unpcklps))]
pub unsafe fn _mm_set_ps(a: f32, b: f32, c: f32, d: f32) -> f32x4 {
    ::mem::transmute(::myarch::_mm_set_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c), ::mem::transmute(d)))
}

/// Construct a `__m128` from four floating point values lowest to highest.
///
/// This matches the memory order of `__m128`, i.e., `a` will be the lowest 32
/// bits of the result, and `d` the highest.
///
/// ```text
/// assert_eq!(__m128::new(a, b, c, d), _mm_setr_ps(a, b, c, d));
/// ```
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(all(test, target_arch = "x86_64"), assert_instr(unpcklps))]
// On a 32-bit architecture it just copies the operands from the stack.
#[cfg_attr(all(test, target_arch = "x86"), assert_instr(movaps))]
pub unsafe fn _mm_setr_ps(a: f32, b: f32, c: f32, d: f32) -> f32x4 {
    ::mem::transmute(::myarch::_mm_setr_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c), ::mem::transmute(d)))
}

/// Construct a `__m128` with all elements initialized to zero.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(xorps))]
pub unsafe fn _mm_setzero_ps() -> f32x4 {
    ::mem::transmute(::myarch::_mm_setzero_ps())
}

/// Shuffle packed single-precision (32-bit) floating-point elements in `a` and
/// `b` using `mask`.
///
/// The lower half of result takes values from `a` and the higher half from
/// `b`. Mask is split to 2 control bits each to index the element from inputs.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(shufps, mask = 3))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_shuffle_ps(a: f32x4, b: f32x4, mask: u32) -> f32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_shuffle_ps(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(mask, call))
}

/// Unpack and interleave single-precision (32-bit) floating-point elements
/// from the higher half of `a` and `b`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(unpckhps))]
pub unsafe fn _mm_unpackhi_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_unpackhi_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave single-precision (32-bit) floating-point elements
/// from the lower half of `a` and `b`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(unpcklps))]
pub unsafe fn _mm_unpacklo_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_unpacklo_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Combine higher half of `a` and `b`. The highwe half of `b` occupies the
/// lower half of result.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(movhlps))]
pub unsafe fn _mm_movehl_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_movehl_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Combine lower half of `a` and `b`. The lower half of `b` occupies the
/// higher half of result.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(movlhps))]
pub unsafe fn _mm_movelh_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_movelh_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a mask of the most significant bit of each element in `a`.
///
/// The mask is stored in the 4 least significant bits of the return value.
/// All other bits are set to `0`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(movmskps))]
pub unsafe fn _mm_movemask_ps(a: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_movemask_ps(::mem::transmute(a)))
}

/// Set the upper two single-precision floating-point values with 64 bits of
/// data loaded from the address `p`; the lower two values are passed through
/// from `a`.
///
/// This corresponds to the `MOVHPS` / `MOVHPD` / `VMOVHPD` instructions.
///
/// ```rust
/// # #![feature(cfg_target_feature)]
/// # #![feature(target_feature, stdsimd)]
/// # #![cfg_attr(not(dox), no_std)]
/// # #[cfg(not(dox))]
/// # extern crate std as real_std;
/// # #[cfg(not(dox))]
/// # #[macro_use]
/// # extern crate stdsimd as std;
/// #[cfg(target_arch = "x86")]
/// use std::arch::x86::*;
/// #[cfg(target_arch = "x86_64")]
/// use std::arch::x86_64::*;
///
/// #
/// # // The real main function
/// # fn main() {
/// #     if is_x86_feature_detected!("sse") {
/// #         #[target_feature(enable = "sse")]
/// #         unsafe fn worker() {
/// #
/// let a = _mm_setr_ps(1.0, 2.0, 3.0, 4.0);
/// let data: [f32; 4] = [5.0, 6.0, 7.0, 8.0];
/// let r = _mm_loadh_pi(a, data[..].as_ptr() as *const _) ;
/// // assert_eq!(r, _mm_setr_ps(1.0, 2.0, 5.0, 6.0));
/// #
/// #         }
/// #         unsafe { worker(); }
/// #     }
/// # }
/// ```
#[inline]
#[target_feature(enable = "sse")]
// TODO: generates MOVHPD if the CPU supports SSE2.
// #[cfg_attr(test, assert_instr(movhps))]
#[cfg_attr(all(test, target_arch = "x86_64"), assert_instr(movhpd))]
// 32-bit codegen does not generate `movhps` or `movhpd`, but instead
// `movsd` followed by `unpcklpd` (or `movss'/`unpcklps` if there's no SSE2).
#[cfg_attr(all(test, target_arch = "x86", target_feature = "sse2"),
           assert_instr(movlhps))]
#[cfg_attr(all(test, target_arch = "x86", not(target_feature = "sse2")),
           assert_instr(unpcklps))]
// TODO: This function is actually not limited to floats, but that's what
// what matches the C type most closely: (__m128, *const __m64) -> __m128
pub unsafe fn _mm_loadh_pi(a: f32x4, p: *const __m64) -> f32x4 {
    ::mem::transmute(::myarch::_mm_loadh_pi(::mem::transmute(a), ::mem::transmute(p)))
}

/// Load two floats from `p` into the lower half of a `__m128`. The upper half
/// is copied from the upper half of `a`.
///
/// This corresponds to the `MOVLPS` / `MOVLDP` / `VMOVLDP` instructions.
///
/// ```rust
/// # #![feature(cfg_target_feature)]
/// # #![feature(target_feature, stdsimd)]
/// # #![cfg_attr(not(dox), no_std)]
/// # #[cfg(not(dox))]
/// # extern crate std as real_std;
/// # #[cfg(not(dox))]
/// # #[macro_use]
/// # extern crate stdsimd as std;
/// #[cfg(target_arch = "x86")]
/// use std::arch::x86::*;
/// #[cfg(target_arch = "x86_64")]
/// use std::arch::x86_64::*;
///
/// # // The real main function
/// # fn main() {
/// #     if is_x86_feature_detected!("sse") {
/// #         #[target_feature(enable = "sse")]
/// #         unsafe fn worker() {
/// #
/// let a = _mm_setr_ps(1.0, 2.0, 3.0, 4.0);
/// let data: [f32; 4] = [5.0, 6.0, 7.0, 8.0];
/// let r = _mm_loadh_pi(a, data[..].as_ptr() as *const _) ;
/// // assert_eq!(r, _mm_setr_ps(5.0, 6.0, 3.0, 4.0));
/// #
/// #         }
/// #         unsafe { worker(); }
/// #     }
/// # }
/// ```
#[inline]
#[target_feature(enable = "sse")]
// TODO: generates MOVLPD if the CPU supports SSE2.
// #[cfg_attr(test, assert_instr(movlps))]
#[cfg_attr(all(test, target_arch = "x86_64"), assert_instr(movlpd))]
// On 32-bit targets with SSE2, it just generates two `movsd`.
#[cfg_attr(all(test, target_arch = "x86", target_feature = "sse2"),
           assert_instr(movsd))]
// It should really generate "movlps", but oh well...
#[cfg_attr(all(test, target_arch = "x86", not(target_feature = "sse2")),
           assert_instr(movss))]
// TODO: Like _mm_loadh_pi, this also isn't limited to floats.
pub unsafe fn _mm_loadl_pi(a: f32x4, p: *const __m64) -> f32x4 {
    ::mem::transmute(::myarch::_mm_loadl_pi(::mem::transmute(a), ::mem::transmute(p)))
}

/// Construct a `__m128` with the lowest element read from `p` and the other
/// elements set to zero.
///
/// This corresponds to instructions `VMOVSS` / `MOVSS`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(movss))]
pub unsafe fn _mm_load_ss(p: *const f32) -> f32x4 {
    ::mem::transmute(::myarch::_mm_load_ss(::mem::transmute(p)))
}

/// Construct a `__m128` by duplicating the value read from `p` into all
/// elements.
///
/// This corresponds to instructions `VMOVSS` / `MOVSS` followed by some
/// shuffling.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(movss))]
pub unsafe fn _mm_load1_ps(p: *const f32) -> f32x4 {
    ::mem::transmute(::myarch::_mm_load1_ps(::mem::transmute(p)))
}

/// Alias for [`_mm_load1_ps`](fn._mm_load1_ps.html)
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(movss))]
pub unsafe fn _mm_load_ps1(p: *const f32) -> f32x4 {
    ::mem::transmute(::myarch::_mm_load_ps1(::mem::transmute(p)))
}

/// Load four `f32` values from *aligned* memory into a `__m128`. If the
/// pointer is not aligned to a 128-bit boundary (16 bytes) a general
/// protection fault will be triggered (fatal program crash).
///
/// Use [`_mm_loadu_ps`](fn._mm_loadu_ps.html) for potentially unaligned
/// memory.
///
/// This corresponds to instructions `VMOVAPS` / `MOVAPS`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(movaps))]
pub unsafe fn _mm_load_ps(p: *const f32) -> f32x4 {
    ::mem::transmute(::myarch::_mm_load_ps(::mem::transmute(p)))
}

/// Load four `f32` values from memory into a `__m128`. There are no
/// restrictions
/// on memory alignment. For aligned memory
/// [`_mm_load_ps`](fn._mm_load_ps.html)
/// may be faster.
///
/// This corresponds to instructions `VMOVUPS` / `MOVUPS`.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(movups))]
pub unsafe fn _mm_loadu_ps(p: *const f32) -> f32x4 {
    ::mem::transmute(::myarch::_mm_loadu_ps(::mem::transmute(p)))
}

/// Load four `f32` values from aligned memory into a `__m128` in reverse
/// order.
///
/// If the pointer is not aligned to a 128-bit boundary (16 bytes) a general
/// protection fault will be triggered (fatal program crash).
///
/// Functionally equivalent to the following code sequence (assuming `p`
/// satisfies the alignment restrictions):
///
/// ```text
/// let a0 = *p;
/// let a1 = *p.offset(1);
/// let a2 = *p.offset(2);
/// let a3 = *p.offset(3);
/// __m128::new(a3, a2, a1, a0)
/// ```
///
/// This corresponds to instructions `VMOVAPS` / `MOVAPS` followed by some
/// shuffling.
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(movaps))]
pub unsafe fn _mm_loadr_ps(p: *const f32) -> f32x4 {
    ::mem::transmute(::myarch::_mm_loadr_ps(::mem::transmute(p)))
}

/// Return a `__m128` with the first component from `b` and the remaining
/// components from `a`.
///
/// In other words for any `a` and `b`:
/// ```text
/// _mm_move_ss(a, b) == a.replace(0, b.extract(0))
/// ```
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(movss))]
pub unsafe fn _mm_move_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_move_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Get the unsigned 32-bit value of the MXCSR control and status register.
///
/// For more info see [`_mm_setcsr`](fn._mm_setcsr.html)
#[inline]
#[target_feature(enable = "sse")]
#[cfg_attr(test, assert_instr(stmxcsr))]
pub unsafe fn _mm_getcsr() -> u32 {
    ::mem::transmute(::myarch::_mm_getcsr())
}

/// See [`_mm_setcsr`](fn._mm_setcsr.html)
#[inline]
#[allow(non_snake_case)]
#[target_feature(enable = "sse")]
pub unsafe fn _MM_GET_EXCEPTION_MASK() -> u32 {
    ::mem::transmute(::myarch::_MM_GET_EXCEPTION_MASK())
}

/// See [`_mm_setcsr`](fn._mm_setcsr.html)
#[inline]
#[allow(non_snake_case)]
#[target_feature(enable = "sse")]
pub unsafe fn _MM_GET_EXCEPTION_STATE() -> u32 {
    ::mem::transmute(::myarch::_MM_GET_EXCEPTION_STATE())
}

/// See [`_mm_setcsr`](fn._mm_setcsr.html)
#[inline]
#[allow(non_snake_case)]
#[target_feature(enable = "sse")]
pub unsafe fn _MM_GET_FLUSH_ZERO_MODE() -> u32 {
    ::mem::transmute(::myarch::_MM_GET_FLUSH_ZERO_MODE())
}

/// See [`_mm_setcsr`](fn._mm_setcsr.html)
#[inline]
#[allow(non_snake_case)]
#[target_feature(enable = "sse")]
pub unsafe fn _MM_GET_ROUNDING_MODE() -> u32 {
    ::mem::transmute(::myarch::_MM_GET_ROUNDING_MODE())
}

/// Return vector of type __m128 with undefined elements.
#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_undefined_ps() -> f32x4 {
    ::mem::transmute(::myarch::_mm_undefined_ps())
}

/// Compares the packed 16-bit signed integers of `a` and `b` writing the
/// greatest value into the result.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pmaxsw))]
pub unsafe fn _mm_max_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_max_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compares the packed 16-bit signed integers of `a` and `b` writing the
/// greatest value into the result.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pmaxsw))]
pub unsafe fn _m_pmaxsw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_pmaxsw(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compares the packed 8-bit signed integers of `a` and `b` writing the
/// greatest value into the result.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pmaxub))]
pub unsafe fn _mm_max_pu8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_max_pu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compares the packed 8-bit signed integers of `a` and `b` writing the
/// greatest value into the result.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pmaxub))]
pub unsafe fn _m_pmaxub(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_pmaxub(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compares the packed 16-bit signed integers of `a` and `b` writing the
/// smallest value into the result.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pminsw))]
pub unsafe fn _mm_min_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_min_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compares the packed 16-bit signed integers of `a` and `b` writing the
/// smallest value into the result.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pminsw))]
pub unsafe fn _m_pminsw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_pminsw(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compares the packed 8-bit signed integers of `a` and `b` writing the
/// smallest value into the result.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pminub))]
pub unsafe fn _mm_min_pu8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_min_pu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compares the packed 8-bit signed integers of `a` and `b` writing the
/// smallest value into the result.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pminub))]
pub unsafe fn _m_pminub(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_pminub(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiplies packed 16-bit unsigned integer values and writes the
/// high-order 16 bits of each 32-bit product to the corresponding bits in
/// the destination.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pmulhuw))]
pub unsafe fn _mm_mulhi_pu16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_mulhi_pu16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiplies packed 16-bit unsigned integer values and writes the
/// high-order 16 bits of each 32-bit product to the corresponding bits in
/// the destination.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pmulhuw))]
pub unsafe fn _m_pmulhuw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_pmulhuw(::mem::transmute(a), ::mem::transmute(b)))
}

/// Computes the rounded averages of the packed unsigned 8-bit integer
/// values and writes the averages to the corresponding bits in the
/// destination.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pavgb))]
pub unsafe fn _mm_avg_pu8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_avg_pu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Computes the rounded averages of the packed unsigned 8-bit integer
/// values and writes the averages to the corresponding bits in the
/// destination.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pavgb))]
pub unsafe fn _m_pavgb(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_pavgb(::mem::transmute(a), ::mem::transmute(b)))
}

/// Computes the rounded averages of the packed unsigned 16-bit integer
/// values and writes the averages to the corresponding bits in the
/// destination.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pavgw))]
pub unsafe fn _mm_avg_pu16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_avg_pu16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Computes the rounded averages of the packed unsigned 16-bit integer
/// values and writes the averages to the corresponding bits in the
/// destination.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pavgw))]
pub unsafe fn _m_pavgw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_pavgw(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtracts the corresponding 8-bit unsigned integer values of the two
/// 64-bit vector operands and computes the absolute value for each of the
/// difference. Then sum of the 8 absolute differences is written to the
/// bits [15:0] of the destination; the remaining bits [63:16] are cleared.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(psadbw))]
pub unsafe fn _mm_sad_pu8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_sad_pu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtracts the corresponding 8-bit unsigned integer values of the two
/// 64-bit vector operands and computes the absolute value for each of the
/// difference. Then sum of the 8 absolute differences is written to the
/// bits [15:0] of the destination; the remaining bits [63:16] are cleared.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(psadbw))]
pub unsafe fn _m_psadbw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_psadbw(::mem::transmute(a), ::mem::transmute(b)))
}

/// Converts two elements of a 64-bit vector of [2 x i32] into two
/// floating point values and writes them to the lower 64-bits of the
/// destination. The remaining higher order elements of the destination are
/// copied from the corresponding elements in the first operand.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(cvtpi2ps))]
pub unsafe fn _mm_cvtpi32_ps(a: f32x4, b: __m64) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cvtpi32_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Converts two elements of a 64-bit vector of [2 x i32] into two
/// floating point values and writes them to the lower 64-bits of the
/// destination. The remaining higher order elements of the destination are
/// copied from the corresponding elements in the first operand.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(cvtpi2ps))]
pub unsafe fn _mm_cvt_pi2ps(a: f32x4, b: __m64) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cvt_pi2ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Converts the lower 4 8-bit values of `a` into a 128-bit vector of 4 `f32`s.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(cvtpi2ps))]
pub unsafe fn _mm_cvtpi8_ps(a: __m64) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cvtpi8_ps(::mem::transmute(a)))
}

/// Converts the lower 4 8-bit values of `a` into a 128-bit vector of 4 `f32`s.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(cvtpi2ps))]
pub unsafe fn _mm_cvtpu8_ps(a: __m64) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cvtpu8_ps(::mem::transmute(a)))
}

/// Converts a 64-bit vector of `i16`s into a 128-bit vector of 4 `f32`s.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(cvtpi2ps))]
pub unsafe fn _mm_cvtpi16_ps(a: __m64) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cvtpi16_ps(::mem::transmute(a)))
}

/// Converts a 64-bit vector of `i16`s into a 128-bit vector of 4 `f32`s.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(cvtpi2ps))]
pub unsafe fn _mm_cvtpu16_ps(a: __m64) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cvtpu16_ps(::mem::transmute(a)))
}

/// Converts the two 32-bit signed integer values from each 64-bit vector
/// operand of [2 x i32] into a 128-bit vector of [4 x float].
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(cvtpi2ps))]
pub unsafe fn _mm_cvtpi32x2_ps(a: __m64, b: __m64) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cvtpi32x2_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Extracts 16-bit element from a 64-bit vector of [4 x i16] and
/// returns it, as specified by the immediate integer operand.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pextrw, imm2 = 0))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_extract_pi16(a: __m64, imm2: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_extract_pi16(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm2, call))
}

/// Extracts 16-bit element from a 64-bit vector of [4 x i16] and
/// returns it, as specified by the immediate integer operand.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pextrw, imm2 = 0))]
#[rustc_args_required_const(1)]
pub unsafe fn _m_pextrw(a: __m64, imm2: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_m_pextrw(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm2, call))
}

/// Copies data from the 64-bit vector of [4 x i16] to the destination,
/// and inserts the lower 16-bits of an integer operand at the 16-bit offset
/// specified by the immediate operand `n`.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pinsrw, imm2 = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_insert_pi16(a: __m64, d: i32, imm2: i32) -> __m64 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_insert_pi16(::mem::transmute(a), ::mem::transmute(d), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm2, call))
}

/// Copies data from the 64-bit vector of [4 x i16] to the destination,
/// and inserts the lower 16-bits of an integer operand at the 16-bit offset
/// specified by the immediate operand `n`.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pinsrw, imm2 = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _m_pinsrw(a: __m64, d: i32, imm2: i32) -> __m64 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_m_pinsrw(::mem::transmute(a), ::mem::transmute(d), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm2, call))
}

/// Takes the most significant bit from each 8-bit element in a 64-bit
/// integer vector to create a 16-bit mask value. Zero-extends the value to
/// 32-bit integer and writes it to the destination.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pmovmskb))]
pub unsafe fn _mm_movemask_pi8(a: __m64) -> i32 {
    ::mem::transmute(::myarch::_mm_movemask_pi8(::mem::transmute(a)))
}

/// Takes the most significant bit from each 8-bit element in a 64-bit
/// integer vector to create a 16-bit mask value. Zero-extends the value to
/// 32-bit integer and writes it to the destination.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pmovmskb))]
pub unsafe fn _m_pmovmskb(a: __m64) -> i32 {
    ::mem::transmute(::myarch::_m_pmovmskb(::mem::transmute(a)))
}

/// Shuffles the 4 16-bit integers from a 64-bit integer vector to the
/// destination, as specified by the immediate value operand.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pshufw, imm8 = 0))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_shuffle_pi16(a: __m64, imm8: i32) -> __m64 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_shuffle_pi16(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shuffles the 4 16-bit integers from a 64-bit integer vector to the
/// destination, as specified by the immediate value operand.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(pshufw, imm8 = 0))]
#[rustc_args_required_const(1)]
pub unsafe fn _m_pshufw(a: __m64, imm8: i32) -> __m64 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_m_pshufw(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Convert the two lower packed single-precision (32-bit) floating-point
/// elements in `a` to packed 32-bit integers with truncation.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(cvttps2pi))]
pub unsafe fn _mm_cvttps_pi32(a: f32x4) -> __m64 {
    ::mem::transmute(::myarch::_mm_cvttps_pi32(::mem::transmute(a)))
}

/// Convert the two lower packed single-precision (32-bit) floating-point
/// elements in `a` to packed 32-bit integers with truncation.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(cvttps2pi))]
pub unsafe fn _mm_cvtt_ps2pi(a: f32x4) -> __m64 {
    ::mem::transmute(::myarch::_mm_cvtt_ps2pi(::mem::transmute(a)))
}

/// Convert the two lower packed single-precision (32-bit) floating-point
/// elements in `a` to packed 32-bit integers.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(cvtps2pi))]
pub unsafe fn _mm_cvtps_pi32(a: f32x4) -> __m64 {
    ::mem::transmute(::myarch::_mm_cvtps_pi32(::mem::transmute(a)))
}

/// Convert the two lower packed single-precision (32-bit) floating-point
/// elements in `a` to packed 32-bit integers.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(cvtps2pi))]
pub unsafe fn _mm_cvt_ps2pi(a: f32x4) -> __m64 {
    ::mem::transmute(::myarch::_mm_cvt_ps2pi(::mem::transmute(a)))
}

/// Convert packed single-precision (32-bit) floating-point elements in `a` to
/// packed 16-bit integers.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(cvtps2pi))]
pub unsafe fn _mm_cvtps_pi16(a: f32x4) -> __m64 {
    ::mem::transmute(::myarch::_mm_cvtps_pi16(::mem::transmute(a)))
}

/// Convert packed single-precision (32-bit) floating-point elements in `a` to
/// packed 8-bit integers, and returns theem in the lower 4 elements of the
/// result.
#[inline]
#[target_feature(enable = "sse,mmx")]
#[cfg_attr(test, assert_instr(cvtps2pi))]
pub unsafe fn _mm_cvtps_pi8(a: f32x4) -> __m64 {
    ::mem::transmute(::myarch::_mm_cvtps_pi8(::mem::transmute(a)))
}

