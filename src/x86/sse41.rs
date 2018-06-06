#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Blend packed 8-bit integers from `a` and `b` using `mask`
///
/// The high bit of each corresponding mask byte determines the selection.
/// If the high bit is set the element of `a` is selected. The element
/// of `b` is selected otherwise.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pblendvb))]
pub unsafe fn _mm_blendv_epi8(a: u8x16, b: u8x16, mask: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_blendv_epi8(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(mask)))
}

/// Blend packed 16-bit integers from `a` and `b` using the mask `imm8`.
///
/// The mask bits determine the selection. A clear bit selects the
/// corresponding element of `a`, and a set bit the corresponding
/// element of `b`.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pblendw, imm8 = 0xF0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_blend_epi16(a: i16x8, b: i16x8, imm8: i32) -> i16x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_blend_epi16(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Blend packed double-precision (64-bit) floating-point elements from `a`
/// and `b` using `mask`
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(blendvpd))]
pub unsafe fn _mm_blendv_pd(a: f64x2, b: f64x2, mask: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_blendv_pd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(mask)))
}

/// Blend packed single-precision (32-bit) floating-point elements from `a`
/// and `b` using `mask`
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(blendvps))]
pub unsafe fn _mm_blendv_ps(a: f32x4, b: f32x4, mask: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_blendv_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(mask)))
}

/// Blend packed double-precision (64-bit) floating-point elements from `a`
/// and `b` using control mask `imm2`
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(blendpd, imm2 = 0b10))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_blend_pd(a: f64x2, b: f64x2, imm2: i32) -> f64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_blend_pd(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm2, call))
}

/// Blend packed single-precision (32-bit) floating-point elements from `a`
/// and `b` using mask `imm4`
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(blendps, imm4 = 0b0101))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_blend_ps(a: f32x4, b: f32x4, imm4: i32) -> f32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_blend_ps(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm4, call))
}

/// Extract a single-precision (32-bit) floating-point element from `a`,
/// selected with `imm8`
#[inline]
#[target_feature(enable = "sse4.1")]
// TODO: Add test for Windows
#[cfg_attr(test, assert_instr(extractps, imm8 = 0))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_extract_ps(a: f32x4, imm8: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_extract_ps(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Extract an 8-bit integer from `a`, selected with `imm8`. Returns a 32-bit
/// integer containing the zero-extended integer data.
///
/// See [LLVM commit D20468][https://reviews.llvm.org/D20468].
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pextrb, imm8 = 0))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_extract_epi8(a: u8x16, imm8: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_extract_epi8(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Extract an 32-bit integer from `a` selected with `imm8`
#[inline]
#[target_feature(enable = "sse4.1")]
// TODO: Add test for Windows
#[cfg_attr(test, assert_instr(extractps, imm8 = 1))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_extract_epi32(a: i32x4, imm8: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_extract_epi32(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Select a single value in `a` to store at some position in `b`,
/// Then zero elements according to `imm8`.
///
/// `imm8` specifies which bits from operand `a` will be copied, which bits in
/// the result they will be copied to, and which bits in the result will be
/// cleared. The following assignments are made:
///
/// * Bits `[7:6]` specify the bits to copy from operand `a`:
///     - `00`: Selects bits `[31:0]` from operand `a`.
///     - `01`: Selects bits `[63:32]` from operand `a`.
///     - `10`: Selects bits `[95:64]` from operand `a`.
///     - `11`: Selects bits `[127:96]` from operand `a`.
///
/// * Bits `[5:4]` specify the bits in the result to which the selected bits
/// from operand `a` are copied:
///     - `00`: Copies the selected bits from `a` to result bits `[31:0]`.
///     - `01`: Copies the selected bits from `a` to result bits `[63:32]`.
///     - `10`: Copies the selected bits from `a` to result bits `[95:64]`.
///     - `11`: Copies the selected bits from `a` to result bits `[127:96]`.
///
/// * Bits `[3:0]`: If any of these bits are set, the corresponding result
/// element is cleared.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(insertps, imm8 = 0b1010))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_insert_ps(a: f32x4, b: f32x4, imm8: i32) -> f32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_insert_ps(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Return a copy of `a` with the 8-bit integer from `i` inserted at a
/// location specified by `imm8`.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pinsrb, imm8 = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_insert_epi8(a: u8x16, i: i32, imm8: i32) -> u8x16 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_insert_epi8(::mem::transmute(a), ::mem::transmute(i), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Return a copy of `a` with the 32-bit integer from `i` inserted at a
/// location specified by `imm8`.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pinsrd, imm8 = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_insert_epi32(a: i32x4, i: i32, imm8: i32) -> i32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_insert_epi32(::mem::transmute(a), ::mem::transmute(i), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Compare packed 8-bit integers in `a` and `b` and return packed maximum
/// values in dst.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmaxsb))]
pub unsafe fn _mm_max_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_max_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed unsigned 16-bit integers in `a` and `b`, and return packed
/// maximum.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmaxuw))]
pub unsafe fn _mm_max_epu16(a: u16x8, b: u16x8) -> u16x8 {
    ::mem::transmute(::myarch::_mm_max_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 32-bit integers in `a` and `b`, and return packed maximum
/// values.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmaxsd))]
pub unsafe fn _mm_max_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_max_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed unsigned 32-bit integers in `a` and `b`, and return packed
/// maximum values.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmaxud))]
pub unsafe fn _mm_max_epu32(a: u32x4, b: u32x4) -> u32x4 {
    ::mem::transmute(::myarch::_mm_max_epu32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 8-bit integers in `a` and `b` and return packed minimum
/// values in dst.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pminsb))]
pub unsafe fn _mm_min_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_min_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed unsigned 16-bit integers in `a` and `b`, and return packed
/// minimum.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pminuw))]
pub unsafe fn _mm_min_epu16(a: u16x8, b: u16x8) -> u16x8 {
    ::mem::transmute(::myarch::_mm_min_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 32-bit integers in `a` and `b`, and return packed minimum
/// values.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pminsd))]
pub unsafe fn _mm_min_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_min_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed unsigned 32-bit integers in `a` and `b`, and return packed
/// minimum values.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pminud))]
pub unsafe fn _mm_min_epu32(a: u32x4, b: u32x4) -> u32x4 {
    ::mem::transmute(::myarch::_mm_min_epu32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Convert packed 32-bit integers from `a` and `b` to packed 16-bit integers
/// using unsigned saturation
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(packusdw))]
pub unsafe fn _mm_packus_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_packus_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 64-bit integers in `a` and `b` for equality
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pcmpeqq))]
pub unsafe fn _mm_cmpeq_epi64(a: i64x2, b: i64x2) -> i64x2 {
    ::mem::transmute(::myarch::_mm_cmpeq_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

/// Sign extend packed 8-bit integers in `a` to packed 16-bit integers
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmovsxbw))]
pub unsafe fn _mm_cvtepi8_epi16(a: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_cvtepi8_epi16(::mem::transmute(a)))
}

/// Sign extend packed 8-bit integers in `a` to packed 32-bit integers
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmovsxbd))]
pub unsafe fn _mm_cvtepi8_epi32(a: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_cvtepi8_epi32(::mem::transmute(a)))
}

/// Sign extend packed 8-bit integers in the low 8 bytes of `a` to packed
/// 64-bit integers
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmovsxbq))]
pub unsafe fn _mm_cvtepi8_epi64(a: i64x2) -> i64x2 {
    ::mem::transmute(::myarch::_mm_cvtepi8_epi64(::mem::transmute(a)))
}

/// Sign extend packed 16-bit integers in `a` to packed 32-bit integers
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmovsxwd))]
pub unsafe fn _mm_cvtepi16_epi32(a: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_cvtepi16_epi32(::mem::transmute(a)))
}

/// Sign extend packed 16-bit integers in `a` to packed 64-bit integers
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmovsxwq))]
pub unsafe fn _mm_cvtepi16_epi64(a: i64x2) -> i64x2 {
    ::mem::transmute(::myarch::_mm_cvtepi16_epi64(::mem::transmute(a)))
}

/// Sign extend packed 32-bit integers in `a` to packed 64-bit integers
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmovsxdq))]
pub unsafe fn _mm_cvtepi32_epi64(a: i64x2) -> i64x2 {
    ::mem::transmute(::myarch::_mm_cvtepi32_epi64(::mem::transmute(a)))
}

/// Zero extend packed unsigned 8-bit integers in `a` to packed 16-bit integers
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmovzxbw))]
pub unsafe fn _mm_cvtepu8_epi16(a: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_cvtepu8_epi16(::mem::transmute(a)))
}

/// Zero extend packed unsigned 8-bit integers in `a` to packed 32-bit integers
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmovzxbd))]
pub unsafe fn _mm_cvtepu8_epi32(a: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_cvtepu8_epi32(::mem::transmute(a)))
}

/// Zero extend packed unsigned 8-bit integers in `a` to packed 64-bit integers
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmovzxbq))]
pub unsafe fn _mm_cvtepu8_epi64(a: i64x2) -> i64x2 {
    ::mem::transmute(::myarch::_mm_cvtepu8_epi64(::mem::transmute(a)))
}

/// Zero extend packed unsigned 16-bit integers in `a`
/// to packed 32-bit integers
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmovzxwd))]
pub unsafe fn _mm_cvtepu16_epi32(a: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_cvtepu16_epi32(::mem::transmute(a)))
}

/// Zero extend packed unsigned 16-bit integers in `a`
/// to packed 64-bit integers
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmovzxwq))]
pub unsafe fn _mm_cvtepu16_epi64(a: i64x2) -> i64x2 {
    ::mem::transmute(::myarch::_mm_cvtepu16_epi64(::mem::transmute(a)))
}

/// Zero extend packed unsigned 32-bit integers in `a`
/// to packed 64-bit integers
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmovzxdq))]
pub unsafe fn _mm_cvtepu32_epi64(a: i64x2) -> i64x2 {
    ::mem::transmute(::myarch::_mm_cvtepu32_epi64(::mem::transmute(a)))
}

/// Returns the dot product of two __m128d vectors.
///
/// `imm8[1:0]` is the broadcast mask, and `imm8[5:4]` is the condition mask.
/// If a condition mask bit is zero, the corresponding multiplication is
/// replaced by a value of `0.0`. If a broadcast mask bit is one, the result of
/// the dot product will be stored in the return value component. Otherwise if
/// the broadcast mask bit is zero then the return component will be zero.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(dppd, imm8 = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_dp_pd(a: f64x2, b: f64x2, imm8: i32) -> f64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_dp_pd(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Returns the dot product of two __m128 vectors.
///
/// `imm8[3:0]` is the broadcast mask, and `imm8[7:4]` is the condition mask.
/// If a condition mask bit is zero, the corresponding multiplication is
/// replaced by a value of `0.0`. If a broadcast mask bit is one, the result of
/// the dot product will be stored in the return value component. Otherwise if
/// the broadcast mask bit is zero then the return component will be zero.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(dpps, imm8 = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_dp_ps(a: f32x4, b: f32x4, imm8: i32) -> f32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_dp_ps(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Round the packed double-precision (64-bit) floating-point elements in `a`
/// down to an integer value, and store the results as packed double-precision
/// floating-point elements.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(roundpd))]
pub unsafe fn _mm_floor_pd(a: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_floor_pd(::mem::transmute(a)))
}

/// Round the packed single-precision (32-bit) floating-point elements in `a`
/// down to an integer value, and store the results as packed single-precision
/// floating-point elements.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(roundps))]
pub unsafe fn _mm_floor_ps(a: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_floor_ps(::mem::transmute(a)))
}

/// Round the lower double-precision (64-bit) floating-point element in `b`
/// down to an integer value, store the result as a double-precision
/// floating-point element in the lower element of the intrinsic result,
/// and copy the upper element from `a` to the upper element of the intrinsic
/// result.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(roundsd))]
pub unsafe fn _mm_floor_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_floor_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Round the lower single-precision (32-bit) floating-point element in `b`
/// down to an integer value, store the result as a single-precision
/// floating-point element in the lower element of the intrinsic result,
/// and copy the upper 3 packed elements from `a` to the upper elements
/// of the intrinsic result.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(roundss))]
pub unsafe fn _mm_floor_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_floor_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Round the packed double-precision (64-bit) floating-point elements in `a`
/// up to an integer value, and store the results as packed double-precision
/// floating-point elements.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(roundpd))]
pub unsafe fn _mm_ceil_pd(a: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_ceil_pd(::mem::transmute(a)))
}

/// Round the packed single-precision (32-bit) floating-point elements in `a`
/// up to an integer value, and store the results as packed single-precision
/// floating-point elements.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(roundps))]
pub unsafe fn _mm_ceil_ps(a: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_ceil_ps(::mem::transmute(a)))
}

/// Round the lower double-precision (64-bit) floating-point element in `b`
/// up to an integer value, store the result as a double-precision
/// floating-point element in the lower element of the intrisic result,
/// and copy the upper element from `a` to the upper element
/// of the intrinsic result.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(roundsd))]
pub unsafe fn _mm_ceil_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_ceil_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Round the lower single-precision (32-bit) floating-point element in `b`
/// up to an integer value, store the result as a single-precision
/// floating-point element in the lower element of the intrinsic result,
/// and copy the upper 3 packed elements from `a` to the upper elements
/// of the intrinsic result.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(roundss))]
pub unsafe fn _mm_ceil_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_ceil_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Round the packed double-precision (64-bit) floating-point elements in `a`
/// using the `rounding` parameter, and store the results as packed
/// double-precision floating-point elements.
/// Rounding is done according to the rounding parameter, which can be one of:
///
/// ```
/// #![feature(stdsimd)]
/// # #![cfg_attr(not(dox), no_std)]
/// # #[cfg(not(dox))]
/// # extern crate std as real_std;
/// # #[cfg(not(dox))]
/// # extern crate stdsimd as std;
///
/// #[cfg(target_arch = "x86")]
/// use std::arch::x86::*;
/// #[cfg(target_arch = "x86_64")]
/// use std::arch::x86_64::*;
///
/// # fn main() {
/// // round to nearest, and suppress exceptions:
/// # let _x =
/// _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC;
/// // round down, and suppress exceptions:
/// # let _x =
/// _MM_FROUND_TO_NEG_INF | _MM_FROUND_NO_EXC;
/// // round up, and suppress exceptions:
/// # let _x =
/// _MM_FROUND_TO_POS_INF | _MM_FROUND_NO_EXC;
/// // truncate, and suppress exceptions:
/// # let _x =
/// _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC;
/// // use MXCSR.RC; see `_MM_SET_ROUNDING_MODE`:
/// # let _x =
/// _MM_FROUND_CUR_DIRECTION;
/// # }
/// ```
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(roundpd, rounding = 0))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_round_pd(a: f64x2, rounding: i32) -> f64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_round_pd(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(rounding, call))
}

/// Round the packed single-precision (32-bit) floating-point elements in `a`
/// using the `rounding` parameter, and store the results as packed
/// single-precision floating-point elements.
/// Rounding is done according to the rounding parameter, which can be one of:
///
/// ```
/// #![feature(stdsimd)]
///
/// # #![cfg_attr(not(dox), no_std)]
/// # #[cfg(not(dox))]
/// # extern crate std as real_std;
/// # #[cfg(not(dox))]
/// # extern crate stdsimd as std;
///
/// #[cfg(target_arch = "x86")]
/// use std::arch::x86::*;
/// #[cfg(target_arch = "x86_64")]
/// use std::arch::x86_64::*;
///
/// # fn main() {
/// // round to nearest, and suppress exceptions:
/// # let _x =
/// _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC;
/// // round down, and suppress exceptions:
/// # let _x =
/// _MM_FROUND_TO_NEG_INF | _MM_FROUND_NO_EXC;
/// // round up, and suppress exceptions:
/// # let _x =
/// _MM_FROUND_TO_POS_INF | _MM_FROUND_NO_EXC;
/// // truncate, and suppress exceptions:
/// # let _x =
/// _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC;
/// // use MXCSR.RC; see `_MM_SET_ROUNDING_MODE`:
/// # let _x =
/// _MM_FROUND_CUR_DIRECTION;
/// # }
/// ```
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(roundps, rounding = 0))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_round_ps(a: f32x4, rounding: i32) -> f32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_round_ps(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(rounding, call))
}

/// Round the lower double-precision (64-bit) floating-point element in `b`
/// using the `rounding` parameter, store the result as a double-precision
/// floating-point element in the lower element of the intrinsic result,
/// and copy the upper element from `a` to the upper element of the intrinsic
/// result.
/// Rounding is done according to the rounding parameter, which can be one of:
///
/// ```
/// #![feature(stdsimd)]
/// # #![cfg_attr(not(dox), no_std)]
/// # #[cfg(not(dox))]
/// # extern crate std as real_std;
/// # #[cfg(not(dox))]
/// # extern crate stdsimd as std;
///
/// #[cfg(target_arch = "x86")]
/// use std::arch::x86::*;
/// #[cfg(target_arch = "x86_64")]
/// use std::arch::x86_64::*;
///
/// # fn main() {
/// // round to nearest, and suppress exceptions:
/// # let _x =
/// _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC;
/// // round down, and suppress exceptions:
/// # let _x =
/// _MM_FROUND_TO_NEG_INF | _MM_FROUND_NO_EXC;
/// // round up, and suppress exceptions:
/// # let _x =
/// _MM_FROUND_TO_POS_INF | _MM_FROUND_NO_EXC;
/// // truncate, and suppress exceptions:
/// # let _x =
/// _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC;
/// // use MXCSR.RC; see `_MM_SET_ROUNDING_MODE`:
/// # let _x =
/// _MM_FROUND_CUR_DIRECTION;
/// # }
/// ```
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(roundsd, rounding = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_round_sd(a: f64x2, b: f64x2, rounding: i32) -> f64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_round_sd(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(rounding, call))
}

/// Round the lower single-precision (32-bit) floating-point element in `b`
/// using the `rounding` parameter, store the result as a single-precision
/// floating-point element in the lower element of the intrinsic result,
/// and copy the upper 3 packed elements from `a` to the upper elements
/// of the instrinsic result.
/// Rounding is done according to the rounding parameter, which can be one of:
///
/// ```
/// #![feature(stdsimd)]
/// # #![cfg_attr(not(dox), no_std)]
/// # #[cfg(not(dox))]
/// # extern crate std as real_std;
/// # #[cfg(not(dox))]
/// # extern crate stdsimd as std;
///
/// #[cfg(target_arch = "x86")]
/// use std::arch::x86::*;
/// #[cfg(target_arch = "x86_64")]
/// use std::arch::x86_64::*;
///
/// # fn main() {
/// // round to nearest, and suppress exceptions:
/// # let _x =
/// _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC;
/// // round down, and suppress exceptions:
/// # let _x =
/// _MM_FROUND_TO_NEG_INF | _MM_FROUND_NO_EXC;
/// // round up, and suppress exceptions:
/// # let _x =
/// _MM_FROUND_TO_POS_INF | _MM_FROUND_NO_EXC;
/// // truncate, and suppress exceptions:
/// # let _x =
/// _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC;
/// // use MXCSR.RC; see `_MM_SET_ROUNDING_MODE`:
/// # let _x =
/// _MM_FROUND_CUR_DIRECTION;
/// # }
/// ```
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(roundss, rounding = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_round_ss(a: f32x4, b: f32x4, rounding: i32) -> f32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_round_ss(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(rounding, call))
}

/// Finds the minimum unsigned 16-bit element in the 128-bit __m128i vector,
/// returning a vector containing its value in its first position, and its
/// index
/// in its second position; all other elements are set to zero.
///
/// This intrinsic corresponds to the <c> VPHMINPOSUW / PHMINPOSUW </c>
/// instruction.
///
/// Arguments:
///
/// * `a` - A 128-bit vector of type `__m128i`.
///
/// Returns:
///
/// A 128-bit value where:
///
/// * bits `[15:0]` - contain the minimum value found in parameter `a`,
/// * bits `[18:16]` - contain the index of the minimum value
/// * remaining bits are set to `0`.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(phminposuw))]
pub unsafe fn _mm_minpos_epu16(a: u16x8) -> u16x8 {
    ::mem::transmute(::myarch::_mm_minpos_epu16(::mem::transmute(a)))
}

/// Multiply the low 32-bit integers from each packed 64-bit
/// element in `a` and `b`, and return the signed 64-bit result.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmuldq))]
pub unsafe fn _mm_mul_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_mul_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiply the packed 32-bit integers in `a` and `b`, producing intermediate
/// 64-bit integers, and returns the lowest 32-bit, whatever they might be,
/// reinterpreted as a signed integer. While `pmulld __m128i::splat(2),
/// __m128i::splat(2)` returns the obvious `__m128i::splat(4)`, due to wrapping
/// arithmetic `pmulld __m128i::splat(i32::MAX), __m128i::splat(2)` would
/// return a negative number.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pmulld))]
pub unsafe fn _mm_mullo_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_mullo_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtracts 8-bit unsigned integer values and computes the absolute
/// values of the differences to the corresponding bits in the destination.
/// Then sums of the absolute differences are returned according to the bit
/// fields in the immediate operand.
///
/// The following algorithm is performed:
///
/// ```ignore
/// i = imm8[2] * 4
/// j = imm8[1:0] * 4
/// for k := 0 to 7
///     d0 = abs(a[i + k + 0] - b[j + 0])
///     d1 = abs(a[i + k + 1] - b[j + 1])
///     d2 = abs(a[i + k + 2] - b[j + 2])
///     d3 = abs(a[i + k + 3] - b[j + 3])
///     r[k] = d0 + d1 + d2 + d3
/// ```
///
/// Arguments:
///
/// * `a` - A 128-bit vector of type `__m128i`.
/// * `b` - A 128-bit vector of type `__m128i`.
/// * `imm8` - An 8-bit immediate operand specifying how the absolute
///            differences are to be calculated
///     * Bit `[2]` specify the offset for operand `a`
///     * Bits `[1:0]` specify the offset for operand `b`
///
/// Returns:
///
/// * A `__m128i` vector containing the sums of the sets of
///   absolute differences between both operands.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(mpsadbw, imm8 = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_mpsadbw_epu8(a: u8x16, b: u8x16, imm8: i32) -> u8x16 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_mpsadbw_epu8(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Tests whether the specified bits in a 128-bit integer vector are all
/// zeros.
///
/// Arguments:
///
/// * `a` - A 128-bit integer vector containing the bits to be tested.
/// * `mask` - A 128-bit integer vector selecting which bits to test in
///            operand `a`.
///
/// Returns:
///
/// * `1` - if the specified bits are all zeros,
/// * `0` - otherwise.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(ptest))]
pub unsafe fn _mm_testz_si128(a: __m128i, mask: __m128i) -> i32 {
    ::mem::transmute(::myarch::_mm_testz_si128(::mem::transmute(a), ::mem::transmute(mask)))
}

/// Tests whether the specified bits in a 128-bit integer vector are all
/// ones.
///
/// Arguments:
///
/// * `a` - A 128-bit integer vector containing the bits to be tested.
/// * `mask` - A 128-bit integer vector selecting which bits to test in
///            operand `a`.
///
/// Returns:
///
/// * `1` - if the specified bits are all ones,
/// * `0` - otherwise.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(ptest))]
pub unsafe fn _mm_testc_si128(a: __m128i, mask: __m128i) -> i32 {
    ::mem::transmute(::myarch::_mm_testc_si128(::mem::transmute(a), ::mem::transmute(mask)))
}

/// Tests whether the specified bits in a 128-bit integer vector are
/// neither all zeros nor all ones.
///
/// Arguments:
///
/// * `a` - A 128-bit integer vector containing the bits to be tested.
/// * `mask` - A 128-bit integer vector selecting which bits to test in
///            operand `a`.
///
/// Returns:
///
/// * `1` - if the specified bits are neither all zeros nor all ones,
/// * `0` - otherwise.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(ptest))]
pub unsafe fn _mm_testnzc_si128(a: __m128i, mask: __m128i) -> i32 {
    ::mem::transmute(::myarch::_mm_testnzc_si128(::mem::transmute(a), ::mem::transmute(mask)))
}

/// Tests whether the specified bits in a 128-bit integer vector are all
/// zeros.
///
/// Arguments:
///
/// * `a` - A 128-bit integer vector containing the bits to be tested.
/// * `mask` - A 128-bit integer vector selecting which bits to test in
///            operand `a`.
///
/// Returns:
///
/// * `1` - if the specified bits are all zeros,
/// * `0` - otherwise.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(ptest))]
pub unsafe fn _mm_test_all_zeros(a: __m128i, mask: __m128i) -> i32 {
    ::mem::transmute(::myarch::_mm_test_all_zeros(::mem::transmute(a), ::mem::transmute(mask)))
}

/// Tests whether the specified bits in `a` 128-bit integer vector are all
/// ones.
///
/// Argument:
///
/// * `a` - A 128-bit integer vector containing the bits to be tested.
///
/// Returns:
///
/// * `1` - if the bits specified in the operand are all set to 1,
/// * `0` - otherwise.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pcmpeqd))]
#[cfg_attr(test, assert_instr(ptest))]
pub unsafe fn _mm_test_all_ones(a: __m128i) -> i32 {
    ::mem::transmute(::myarch::_mm_test_all_ones(::mem::transmute(a)))
}

/// Tests whether the specified bits in a 128-bit integer vector are
/// neither all zeros nor all ones.
///
/// Arguments:
///
/// * `a` - A 128-bit integer vector containing the bits to be tested.
/// * `mask` - A 128-bit integer vector selecting which bits to test in
///            operand `a`.
///
/// Returns:
///
/// * `1` - if the specified bits are neither all zeros nor all ones,
/// * `0` - otherwise.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(ptest))]
pub unsafe fn _mm_test_mix_ones_zeros(a: __m128i, mask: __m128i) -> i32 {
    ::mem::transmute(::myarch::_mm_test_mix_ones_zeros(::mem::transmute(a), ::mem::transmute(mask)))
}

