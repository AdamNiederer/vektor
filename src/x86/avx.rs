#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Add packed double-precision (64-bit) floating-point elements
/// in `a` and `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vaddpd))]
pub unsafe fn _mm256_add_pd(a: f64x4, b: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_add_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed single-precision (32-bit) floating-point elements in `a` and
/// `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vaddps))]
pub unsafe fn _mm256_add_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_add_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of a packed double-precision (64-bit)
/// floating-point elements
/// in `a` and `b`.
#[inline]
#[target_feature(enable = "avx")]
// FIXME: Should be 'vandpd' instuction.
// See https://github.com/rust-lang-nursery/stdsimd/issues/71
#[cfg_attr(test, assert_instr(vandps))]
pub unsafe fn _mm256_and_pd(a: f64x4, b: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_and_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of packed single-precision (32-bit) floating-point
/// elements in `a` and `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vandps))]
pub unsafe fn _mm256_and_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_and_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise OR packed double-precision (64-bit) floating-point
/// elements in `a` and `b`.
#[inline]
#[target_feature(enable = "avx")]
// FIXME: Should be 'vorpd' instuction.
// See https://github.com/rust-lang-nursery/stdsimd/issues/71
#[cfg_attr(test, assert_instr(vorps))]
pub unsafe fn _mm256_or_pd(a: f64x4, b: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_or_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise OR packed single-precision (32-bit) floating-point
/// elements in `a` and `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vorps))]
pub unsafe fn _mm256_or_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_or_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Shuffle double-precision (64-bit) floating-point elements within 128-bit
/// lanes using the control in `imm8`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vshufpd, imm8 = 0x1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_shuffle_pd(a: f64x4, b: f64x4, imm8: i32) -> f64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_shuffle_pd(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shuffle single-precision (32-bit) floating-point elements in `a` within
/// 128-bit lanes using the control in `imm8`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vshufps, imm8 = 0x0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_shuffle_ps(a: f32x8, b: f32x8, imm8: i32) -> f32x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_shuffle_ps(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Compute the bitwise NOT of packed double-precision (64-bit) floating-point
/// elements in `a`
/// and then AND with `b`.
#[inline]
#[target_feature(enable = "avx")]
// FIXME: Should be 'vandnpd' instruction.
#[cfg_attr(test, assert_instr(vandnps))]
pub unsafe fn _mm256_andnot_pd(a: f64x4, b: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_andnot_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise NOT of packed single-precision (32-bit) floating-point
/// elements in `a`
/// and then AND with `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vandnps))]
pub unsafe fn _mm256_andnot_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_andnot_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed double-precision (64-bit) floating-point elements
/// in `a` and `b`, and return packed maximum values
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vmaxpd))]
pub unsafe fn _mm256_max_pd(a: f64x4, b: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_max_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed single-precision (32-bit) floating-point elements in `a`
/// and `b`, and return packed maximum values
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vmaxps))]
pub unsafe fn _mm256_max_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_max_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed double-precision (64-bit) floating-point elements
/// in `a` and `b`, and return packed minimum values
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vminpd))]
pub unsafe fn _mm256_min_pd(a: f64x4, b: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_min_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed single-precision (32-bit) floating-point elements in `a`
/// and `b`, and return packed minimum values
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vminps))]
pub unsafe fn _mm256_min_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_min_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed double-precision (64-bit) floating-point elements
/// in `a` and `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vmulpd))]
pub unsafe fn _mm256_mul_pd(a: f64x4, b: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_mul_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed single-precision (32-bit) floating-point elements in `a` and
/// `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vmulps))]
pub unsafe fn _mm256_mul_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_mul_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Alternatively add and subtract packed double-precision (64-bit)
/// floating-point elements in `a` to/from packed elements in `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vaddsubpd))]
pub unsafe fn _mm256_addsub_pd(a: f64x4, b: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_addsub_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Alternatively add and subtract packed single-precision (32-bit)
/// floating-point elements in `a` to/from packed elements in `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vaddsubps))]
pub unsafe fn _mm256_addsub_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_addsub_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed double-precision (64-bit) floating-point elements in `b`
/// from packed elements in `a`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vsubpd))]
pub unsafe fn _mm256_sub_pd(a: f64x4, b: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_sub_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed single-precision (32-bit) floating-point elements in `b`
/// from packed elements in `a`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vsubps))]
pub unsafe fn _mm256_sub_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_sub_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the division of each of the 8 packed 32-bit floating-point elements
/// in `a` by the corresponding packed elements in `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vdivps))]
pub unsafe fn _mm256_div_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_div_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the division of each of the 4 packed 64-bit floating-point elements
/// in `a` by the corresponding packed elements in `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vdivpd))]
pub unsafe fn _mm256_div_pd(a: f64x4, b: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_div_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Round packed double-precision (64-bit) floating point elements in `a`
/// according to the flag `b`. The value of `b` may be as follows:
///
/// - `0x00`: Round to the nearest whole number.
/// - `0x01`: Round down, toward negative infinity.
/// - `0x02`: Round up, toward positive infinity.
/// - `0x03`: Truncate the values.
///
/// For a complete list of options, check [the LLVM docs][llvm_docs].
///
/// [llvm_docs]: https://github.com/llvm-mirror/clang/blob/dcd8d797b20291f1a6b3e0ddda085aa2bbb382a8/lib/Headers/avxintrin.h#L382
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vroundpd, b = 0x3))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_round_pd(a: f64x4, b: i32) -> f64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_round_pd(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(b, call))
}

/// Round packed double-precision (64-bit) floating point elements in `a`
/// toward positive infinity.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vroundpd))]
pub unsafe fn _mm256_ceil_pd(a: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_ceil_pd(::mem::transmute(a)))
}

/// Round packed double-precision (64-bit) floating point elements in `a`
/// toward negative infinity.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vroundpd))]
pub unsafe fn _mm256_floor_pd(a: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_floor_pd(::mem::transmute(a)))
}

/// Round packed single-precision (32-bit) floating point elements in `a`
/// according to the flag `b`. The value of `b` may be as follows:
///
/// - `0x00`: Round to the nearest whole number.
/// - `0x01`: Round down, toward negative infinity.
/// - `0x02`: Round up, toward positive infinity.
/// - `0x03`: Truncate the values.
///
/// For a complete list of options, check [the LLVM docs][llvm_docs].
///
/// [llvm_docs]: https://github.com/llvm-mirror/clang/blob/dcd8d797b20291f1a6b3e0ddda085aa2bbb382a8/lib/Headers/avxintrin.h#L382
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vroundps, b = 0x00))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_round_ps(a: f32x8, b: i32) -> f32x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_round_ps(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(b, call))
}

/// Round packed single-precision (32-bit) floating point elements in `a`
/// toward positive infinity.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vroundps))]
pub unsafe fn _mm256_ceil_ps(a: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_ceil_ps(::mem::transmute(a)))
}

/// Round packed single-precision (32-bit) floating point elements in `a`
/// toward negative infinity.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vroundps))]
pub unsafe fn _mm256_floor_ps(a: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_floor_ps(::mem::transmute(a)))
}

/// Return the square root of packed single-precision (32-bit) floating point
/// elements in `a`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vsqrtps))]
pub unsafe fn _mm256_sqrt_ps(a: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_sqrt_ps(::mem::transmute(a)))
}

/// Return the square root of packed double-precision (64-bit) floating point
/// elements in `a`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vsqrtpd))]
pub unsafe fn _mm256_sqrt_pd(a: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_sqrt_pd(::mem::transmute(a)))
}

/// Blend packed double-precision (64-bit) floating-point elements from
/// `a` and `b` using control mask `imm8`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vblendpd, imm8 = 9))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_blend_pd(a: f64x4, b: f64x4, imm8: i32) -> f64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_blend_pd(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Blend packed single-precision (32-bit) floating-point elements from
/// `a` and `b` using control mask `imm8`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vblendps, imm8 = 9))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_blend_ps(a: f32x8, b: f32x8, imm8: i32) -> f32x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_blend_ps(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Blend packed double-precision (64-bit) floating-point elements from
/// `a` and `b` using `c` as a mask.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vblendvpd))]
pub unsafe fn _mm256_blendv_pd(a: f64x4, b: f64x4, c: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_blendv_pd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c)))
}

/// Blend packed single-precision (32-bit) floating-point elements from
/// `a` and `b` using `c` as a mask.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vblendvps))]
pub unsafe fn _mm256_blendv_ps(a: f32x8, b: f32x8, c: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_blendv_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c)))
}

/// Conditionally multiply the packed single-precision (32-bit) floating-point
/// elements in `a` and `b` using the high 4 bits in `imm8`,
/// sum the four products, and conditionally return the sum
///  using the low 4 bits of `imm8`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vdpps, imm8 = 0x0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_dp_ps(a: f32x8, b: f32x8, imm8: i32) -> f32x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_dp_ps(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Horizontal addition of adjacent pairs in the two packed vectors
/// of 4 64-bit floating points `a` and `b`.
/// In the result, sums of elements from `a` are returned in even locations,
/// while sums of elements from `b` are returned in odd locations.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vhaddpd))]
pub unsafe fn _mm256_hadd_pd(a: f64x4, b: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_hadd_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontal addition of adjacent pairs in the two packed vectors
/// of 8 32-bit floating points `a` and `b`.
/// In the result, sums of elements from `a` are returned in locations of
/// indices 0, 1, 4, 5; while sums of elements from `b` are locations
/// 2, 3, 6, 7.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vhaddps))]
pub unsafe fn _mm256_hadd_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_hadd_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontal subtraction of adjacent pairs in the two packed vectors
/// of 4 64-bit floating points `a` and `b`.
/// In the result, sums of elements from `a` are returned in even locations,
/// while sums of elements from `b` are returned in odd locations.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vhsubpd))]
pub unsafe fn _mm256_hsub_pd(a: f64x4, b: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_hsub_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontal subtraction of adjacent pairs in the two packed vectors
/// of 8 32-bit floating points `a` and `b`.
/// In the result, sums of elements from `a` are returned in locations of
/// indices 0, 1, 4, 5; while sums of elements from `b` are locations
/// 2, 3, 6, 7.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vhsubps))]
pub unsafe fn _mm256_hsub_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_hsub_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise XOR of packed double-precision (64-bit) floating-point
/// elements in `a` and `b`.
#[inline]
#[target_feature(enable = "avx")]
// FIXME Should be 'vxorpd' instruction.
#[cfg_attr(test, assert_instr(vxorps))]
pub unsafe fn _mm256_xor_pd(a: f64x4, b: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_xor_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise XOR of packed single-precision (32-bit) floating-point
/// elements in `a` and `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vxorps))]
pub unsafe fn _mm256_xor_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_xor_ps(::mem::transmute(a), ::mem::transmute(b)))
}

 // TODO Validate vcmppd
#[rustc_args_required_const(2)]
pub unsafe fn _mm_cmp_pd(a: f64x2, b: f64x2, imm8: i32) -> f64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmp_pd(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

 // TODO Validate vcmppd
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_cmp_pd(a: f64x4, b: f64x4, imm8: i32) -> f64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_cmp_pd(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

 // TODO Validate vcmpps
#[rustc_args_required_const(2)]
pub unsafe fn _mm_cmp_ps(a: f32x4, b: f32x4, imm8: i32) -> f32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmp_ps(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

 // TODO Validate vcmpps
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_cmp_ps(a: f32x8, b: f32x8, imm8: i32) -> f32x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_cmp_ps(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

 // TODO Validate vcmpsd
#[rustc_args_required_const(2)]
pub unsafe fn _mm_cmp_sd(a: f64x2, b: f64x2, imm8: i32) -> f64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmp_sd(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

 // TODO Validate vcmpss
#[rustc_args_required_const(2)]
pub unsafe fn _mm_cmp_ss(a: f32x4, b: f32x4, imm8: i32) -> f32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmp_ss(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Convert packed 32-bit integers in `a` to packed double-precision (64-bit)
/// floating-point elements.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vcvtdq2pd))]
pub unsafe fn _mm256_cvtepi32_pd(a: i32x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_cvtepi32_pd(::mem::transmute(a)))
}

/// Convert packed 32-bit integers in `a` to packed single-precision (32-bit)
/// floating-point elements.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vcvtdq2ps))]
pub unsafe fn _mm256_cvtepi32_ps(a: i32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_cvtepi32_ps(::mem::transmute(a)))
}

/// Convert packed double-precision (64-bit) floating-point elements in `a`
/// to packed single-precision (32-bit) floating-point elements.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vcvtpd2ps))]
pub unsafe fn _mm256_cvtpd_ps(a: f64x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm256_cvtpd_ps(::mem::transmute(a)))
}

/// Convert packed single-precision (32-bit) floating-point elements in `a`
/// to packed 32-bit integers.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vcvtps2dq))]
pub unsafe fn _mm256_cvtps_epi32(a: f32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_cvtps_epi32(::mem::transmute(a)))
}

/// Convert packed single-precision (32-bit) floating-point elements in `a`
/// to packed double-precision (64-bit) floating-point elements.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vcvtps2pd))]
pub unsafe fn _mm256_cvtps_pd(a: f32x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_cvtps_pd(::mem::transmute(a)))
}

/// Convert packed double-precision (64-bit) floating-point elements in `a`
/// to packed 32-bit integers with truncation.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vcvttpd2dq))]
pub unsafe fn _mm256_cvttpd_epi32(a: f64x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm256_cvttpd_epi32(::mem::transmute(a)))
}

/// Convert packed double-precision (64-bit) floating-point elements in `a`
/// to packed 32-bit integers.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vcvtpd2dq))]
pub unsafe fn _mm256_cvtpd_epi32(a: f64x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm256_cvtpd_epi32(::mem::transmute(a)))
}

/// Convert packed single-precision (32-bit) floating-point elements in `a`
/// to packed 32-bit integers with truncation.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vcvttps2dq))]
pub unsafe fn _mm256_cvttps_epi32(a: f32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_cvttps_epi32(::mem::transmute(a)))
}

/// Extract 128 bits (composed of 4 packed single-precision (32-bit)
/// floating-point elements) from `a`, selected with `imm8`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vextractf128, imm8 = 1))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_extractf128_ps(a: f32x8, imm8: i32) -> f32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_extractf128_ps(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Extract 128 bits (composed of 2 packed double-precision (64-bit)
/// floating-point elements) from `a`, selected with `imm8`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vextractf128, imm8 = 1))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_extractf128_pd(a: f64x4, imm8: i32) -> f64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_extractf128_pd(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Extract 128 bits (composed of integer data) from `a`, selected with `imm8`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vextractf128, imm8 = 1))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_extractf128_si256(a: __m256i, imm8: i32) -> __m128i {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_extractf128_si256(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shuffle single-precision (32-bit) floating-point elements in `a`
/// within 128-bit lanes using the control in `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vpermilps))]
pub unsafe fn _mm256_permutevar_ps(a: f32x8, b: __m256i) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_permutevar_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Shuffle single-precision (32-bit) floating-point elements in `a`
/// using the control in `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vpermilps))]
pub unsafe fn _mm_permutevar_ps(a: f32x4, b: __m128i) -> f32x4 {
    ::mem::transmute(::myarch::_mm_permutevar_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Shuffle single-precision (32-bit) floating-point elements in `a`
/// within 128-bit lanes using the control in `imm8`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vpermilps, imm8 = 9))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_permute_ps(a: f32x8, imm8: i32) -> f32x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_permute_ps(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shuffle single-precision (32-bit) floating-point elements in `a`
/// using the control in `imm8`.
#[inline]
#[target_feature(enable = "avx,sse")]
#[cfg_attr(test, assert_instr(vpermilps, imm8 = 9))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_permute_ps(a: f32x4, imm8: i32) -> f32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_permute_ps(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shuffle double-precision (64-bit) floating-point elements in `a`
/// within 256-bit lanes using the control in `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vpermilpd))]
pub unsafe fn _mm256_permutevar_pd(a: f64x4, b: __m256i) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_permutevar_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Shuffle double-precision (64-bit) floating-point elements in `a`
/// using the control in `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vpermilpd))]
pub unsafe fn _mm_permutevar_pd(a: f64x2, b: __m128i) -> f64x2 {
    ::mem::transmute(::myarch::_mm_permutevar_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Shuffle double-precision (64-bit) floating-point elements in `a`
/// within 128-bit lanes using the control in `imm8`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vpermilpd, imm8 = 0x1))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_permute_pd(a: f64x4, imm8: i32) -> f64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_permute_pd(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shuffle double-precision (64-bit) floating-point elements in `a`
/// using the control in `imm8`.
#[inline]
#[target_feature(enable = "avx,sse2")]
#[cfg_attr(test, assert_instr(vpermilpd, imm8 = 0x1))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_permute_pd(a: f64x2, imm8: i32) -> f64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_permute_pd(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shuffle 256-bits (composed of 8 packed single-precision (32-bit)
/// floating-point elements) selected by `imm8` from `a` and `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vperm2f128, imm8 = 0x5))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_permute2f128_ps(a: f32x8, b: f32x8, imm8: i32) -> f32x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_permute2f128_ps(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shuffle 256-bits (composed of 4 packed double-precision (64-bit)
/// floating-point elements) selected by `imm8` from `a` and `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vperm2f128, imm8 = 0x31))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_permute2f128_pd(a: f64x4, b: f64x4, imm8: i32) -> f64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_permute2f128_pd(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shuffle 258-bits (composed of integer data) selected by `imm8`
/// from `a` and `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vperm2f128, imm8 = 0x31))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_permute2f128_si256(a: __m256i, b: __m256i, imm8: i32) -> __m256i {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_permute2f128_si256(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Broadcast a single-precision (32-bit) floating-point element from memory
/// to all elements of the returned vector.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vbroadcastss))]
pub unsafe fn _mm256_broadcast_ss(f: &f32) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_broadcast_ss(::mem::transmute(f)))
}

/// Broadcast a single-precision (32-bit) floating-point element from memory
/// to all elements of the returned vector.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vbroadcastss))]
pub unsafe fn _mm_broadcast_ss(f: &f32) -> f32x4 {
    ::mem::transmute(::myarch::_mm_broadcast_ss(::mem::transmute(f)))
}

/// Broadcast a double-precision (64-bit) floating-point element from memory
/// to all elements of the returned vector.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vbroadcastsd))]
pub unsafe fn _mm256_broadcast_sd(f: &f64) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_broadcast_sd(::mem::transmute(f)))
}

/// Broadcast 128 bits from memory (composed of 4 packed single-precision
/// (32-bit) floating-point elements) to all elements of the returned vector.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vbroadcastf128))]
pub unsafe fn _mm256_broadcast_ps(a: &f32x4) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_broadcast_ps(::mem::transmute(a)))
}

/// Broadcast 128 bits from memory (composed of 2 packed double-precision
/// (64-bit) floating-point elements) to all elements of the returned vector.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vbroadcastf128))]
pub unsafe fn _mm256_broadcast_pd(a: &f64x2) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_broadcast_pd(::mem::transmute(a)))
}

/// Copy `a` to result, then insert 128 bits (composed of 4 packed
/// single-precision (32-bit) floating-point elements) from `b` into result
/// at the location specified by `imm8`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vinsertf128, imm8 = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_insertf128_ps(a: f32x8, b: f32x4, imm8: i32) -> f32x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_insertf128_ps(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Copy `a` to result, then insert 128 bits (composed of 2 packed
/// double-precision (64-bit) floating-point elements) from `b` into result
/// at the location specified by `imm8`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vinsertf128, imm8 = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_insertf128_pd(a: f64x4, b: f64x2, imm8: i32) -> f64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_insertf128_pd(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Copy `a` to result, then insert 128 bits from `b` into result
/// at the location specified by `imm8`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vinsertf128, imm8 = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_insertf128_si256(a: __m256i, b: __m128i, imm8: i32) -> __m256i {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_insertf128_si256(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Copy `a` to result, and insert the 8-bit integer `i` into result
/// at the location specified by `index`.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_insert_epi8(a: i8x32, i: i8, index: i32) -> i8x32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_insert_epi8(::mem::transmute(a), ::mem::transmute(i), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(index, call))
}

/// Copy `a` to result, and insert the 16-bit integer `i` into result
/// at the location specified by `index`.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_insert_epi16(a: i16x16, i: i16, index: i32) -> i16x16 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_insert_epi16(::mem::transmute(a), ::mem::transmute(i), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(index, call))
}

/// Copy `a` to result, and insert the 32-bit integer `i` into result
/// at the location specified by `index`.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_insert_epi32(a: i32x8, i: i32, index: i32) -> i32x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_insert_epi32(::mem::transmute(a), ::mem::transmute(i), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(index, call))
}

 // FIXME vmovapd expected
pub unsafe fn _mm256_load_pd(mem_addr: *const f64) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_load_pd(::mem::transmute(mem_addr)))
}

/// Load 256-bits (composed of 8 packed single-precision (32-bit)
/// floating-point elements) from memory into result.
/// `mem_addr` must be aligned on a 32-byte boundary or a
/// general-protection exception may be generated.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vmovaps))]
pub unsafe fn _mm256_load_ps(mem_addr: *const f32) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_load_ps(::mem::transmute(mem_addr)))
}

 // FIXME vmovupd expected
pub unsafe fn _mm256_loadu_pd(mem_addr: *const f64) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_loadu_pd(::mem::transmute(mem_addr)))
}

/// Load 256-bits (composed of 8 packed single-precision (32-bit)
/// floating-point elements) from memory into result.
/// `mem_addr` does not need to be aligned on any particular boundary.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vmovups))]
pub unsafe fn _mm256_loadu_ps(mem_addr: *const f32) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_loadu_ps(::mem::transmute(mem_addr)))
}

 // FIXME vmovdqa expected
pub unsafe fn _mm256_load_si256(mem_addr: *const __m256i) -> __m256i {
    ::mem::transmute(::myarch::_mm256_load_si256(::mem::transmute(mem_addr)))
}

 // FIXME vmovdqu expected
pub unsafe fn _mm256_loadu_si256(mem_addr: *const __m256i) -> __m256i {
    ::mem::transmute(::myarch::_mm256_loadu_si256(::mem::transmute(mem_addr)))
}

/// Load packed double-precision (64-bit) floating-point elements from memory
/// into result using `mask` (elements are zeroed out when the high bit of the
/// corresponding element is not set).
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vmaskmovpd))]
pub unsafe fn _mm256_maskload_pd(mem_addr: *const f64, mask: __m256i) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_maskload_pd(::mem::transmute(mem_addr), ::mem::transmute(mask)))
}

/// Load packed double-precision (64-bit) floating-point elements from memory
/// into result using `mask` (elements are zeroed out when the high bit of the
/// corresponding element is not set).
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vmaskmovpd))]
pub unsafe fn _mm_maskload_pd(mem_addr: *const f64, mask: __m128i) -> f64x2 {
    ::mem::transmute(::myarch::_mm_maskload_pd(::mem::transmute(mem_addr), ::mem::transmute(mask)))
}

/// Load packed single-precision (32-bit) floating-point elements from memory
/// into result using `mask` (elements are zeroed out when the high bit of the
/// corresponding element is not set).
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vmaskmovps))]
pub unsafe fn _mm256_maskload_ps(mem_addr: *const f32, mask: __m256i) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_maskload_ps(::mem::transmute(mem_addr), ::mem::transmute(mask)))
}

/// Load packed single-precision (32-bit) floating-point elements from memory
/// into result using `mask` (elements are zeroed out when the high bit of the
/// corresponding element is not set).
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vmaskmovps))]
pub unsafe fn _mm_maskload_ps(mem_addr: *const f32, mask: __m128i) -> f32x4 {
    ::mem::transmute(::myarch::_mm_maskload_ps(::mem::transmute(mem_addr), ::mem::transmute(mask)))
}

/// Duplicate odd-indexed single-precision (32-bit) floating-point elements
/// from `a`, and return the results.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vmovshdup))]
pub unsafe fn _mm256_movehdup_ps(a: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_movehdup_ps(::mem::transmute(a)))
}

/// Duplicate even-indexed single-precision (32-bit) floating-point elements
/// from `a`, and return the results.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vmovsldup))]
pub unsafe fn _mm256_moveldup_ps(a: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_moveldup_ps(::mem::transmute(a)))
}

/// Duplicate even-indexed double-precision (64-bit) floating-point elements
/// from "a", and return the results.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vmovddup))]
pub unsafe fn _mm256_movedup_pd(a: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_movedup_pd(::mem::transmute(a)))
}

/// Load 256-bits of integer data from unaligned memory into result.
/// This intrinsic may perform better than `_mm256_loadu_si256` when the
/// data crosses a cache line boundary.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vlddqu))]
pub unsafe fn _mm256_lddqu_si256(mem_addr: *const __m256i) -> __m256i {
    ::mem::transmute(::myarch::_mm256_lddqu_si256(::mem::transmute(mem_addr)))
}

/// Compute the approximate reciprocal of packed single-precision (32-bit)
/// floating-point elements in `a`, and return the results. The maximum
/// relative error for this approximation is less than 1.5*2^-12.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vrcpps))]
pub unsafe fn _mm256_rcp_ps(a: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_rcp_ps(::mem::transmute(a)))
}

/// Compute the approximate reciprocal square root of packed single-precision
/// (32-bit) floating-point elements in `a`, and return the results.
/// The maximum relative error for this approximation is less than 1.5*2^-12.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vrsqrtps))]
pub unsafe fn _mm256_rsqrt_ps(a: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_rsqrt_ps(::mem::transmute(a)))
}

/// Unpack and interleave double-precision (64-bit) floating-point elements
/// from the high half of each 128-bit lane in `a` and `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vunpckhpd))]
pub unsafe fn _mm256_unpackhi_pd(a: f64x4, b: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_unpackhi_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave single-precision (32-bit) floating-point elements
/// from the high half of each 128-bit lane in `a` and `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vunpckhps))]
pub unsafe fn _mm256_unpackhi_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_unpackhi_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave double-precision (64-bit) floating-point elements
/// from the low half of each 128-bit lane in `a` and `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vunpcklpd))]
pub unsafe fn _mm256_unpacklo_pd(a: f64x4, b: f64x4) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_unpacklo_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave single-precision (32-bit) floating-point elements
/// from the low half of each 128-bit lane in `a` and `b`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vunpcklps))]
pub unsafe fn _mm256_unpacklo_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_unpacklo_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of 256 bits (representing integer data) in `a` and
/// `b`, and set `ZF` to 1 if the result is zero, otherwise set `ZF` to 0.
/// Compute the bitwise NOT of `a` and then AND with `b`, and set `CF` to 1 if
/// the result is zero, otherwise set `CF` to 0. Return the `ZF` value.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vptest))]
pub unsafe fn _mm256_testz_si256(a: __m256i, b: __m256i) -> i32 {
    ::mem::transmute(::myarch::_mm256_testz_si256(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of 256 bits (representing integer data) in `a` and
/// `b`, and set `ZF` to 1 if the result is zero, otherwise set `ZF` to 0.
/// Compute the bitwise NOT of `a` and then AND with `b`, and set `CF` to 1 if
/// the result is zero, otherwise set `CF` to 0. Return the `CF` value.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vptest))]
pub unsafe fn _mm256_testc_si256(a: __m256i, b: __m256i) -> i32 {
    ::mem::transmute(::myarch::_mm256_testc_si256(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of 256 bits (representing integer data) in `a` and
/// `b`, and set `ZF` to 1 if the result is zero, otherwise set `ZF` to 0.
/// Compute the bitwise NOT of `a` and then AND with `b`, and set `CF` to 1 if
/// the result is zero, otherwise set `CF` to 0. Return 1 if both the `ZF` and
/// `CF` values are zero, otherwise return 0.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vptest))]
pub unsafe fn _mm256_testnzc_si256(a: __m256i, b: __m256i) -> i32 {
    ::mem::transmute(::myarch::_mm256_testnzc_si256(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of 256 bits (representing double-precision (64-bit)
/// floating-point elements) in `a` and `b`, producing an intermediate 256-bit
/// value, and set `ZF` to 1 if the sign bit of each 64-bit element in the
/// intermediate value is zero, otherwise set `ZF` to 0. Compute the bitwise
/// NOT of `a` and then AND with `b`, producing an intermediate value, and set
/// `CF` to 1 if the sign bit of each 64-bit element in the intermediate value
/// is zero, otherwise set `CF` to 0. Return the `ZF` value.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vtestpd))]
pub unsafe fn _mm256_testz_pd(a: f64x4, b: f64x4) -> i32 {
    ::mem::transmute(::myarch::_mm256_testz_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of 256 bits (representing double-precision (64-bit)
/// floating-point elements) in `a` and `b`, producing an intermediate 256-bit
/// value, and set `ZF` to 1 if the sign bit of each 64-bit element in the
/// intermediate value is zero, otherwise set `ZF` to 0. Compute the bitwise
/// NOT of `a` and then AND with `b`, producing an intermediate value, and set
/// `CF` to 1 if the sign bit of each 64-bit element in the intermediate value
/// is zero, otherwise set `CF` to 0. Return the `CF` value.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vtestpd))]
pub unsafe fn _mm256_testc_pd(a: f64x4, b: f64x4) -> i32 {
    ::mem::transmute(::myarch::_mm256_testc_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of 256 bits (representing double-precision (64-bit)
/// floating-point elements) in `a` and `b`, producing an intermediate 256-bit
/// value, and set `ZF` to 1 if the sign bit of each 64-bit element in the
/// intermediate value is zero, otherwise set `ZF` to 0. Compute the bitwise
/// NOT of `a` and then AND with `b`, producing an intermediate value, and set
/// `CF` to 1 if the sign bit of each 64-bit element in the intermediate value
/// is zero, otherwise set `CF` to 0. Return 1 if both the `ZF` and `CF` values
/// are zero, otherwise return 0.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vtestpd))]
pub unsafe fn _mm256_testnzc_pd(a: f64x4, b: f64x4) -> i32 {
    ::mem::transmute(::myarch::_mm256_testnzc_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of 128 bits (representing double-precision (64-bit)
/// floating-point elements) in `a` and `b`, producing an intermediate 128-bit
/// value, and set `ZF` to 1 if the sign bit of each 64-bit element in the
/// intermediate value is zero, otherwise set `ZF` to 0. Compute the bitwise
/// NOT of `a` and then AND with `b`, producing an intermediate value, and set
/// `CF` to 1 if the sign bit of each 64-bit element in the intermediate value
/// is zero, otherwise set `CF` to 0. Return the `ZF` value.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vtestpd))]
pub unsafe fn _mm_testz_pd(a: f64x2, b: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_testz_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of 128 bits (representing double-precision (64-bit)
/// floating-point elements) in `a` and `b`, producing an intermediate 128-bit
/// value, and set `ZF` to 1 if the sign bit of each 64-bit element in the
/// intermediate value is zero, otherwise set `ZF` to 0. Compute the bitwise
/// NOT of `a` and then AND with `b`, producing an intermediate value, and set
/// `CF` to 1 if the sign bit of each 64-bit element in the intermediate value
/// is zero, otherwise set `CF` to 0. Return the `CF` value.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vtestpd))]
pub unsafe fn _mm_testc_pd(a: f64x2, b: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_testc_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of 128 bits (representing double-precision (64-bit)
/// floating-point elements) in `a` and `b`, producing an intermediate 128-bit
/// value, and set `ZF` to 1 if the sign bit of each 64-bit element in the
/// intermediate value is zero, otherwise set `ZF` to 0. Compute the bitwise
/// NOT of `a` and then AND with `b`, producing an intermediate value, and set
/// `CF` to 1 if the sign bit of each 64-bit element in the intermediate value
/// is zero, otherwise set `CF` to 0. Return 1 if both the `ZF` and `CF` values
/// are zero, otherwise return 0.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vtestpd))]
pub unsafe fn _mm_testnzc_pd(a: f64x2, b: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_testnzc_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of 256 bits (representing single-precision (32-bit)
/// floating-point elements) in `a` and `b`, producing an intermediate 256-bit
/// value, and set `ZF` to 1 if the sign bit of each 32-bit element in the
/// intermediate value is zero, otherwise set `ZF` to 0. Compute the bitwise
/// NOT of `a` and then AND with `b`, producing an intermediate value, and set
/// `CF` to 1 if the sign bit of each 32-bit element in the intermediate value
/// is zero, otherwise set `CF` to 0. Return the `ZF` value.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vtestps))]
pub unsafe fn _mm256_testz_ps(a: f32x8, b: f32x8) -> i32 {
    ::mem::transmute(::myarch::_mm256_testz_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of 256 bits (representing single-precision (32-bit)
/// floating-point elements) in `a` and `b`, producing an intermediate 256-bit
/// value, and set `ZF` to 1 if the sign bit of each 32-bit element in the
/// intermediate value is zero, otherwise set `ZF` to 0. Compute the bitwise
/// NOT of `a` and then AND with `b`, producing an intermediate value, and set
/// `CF` to 1 if the sign bit of each 32-bit element in the intermediate value
/// is zero, otherwise set `CF` to 0. Return the `CF` value.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vtestps))]
pub unsafe fn _mm256_testc_ps(a: f32x8, b: f32x8) -> i32 {
    ::mem::transmute(::myarch::_mm256_testc_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of 256 bits (representing single-precision (32-bit)
/// floating-point elements) in `a` and `b`, producing an intermediate 256-bit
/// value, and set `ZF` to 1 if the sign bit of each 32-bit element in the
/// intermediate value is zero, otherwise set `ZF` to 0. Compute the bitwise
/// NOT of `a` and then AND with `b`, producing an intermediate value, and set
/// `CF` to 1 if the sign bit of each 32-bit element in the intermediate value
/// is zero, otherwise set `CF` to 0. Return 1 if both the `ZF` and `CF` values
/// are zero, otherwise return 0.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vtestps))]
pub unsafe fn _mm256_testnzc_ps(a: f32x8, b: f32x8) -> i32 {
    ::mem::transmute(::myarch::_mm256_testnzc_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of 128 bits (representing single-precision (32-bit)
/// floating-point elements) in `a` and `b`, producing an intermediate 128-bit
/// value, and set `ZF` to 1 if the sign bit of each 32-bit element in the
/// intermediate value is zero, otherwise set `ZF` to 0. Compute the bitwise
/// NOT of `a` and then AND with `b`, producing an intermediate value, and set
/// `CF` to 1 if the sign bit of each 32-bit element in the intermediate value
/// is zero, otherwise set `CF` to 0. Return the `ZF` value.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vtestps))]
pub unsafe fn _mm_testz_ps(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_testz_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of 128 bits (representing single-precision (32-bit)
/// floating-point elements) in `a` and `b`, producing an intermediate 128-bit
/// value, and set `ZF` to 1 if the sign bit of each 32-bit element in the
/// intermediate value is zero, otherwise set `ZF` to 0. Compute the bitwise
/// NOT of `a` and then AND with `b`, producing an intermediate value, and set
/// `CF` to 1 if the sign bit of each 32-bit element in the intermediate value
/// is zero, otherwise set `CF` to 0. Return the `CF` value.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vtestps))]
pub unsafe fn _mm_testc_ps(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_testc_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of 128 bits (representing single-precision (32-bit)
/// floating-point elements) in `a` and `b`, producing an intermediate 128-bit
/// value, and set `ZF` to 1 if the sign bit of each 32-bit element in the
/// intermediate value is zero, otherwise set `ZF` to 0. Compute the bitwise
/// NOT of `a` and then AND with `b`, producing an intermediate value, and set
/// `CF` to 1 if the sign bit of each 32-bit element in the intermediate value
/// is zero, otherwise set `CF` to 0. Return 1 if both the `ZF` and `CF` values
/// are zero, otherwise return 0.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vtestps))]
pub unsafe fn _mm_testnzc_ps(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::myarch::_mm_testnzc_ps(::mem::transmute(a), ::mem::transmute(b)))
}

/// Set each bit of the returned mask based on the most significant bit of the
/// corresponding packed double-precision (64-bit) floating-point element in
/// `a`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vmovmskpd))]
pub unsafe fn _mm256_movemask_pd(a: f64x4) -> i32 {
    ::mem::transmute(::myarch::_mm256_movemask_pd(::mem::transmute(a)))
}

/// Set each bit of the returned mask based on the most significant bit of the
/// corresponding packed single-precision (32-bit) floating-point element in
/// `a`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vmovmskps))]
pub unsafe fn _mm256_movemask_ps(a: f32x8) -> i32 {
    ::mem::transmute(::myarch::_mm256_movemask_ps(::mem::transmute(a)))
}

 // FIXME vxorpd expected
pub unsafe fn _mm256_setzero_pd() -> f64x4 {
    ::mem::transmute(::myarch::_mm256_setzero_pd())
}

/// Return vector of type __m256 with all elements set to zero.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vxorps))]
pub unsafe fn _mm256_setzero_ps() -> f32x8 {
    ::mem::transmute(::myarch::_mm256_setzero_ps())
}

/// Return vector of type __m256i with all elements set to zero.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vxor))]
pub unsafe fn _mm256_setzero_si256() -> __m256i {
    ::mem::transmute(::myarch::_mm256_setzero_si256())
}

/// Set packed double-precision (64-bit) floating-point elements in returned
/// vector with the supplied values.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
#[cfg_attr(test, assert_instr(vinsertf128))]
pub unsafe fn _mm256_set_pd(a: f64, b: f64, c: f64, d: f64) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_set_pd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c), ::mem::transmute(d)))
}

/// Set packed single-precision (32-bit) floating-point elements in returned
/// vector with the supplied values.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_set_ps(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_set_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c), ::mem::transmute(d), ::mem::transmute(e), ::mem::transmute(f), ::mem::transmute(g), ::mem::transmute(h)))
}

/// Set packed 8-bit integers in returned vector with the supplied values in
/// reverse order.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_set_epi8(e00: i8, e01: i8, e02: i8, e03: i8, e04: i8, e05: i8, e06: i8, e07: i8, e08: i8, e09: i8, e10: i8, e11: i8, e12: i8, e13: i8, e14: i8, e15: i8, e16: i8, e17: i8, e18: i8, e19: i8, e20: i8, e21: i8, e22: i8, e23: i8, e24: i8, e25: i8, e26: i8, e27: i8, e28: i8, e29: i8, e30: i8, e31: i8) -> i8x32 {
    ::mem::transmute(::myarch::_mm256_set_epi8(::mem::transmute(e00), ::mem::transmute(e01), ::mem::transmute(e02), ::mem::transmute(e03), ::mem::transmute(e04), ::mem::transmute(e05), ::mem::transmute(e06), ::mem::transmute(e07), ::mem::transmute(e08), ::mem::transmute(e09), ::mem::transmute(e10), ::mem::transmute(e11), ::mem::transmute(e12), ::mem::transmute(e13), ::mem::transmute(e14), ::mem::transmute(e15), ::mem::transmute(e16), ::mem::transmute(e17), ::mem::transmute(e18), ::mem::transmute(e19), ::mem::transmute(e20), ::mem::transmute(e21), ::mem::transmute(e22), ::mem::transmute(e23), ::mem::transmute(e24), ::mem::transmute(e25), ::mem::transmute(e26), ::mem::transmute(e27), ::mem::transmute(e28), ::mem::transmute(e29), ::mem::transmute(e30), ::mem::transmute(e31)))
}

/// Set packed 16-bit integers in returned vector with the supplied values.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_set_epi16(e00: i16, e01: i16, e02: i16, e03: i16, e04: i16, e05: i16, e06: i16, e07: i16, e08: i16, e09: i16, e10: i16, e11: i16, e12: i16, e13: i16, e14: i16, e15: i16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_set_epi16(::mem::transmute(e00), ::mem::transmute(e01), ::mem::transmute(e02), ::mem::transmute(e03), ::mem::transmute(e04), ::mem::transmute(e05), ::mem::transmute(e06), ::mem::transmute(e07), ::mem::transmute(e08), ::mem::transmute(e09), ::mem::transmute(e10), ::mem::transmute(e11), ::mem::transmute(e12), ::mem::transmute(e13), ::mem::transmute(e14), ::mem::transmute(e15)))
}

/// Set packed 32-bit integers in returned vector with the supplied values.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_set_epi32(e0: i32, e1: i32, e2: i32, e3: i32, e4: i32, e5: i32, e6: i32, e7: i32) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_set_epi32(::mem::transmute(e0), ::mem::transmute(e1), ::mem::transmute(e2), ::mem::transmute(e3), ::mem::transmute(e4), ::mem::transmute(e5), ::mem::transmute(e6), ::mem::transmute(e7)))
}

/// Set packed 64-bit integers in returned vector with the supplied values.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_set_epi64x(a: i64, b: i64, c: i64, d: i64) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_set_epi64x(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c), ::mem::transmute(d)))
}

/// Set packed double-precision (64-bit) floating-point elements in returned
/// vector with the supplied values in reverse order.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_setr_pd(a: f64, b: f64, c: f64, d: f64) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_setr_pd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c), ::mem::transmute(d)))
}

/// Set packed single-precision (32-bit) floating-point elements in returned
/// vector with the supplied values in reverse order.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_setr_ps(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_setr_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c), ::mem::transmute(d), ::mem::transmute(e), ::mem::transmute(f), ::mem::transmute(g), ::mem::transmute(h)))
}

/// Set packed 8-bit integers in returned vector with the supplied values in
/// reverse order.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_setr_epi8(e00: i8, e01: i8, e02: i8, e03: i8, e04: i8, e05: i8, e06: i8, e07: i8, e08: i8, e09: i8, e10: i8, e11: i8, e12: i8, e13: i8, e14: i8, e15: i8, e16: i8, e17: i8, e18: i8, e19: i8, e20: i8, e21: i8, e22: i8, e23: i8, e24: i8, e25: i8, e26: i8, e27: i8, e28: i8, e29: i8, e30: i8, e31: i8) -> i8x32 {
    ::mem::transmute(::myarch::_mm256_setr_epi8(::mem::transmute(e00), ::mem::transmute(e01), ::mem::transmute(e02), ::mem::transmute(e03), ::mem::transmute(e04), ::mem::transmute(e05), ::mem::transmute(e06), ::mem::transmute(e07), ::mem::transmute(e08), ::mem::transmute(e09), ::mem::transmute(e10), ::mem::transmute(e11), ::mem::transmute(e12), ::mem::transmute(e13), ::mem::transmute(e14), ::mem::transmute(e15), ::mem::transmute(e16), ::mem::transmute(e17), ::mem::transmute(e18), ::mem::transmute(e19), ::mem::transmute(e20), ::mem::transmute(e21), ::mem::transmute(e22), ::mem::transmute(e23), ::mem::transmute(e24), ::mem::transmute(e25), ::mem::transmute(e26), ::mem::transmute(e27), ::mem::transmute(e28), ::mem::transmute(e29), ::mem::transmute(e30), ::mem::transmute(e31)))
}

/// Set packed 16-bit integers in returned vector with the supplied values in
/// reverse order.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_setr_epi16(e00: i16, e01: i16, e02: i16, e03: i16, e04: i16, e05: i16, e06: i16, e07: i16, e08: i16, e09: i16, e10: i16, e11: i16, e12: i16, e13: i16, e14: i16, e15: i16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_setr_epi16(::mem::transmute(e00), ::mem::transmute(e01), ::mem::transmute(e02), ::mem::transmute(e03), ::mem::transmute(e04), ::mem::transmute(e05), ::mem::transmute(e06), ::mem::transmute(e07), ::mem::transmute(e08), ::mem::transmute(e09), ::mem::transmute(e10), ::mem::transmute(e11), ::mem::transmute(e12), ::mem::transmute(e13), ::mem::transmute(e14), ::mem::transmute(e15)))
}

/// Set packed 32-bit integers in returned vector with the supplied values in
/// reverse order.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_setr_epi32(e0: i32, e1: i32, e2: i32, e3: i32, e4: i32, e5: i32, e6: i32, e7: i32) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_setr_epi32(::mem::transmute(e0), ::mem::transmute(e1), ::mem::transmute(e2), ::mem::transmute(e3), ::mem::transmute(e4), ::mem::transmute(e5), ::mem::transmute(e6), ::mem::transmute(e7)))
}

/// Set packed 64-bit integers in returned vector with the supplied values in
/// reverse order.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_setr_epi64x(a: i64, b: i64, c: i64, d: i64) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_setr_epi64x(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c), ::mem::transmute(d)))
}

/// Broadcast double-precision (64-bit) floating-point value `a` to all
/// elements of returned vector.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_set1_pd(a: f64) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_set1_pd(::mem::transmute(a)))
}

/// Broadcast single-precision (32-bit) floating-point value `a` to all
/// elements of returned vector.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_set1_ps(a: f32) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_set1_ps(::mem::transmute(a)))
}

/// Broadcast 8-bit integer `a` to all elements of returned vector.
/// This intrinsic may generate the `vpbroadcastb`.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vpshufb))]
#[cfg_attr(test, assert_instr(vinsertf128))]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_set1_epi8(a: i8) -> i8x32 {
    ::mem::transmute(::myarch::_mm256_set1_epi8(::mem::transmute(a)))
}

/// Broadcast 16-bit integer `a` to all all elements of returned vector.
/// This intrinsic may generate the `vpbroadcastw`.
#[inline]
#[target_feature(enable = "avx")]
//#[cfg_attr(test, assert_instr(vpshufb))]
#[cfg_attr(test, assert_instr(vinsertf128))]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_set1_epi16(a: i16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_set1_epi16(::mem::transmute(a)))
}

/// Broadcast 32-bit integer `a` to all elements of returned vector.
/// This intrinsic may generate the `vpbroadcastd`.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_set1_epi32(a: i32) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_set1_epi32(::mem::transmute(a)))
}

/// Broadcast 64-bit integer `a` to all elements of returned vector.
/// This intrinsic may generate the `vpbroadcastq`.
#[inline]
#[target_feature(enable = "avx")]
//#[cfg_attr(test, assert_instr(vmovddup))]
#[cfg_attr(test, assert_instr(vinsertf128))]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_set1_epi64x(a: i64) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_set1_epi64x(::mem::transmute(a)))
}

/// Cast vector of type __m256d to type __m256.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic is only used for compilation and does not generate any
// instructions, thus it has zero latency.
pub unsafe fn _mm256_castpd_ps(a: f64x4) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_castpd_ps(::mem::transmute(a)))
}

/// Cast vector of type __m256 to type __m256d.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic is only used for compilation and does not generate any
// instructions, thus it has zero latency.
pub unsafe fn _mm256_castps_pd(a: f32x8) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_castps_pd(::mem::transmute(a)))
}

/// Casts vector of type __m256 to type __m256i.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic is only used for compilation and does not generate any
// instructions, thus it has zero latency.
pub unsafe fn _mm256_castps_si256(a: f32x8) -> __m256i {
    ::mem::transmute(::myarch::_mm256_castps_si256(::mem::transmute(a)))
}

/// Casts vector of type __m256i to type __m256.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic is only used for compilation and does not generate any
// instructions, thus it has zero latency.
pub unsafe fn _mm256_castsi256_ps(a: __m256i) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_castsi256_ps(::mem::transmute(a)))
}

/// Casts vector of type __m256d to type __m256i.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic is only used for compilation and does not generate any
// instructions, thus it has zero latency.
pub unsafe fn _mm256_castpd_si256(a: f64x4) -> __m256i {
    ::mem::transmute(::myarch::_mm256_castpd_si256(::mem::transmute(a)))
}

/// Casts vector of type __m256i to type __m256d.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic is only used for compilation and does not generate any
// instructions, thus it has zero latency.
pub unsafe fn _mm256_castsi256_pd(a: __m256i) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_castsi256_pd(::mem::transmute(a)))
}

/// Casts vector of type __m256 to type __m128.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic is only used for compilation and does not generate any
// instructions, thus it has zero latency.
pub unsafe fn _mm256_castps256_ps128(a: f32x8) -> f32x4 {
    ::mem::transmute(::myarch::_mm256_castps256_ps128(::mem::transmute(a)))
}

/// Casts vector of type __m256d to type __m128d.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic is only used for compilation and does not generate any
// instructions, thus it has zero latency.
pub unsafe fn _mm256_castpd256_pd128(a: f64x4) -> f64x2 {
    ::mem::transmute(::myarch::_mm256_castpd256_pd128(::mem::transmute(a)))
}

/// Casts vector of type __m256i to type __m128i.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic is only used for compilation and does not generate any
// instructions, thus it has zero latency.
pub unsafe fn _mm256_castsi256_si128(a: __m256i) -> __m128i {
    ::mem::transmute(::myarch::_mm256_castsi256_si128(::mem::transmute(a)))
}

/// Casts vector of type __m128 to type __m256;
/// the upper 128 bits of the result are undefined.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic is only used for compilation and does not generate any
// instructions, thus it has zero latency.
pub unsafe fn _mm256_castps128_ps256(a: f32x4) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_castps128_ps256(::mem::transmute(a)))
}

/// Casts vector of type __m128d to type __m256d;
/// the upper 128 bits of the result are undefined.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic is only used for compilation and does not generate any
// instructions, thus it has zero latency.
pub unsafe fn _mm256_castpd128_pd256(a: f64x2) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_castpd128_pd256(::mem::transmute(a)))
}

/// Casts vector of type __m128i to type __m256i;
/// the upper 128 bits of the result are undefined.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic is only used for compilation and does not generate any
// instructions, thus it has zero latency.
pub unsafe fn _mm256_castsi128_si256(a: __m128i) -> __m256i {
    ::mem::transmute(::myarch::_mm256_castsi128_si256(::mem::transmute(a)))
}

/// Constructs a 256-bit floating-point vector of [8 x float] from a
/// 128-bit floating-point vector of [4 x float]. The lower 128 bits contain
/// the value of the source vector. The upper 128 bits are set to zero.
#[inline]
#[target_feature(enable = "avx,sse")]
// This intrinsic is only used for compilation and does not generate any
// instructions, thus it has zero latency.
pub unsafe fn _mm256_zextps128_ps256(a: f32x4) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_zextps128_ps256(::mem::transmute(a)))
}

/// Constructs a 256-bit integer vector from a 128-bit integer vector.
/// The lower 128 bits contain the value of the source vector. The upper
/// 128 bits are set to zero.
#[inline]
#[target_feature(enable = "avx,sse2")]
// This intrinsic is only used for compilation and does not generate any
// instructions, thus it has zero latency.
pub unsafe fn _mm256_zextsi128_si256(a: __m128i) -> __m256i {
    ::mem::transmute(::myarch::_mm256_zextsi128_si256(::mem::transmute(a)))
}

/// Constructs a 256-bit floating-point vector of [4 x double] from a
/// 128-bit floating-point vector of [2 x double]. The lower 128 bits
/// contain the value of the source vector. The upper 128 bits are set
/// to zero.
#[inline]
#[target_feature(enable = "avx,sse2")]
// This intrinsic is only used for compilation and does not generate any
// instructions, thus it has zero latency.
pub unsafe fn _mm256_zextpd128_pd256(a: f64x2) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_zextpd128_pd256(::mem::transmute(a)))
}

/// Return vector of type `__m256` with undefined elements.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_undefined_ps() -> f32x8 {
    ::mem::transmute(::myarch::_mm256_undefined_ps())
}

/// Return vector of type `__m256d` with undefined elements.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_undefined_pd() -> f64x4 {
    ::mem::transmute(::myarch::_mm256_undefined_pd())
}

/// Return vector of type __m256i with undefined elements.
#[inline]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_undefined_si256() -> __m256i {
    ::mem::transmute(::myarch::_mm256_undefined_si256())
}

/// Set packed __m256 returned vector with the supplied values.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vinsertf128))]
pub unsafe fn _mm256_set_m128(hi: f32x4, lo: f32x4) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_set_m128(::mem::transmute(hi), ::mem::transmute(lo)))
}

/// Set packed __m256d returned vector with the supplied values.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vinsertf128))]
pub unsafe fn _mm256_set_m128d(hi: f64x2, lo: f64x2) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_set_m128d(::mem::transmute(hi), ::mem::transmute(lo)))
}

/// Set packed __m256i returned vector with the supplied values.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vinsertf128))]
pub unsafe fn _mm256_set_m128i(hi: __m128i, lo: __m128i) -> __m256i {
    ::mem::transmute(::myarch::_mm256_set_m128i(::mem::transmute(hi), ::mem::transmute(lo)))
}

/// Set packed __m256 returned vector with the supplied values.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vinsertf128))]
pub unsafe fn _mm256_setr_m128(lo: f32x4, hi: f32x4) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_setr_m128(::mem::transmute(lo), ::mem::transmute(hi)))
}

/// Set packed __m256d returned vector with the supplied values.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vinsertf128))]
pub unsafe fn _mm256_setr_m128d(lo: f64x2, hi: f64x2) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_setr_m128d(::mem::transmute(lo), ::mem::transmute(hi)))
}

/// Set packed __m256i returned vector with the supplied values.
#[inline]
#[target_feature(enable = "avx")]
#[cfg_attr(test, assert_instr(vinsertf128))]
pub unsafe fn _mm256_setr_m128i(lo: __m128i, hi: __m128i) -> __m256i {
    ::mem::transmute(::myarch::_mm256_setr_m128i(::mem::transmute(lo), ::mem::transmute(hi)))
}

/// Load two 128-bit values (composed of 4 packed single-precision (32-bit)
/// floating-point elements) from memory, and combine them into a 256-bit
/// value.
/// `hiaddr` and `loaddr` do not need to be aligned on any particular boundary.
#[inline]
#[target_feature(enable = "avx,sse")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_loadu2_m128(hiaddr: *const f32, loaddr: *const f32) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_loadu2_m128(::mem::transmute(hiaddr), ::mem::transmute(loaddr)))
}

/// Load two 128-bit values (composed of 2 packed double-precision (64-bit)
/// floating-point elements) from memory, and combine them into a 256-bit
/// value.
/// `hiaddr` and `loaddr` do not need to be aligned on any particular boundary.
#[inline]
#[target_feature(enable = "avx,sse2")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_loadu2_m128d(hiaddr: *const f64, loaddr: *const f64) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_loadu2_m128d(::mem::transmute(hiaddr), ::mem::transmute(loaddr)))
}

/// Load two 128-bit values (composed of integer data) from memory, and combine
/// them into a 256-bit value.
/// `hiaddr` and `loaddr` do not need to be aligned on any particular boundary.
#[inline]
#[target_feature(enable = "avx,sse2")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_loadu2_m128i(hiaddr: *const __m128i, loaddr: *const __m128i) -> __m256i {
    ::mem::transmute(::myarch::_mm256_loadu2_m128i(::mem::transmute(hiaddr), ::mem::transmute(loaddr)))
}

/// Returns the first element of the input vector of [8 x float].
#[inline]
#[target_feature(enable = "avx")]
//#[cfg_attr(test, assert_instr(movss))] FIXME
pub unsafe fn _mm256_cvtss_f32(a: f32x8) -> f32 {
    ::mem::transmute(::myarch::_mm256_cvtss_f32(::mem::transmute(a)))
}

