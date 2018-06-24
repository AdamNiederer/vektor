#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Computes the absolute values of packed 32-bit integers in `a`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpabsd))]
pub unsafe fn _mm256_abs_epi32(a: i32x8) -> u32x8 {
    ::mem::transmute(::myarch::_mm256_abs_epi32(::mem::transmute(a)))
}

/// Computes the absolute values of packed 16-bit integers in `a`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpabsw))]
pub unsafe fn _mm256_abs_epi16(a: i16x16) -> u16x16 {
    ::mem::transmute(::myarch::_mm256_abs_epi16(::mem::transmute(a)))
}

/// Computes the absolute values of packed 8-bit integers in `a`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpabsb))]
pub unsafe fn _mm256_abs_epi8(a: i8x32) -> u8x32 {
    ::mem::transmute(::myarch::_mm256_abs_epi8(::mem::transmute(a)))
}

/// Add packed 64-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpaddq))]
pub unsafe fn _mm256_add_epi64(a: i64x4, b: i64x4) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_add_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 32-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpaddd))]
pub unsafe fn _mm256_add_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_add_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 16-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpaddw))]
pub unsafe fn _mm256_add_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_add_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 8-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpaddb))]
pub unsafe fn _mm256_add_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::myarch::_mm256_add_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 8-bit integers in `a` and `b` using saturation.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpaddsb))]
pub unsafe fn _mm256_adds_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::myarch::_mm256_adds_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 16-bit integers in `a` and `b` using saturation.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpaddsw))]
pub unsafe fn _mm256_adds_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_adds_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed unsigned 8-bit integers in `a` and `b` using saturation.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpaddusb))]
pub unsafe fn _mm256_adds_epu8(a: u8x32, b: u8x32) -> u8x32 {
    ::mem::transmute(::myarch::_mm256_adds_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed unsigned 16-bit integers in `a` and `b` using saturation.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpaddusw))]
pub unsafe fn _mm256_adds_epu16(a: u16x16, b: u16x16) -> u16x16 {
    ::mem::transmute(::myarch::_mm256_adds_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Concatenate pairs of 16-byte blocks in `a` and `b` into a 32-byte temporary
/// result, shift the result right by `n` bytes, and return the low 16 bytes.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpalignr, n = 7))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_alignr_epi8(a: i8x32, b: i8x32, n: i32) -> i8x32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_alignr_epi8(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(n, call))
}

/// Compute the bitwise AND of 256 bits (representing integer data)
/// in `a` and `b`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vandps))]
pub unsafe fn _mm256_and_si256(a: __m256i, b: __m256i) -> __m256i {
    ::mem::transmute(::myarch::_mm256_and_si256(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise NOT of 256 bits (representing integer data)
/// in `a` and then AND with `b`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vandnps))]
pub unsafe fn _mm256_andnot_si256(a: __m256i, b: __m256i) -> __m256i {
    ::mem::transmute(::myarch::_mm256_andnot_si256(::mem::transmute(a), ::mem::transmute(b)))
}

/// Average packed unsigned 16-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpavgw))]
pub unsafe fn _mm256_avg_epu16(a: u16x16, b: u16x16) -> u16x16 {
    ::mem::transmute(::myarch::_mm256_avg_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Average packed unsigned 8-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpavgb))]
pub unsafe fn _mm256_avg_epu8(a: u8x32, b: u8x32) -> u8x32 {
    ::mem::transmute(::myarch::_mm256_avg_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Blend packed 32-bit integers from `a` and `b` using control mask `imm8`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vblendps, imm8 = 9))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_blend_epi32(a: i32x4, b: i32x4, imm8: i32) -> i32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_blend_epi32(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Blend packed 32-bit integers from `a` and `b` using control mask `imm8`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vblendps, imm8 = 9))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_blend_epi32(a: i32x8, b: i32x8, imm8: i32) -> i32x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_blend_epi32(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Blend packed 16-bit integers from `a` and `b` using control mask `imm8`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpblendw, imm8 = 9))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_blend_epi16(a: i16x16, b: i16x16, imm8: i32) -> i16x16 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_blend_epi16(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Blend packed 8-bit integers from `a` and `b` using `mask`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpblendvb))]
pub unsafe fn _mm256_blendv_epi8(a: i8x32, b: i8x32, mask: i8x32) -> i8x32 {
    ::mem::transmute(::myarch::_mm256_blendv_epi8(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(mask)))
}

/// Broadcast the low packed 8-bit integer from `a` to all elements of
/// the 128-bit returned value.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpbroadcastb))]
pub unsafe fn _mm_broadcastb_epi8(a: i8x16) -> i8x16 {
    ::mem::transmute(::myarch::_mm_broadcastb_epi8(::mem::transmute(a)))
}

/// Broadcast the low packed 8-bit integer from `a` to all elements of
/// the 256-bit returned value.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpbroadcastb))]
pub unsafe fn _mm256_broadcastb_epi8(a: i8x16) -> i8x32 {
    ::mem::transmute(::myarch::_mm256_broadcastb_epi8(::mem::transmute(a)))
}

// NB: simd_shuffle4 with integer data types for `a` and `b` is
// often compiled to vbroadcastss.
/// Broadcast the low packed 32-bit integer from `a` to all elements of
/// the 128-bit returned value.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vbroadcastss))]
pub unsafe fn _mm_broadcastd_epi32(a: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_broadcastd_epi32(::mem::transmute(a)))
}

// NB: simd_shuffle4 with integer data types for `a` and `b` is
// often compiled to vbroadcastss.
/// Broadcast the low packed 32-bit integer from `a` to all elements of
/// the 256-bit returned value.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vbroadcastss))]
pub unsafe fn _mm256_broadcastd_epi32(a: i32x4) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_broadcastd_epi32(::mem::transmute(a)))
}

/// Broadcast the low packed 64-bit integer from `a` to all elements of
/// the 128-bit returned value.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpbroadcastq))]
pub unsafe fn _mm_broadcastq_epi64(a: i64x2) -> i64x2 {
    ::mem::transmute(::myarch::_mm_broadcastq_epi64(::mem::transmute(a)))
}

// NB: simd_shuffle4 with integer data types for `a` and `b` is
// often compiled to vbroadcastsd.
/// Broadcast the low packed 64-bit integer from `a` to all elements of
/// the 256-bit returned value.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vbroadcastsd))]
pub unsafe fn _mm256_broadcastq_epi64(a: i64x2) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_broadcastq_epi64(::mem::transmute(a)))
}

/// Broadcast the low double-precision (64-bit) floating-point element
/// from `a` to all elements of the 128-bit returned value.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vmovddup))]
pub unsafe fn _mm_broadcastsd_pd(a: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_broadcastsd_pd(::mem::transmute(a)))
}

/// Broadcast the low double-precision (64-bit) floating-point element
/// from `a` to all elements of the 256-bit returned value.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vbroadcastsd))]
pub unsafe fn _mm256_broadcastsd_pd(a: f64x2) -> f64x4 {
    ::mem::transmute(::myarch::_mm256_broadcastsd_pd(::mem::transmute(a)))
}

// NB: broadcastsi128_si256 is often compiled to vinsertf128 or
// vbroadcastf128.
/// Broadcast 128 bits of integer data from a to all 128-bit lanes in
/// the 256-bit returned value.
#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_broadcastsi128_si256(a: __m128i) -> __m256i {
    ::mem::transmute(::myarch::_mm256_broadcastsi128_si256(::mem::transmute(a)))
}

/// Broadcast the low single-precision (32-bit) floating-point element
/// from `a` to all elements of the 128-bit returned value.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vbroadcastss))]
pub unsafe fn _mm_broadcastss_ps(a: f32x4) -> f32x4 {
    ::mem::transmute(::myarch::_mm_broadcastss_ps(::mem::transmute(a)))
}

/// Broadcast the low single-precision (32-bit) floating-point element
/// from `a` to all elements of the 256-bit returned value.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vbroadcastss))]
pub unsafe fn _mm256_broadcastss_ps(a: f32x4) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_broadcastss_ps(::mem::transmute(a)))
}

/// Broadcast the low packed 16-bit integer from a to all elements of
/// the 128-bit returned value
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpbroadcastw))]
pub unsafe fn _mm_broadcastw_epi16(a: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_broadcastw_epi16(::mem::transmute(a)))
}

/// Broadcast the low packed 16-bit integer from a to all elements of
/// the 256-bit returned value
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpbroadcastw))]
pub unsafe fn _mm256_broadcastw_epi16(a: i16x8) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_broadcastw_epi16(::mem::transmute(a)))
}

/// Compare packed 64-bit integers in `a` and `b` for equality.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpcmpeqq))]
pub unsafe fn _mm256_cmpeq_epi64(a: i64x4, b: i64x4) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_cmpeq_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 32-bit integers in `a` and `b` for equality.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpcmpeqd))]
pub unsafe fn _mm256_cmpeq_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_cmpeq_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 16-bit integers in `a` and `b` for equality.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpcmpeqw))]
pub unsafe fn _mm256_cmpeq_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_cmpeq_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 8-bit integers in `a` and `b` for equality.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpcmpeqb))]
pub unsafe fn _mm256_cmpeq_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::myarch::_mm256_cmpeq_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 64-bit integers in `a` and `b` for greater-than.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpcmpgtq))]
pub unsafe fn _mm256_cmpgt_epi64(a: i64x4, b: i64x4) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_cmpgt_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 32-bit integers in `a` and `b` for greater-than.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpcmpgtd))]
pub unsafe fn _mm256_cmpgt_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_cmpgt_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 16-bit integers in `a` and `b` for greater-than.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpcmpgtw))]
pub unsafe fn _mm256_cmpgt_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_cmpgt_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 8-bit integers in `a` and `b` for greater-than.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpcmpgtb))]
pub unsafe fn _mm256_cmpgt_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::myarch::_mm256_cmpgt_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Sign-extend 16-bit integers to 32-bit integers.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmovsxwd))]
pub unsafe fn _mm256_cvtepi16_epi32(a: i16x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_cvtepi16_epi32(::mem::transmute(a)))
}

/// Sign-extend 16-bit integers to 64-bit integers.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmovsxwq))]
pub unsafe fn _mm256_cvtepi16_epi64(a: i16x8) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_cvtepi16_epi64(::mem::transmute(a)))
}

/// Sign-extend 32-bit integers to 64-bit integers.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmovsxdq))]
pub unsafe fn _mm256_cvtepi32_epi64(a: i32x4) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_cvtepi32_epi64(::mem::transmute(a)))
}

/// Sign-extend 8-bit integers to 16-bit integers.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmovsxbw))]
pub unsafe fn _mm256_cvtepi8_epi16(a: i8x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_cvtepi8_epi16(::mem::transmute(a)))
}

/// Sign-extend 8-bit integers to 32-bit integers.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmovsxbd))]
pub unsafe fn _mm256_cvtepi8_epi32(a: i8x16) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_cvtepi8_epi32(::mem::transmute(a)))
}

/// Sign-extend 8-bit integers to 64-bit integers.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmovsxbq))]
pub unsafe fn _mm256_cvtepi8_epi64(a: i8x16) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_cvtepi8_epi64(::mem::transmute(a)))
}

/// Zero extend packed unsigned 16-bit integers in `a` to packed 32-bit
/// integers, and store the results in dst.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmovzxwd))]
pub unsafe fn _mm256_cvtepu16_epi32(a: u16x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_cvtepu16_epi32(::mem::transmute(a)))
}

/// Zero-extend the lower four unsigned 16-bit integers in `a` to 64-bit
/// integers. The upper four elements of `a` are unused.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmovzxwq))]
pub unsafe fn _mm256_cvtepu16_epi64(a: u16x8) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_cvtepu16_epi64(::mem::transmute(a)))
}

/// Zero-extend unsigned 32-bit integers in `a` to 64-bit integers.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmovzxdq))]
pub unsafe fn _mm256_cvtepu32_epi64(a: u32x4) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_cvtepu32_epi64(::mem::transmute(a)))
}

/// Zero-extend unsigned 8-bit integers in `a` to 16-bit integers.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmovzxbw))]
pub unsafe fn _mm256_cvtepu8_epi16(a: u8x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_cvtepu8_epi16(::mem::transmute(a)))
}

/// Zero-extend the lower eight unsigned 8-bit integers in `a` to 32-bit
/// integers. The upper eight elements of `a` are unused.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmovzxbd))]
pub unsafe fn _mm256_cvtepu8_epi32(a: u8x16) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_cvtepu8_epi32(::mem::transmute(a)))
}

/// Zero-extend the lower four unsigned 8-bit integers in `a` to 64-bit
/// integers. The upper twelve elements of `a` are unused.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmovzxbq))]
pub unsafe fn _mm256_cvtepu8_epi64(a: u8x16) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_cvtepu8_epi64(::mem::transmute(a)))
}

/// Extract 128 bits (of integer data) from `a` selected with `imm8`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vextractf128, imm8 = 1))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_extracti128_si256(a: __m256i, imm8: i32) -> __m128i {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_extracti128_si256(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Horizontally add adjacent pairs of 16-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vphaddw))]
pub unsafe fn _mm256_hadd_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_hadd_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontally add adjacent pairs of 32-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vphaddd))]
pub unsafe fn _mm256_hadd_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_hadd_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontally add adjacent pairs of 16-bit integers in `a` and `b`
/// using saturation.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vphaddsw))]
pub unsafe fn _mm256_hadds_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_hadds_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontally subtract adjacent pairs of 16-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vphsubw))]
pub unsafe fn _mm256_hsub_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_hsub_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontally subtract adjacent pairs of 32-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vphsubd))]
pub unsafe fn _mm256_hsub_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_hsub_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontally subtract adjacent pairs of 16-bit integers in `a` and `b`
/// using saturation.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vphsubsw))]
pub unsafe fn _mm256_hsubs_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_hsubs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpgatherdd, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_i32gather_epi32(slice: *const i32, offsets: i32x4, scale: i32) -> i32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_i32gather_epi32(::mem::transmute(slice), ::mem::transmute(offsets), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8. If mask is set, load the value from `src` in
/// that position instead.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpgatherdd, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm_mask_i32gather_epi32(src: i32x4, slice: *const i32, offsets: i32x4, mask: i32x4, scale: i32) -> i32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_mask_i32gather_epi32(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpgatherdd, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_i32gather_epi32(slice: *const i32, offsets: i32x8, scale: i32) -> i32x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_i32gather_epi32(::mem::transmute(slice), ::mem::transmute(offsets), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8. If mask is set, load the value from `src` in
/// that position instead.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpgatherdd, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm256_mask_i32gather_epi32(src: i32x8, slice: *const i32, offsets: i32x8, mask: i32x8, scale: i32) -> i32x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_mask_i32gather_epi32(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vgatherdps, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_i32gather_ps(slice: *const f32, offsets: __m128i, scale: i32) -> f32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_i32gather_ps(::mem::transmute(slice), ::mem::transmute(offsets), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8. If mask is set, load the value from `src` in
/// that position instead.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vgatherdps, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm_mask_i32gather_ps(src: f32x4, slice: *const f32, offsets: __m128i, mask: f32x4, scale: i32) -> f32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_mask_i32gather_ps(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vgatherdps, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_i32gather_ps(slice: *const f32, offsets: __m256i, scale: i32) -> f32x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_i32gather_ps(::mem::transmute(slice), ::mem::transmute(offsets), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8. If mask is set, load the value from `src` in
/// that position instead.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vgatherdps, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm256_mask_i32gather_ps(src: f32x8, slice: *const f32, offsets: __m256i, mask: f32x8, scale: i32) -> f32x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_mask_i32gather_ps(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpgatherdq, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_i32gather_epi64(slice: *const i64, offsets: i64x2, scale: i32) -> i64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_i32gather_epi64(::mem::transmute(slice), ::mem::transmute(offsets), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8. If mask is set, load the value from `src` in
/// that position instead.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpgatherdq, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm_mask_i32gather_epi64(src: i64x2, slice: *const i64, offsets: i64x2, mask: i64x2, scale: i32) -> i64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_mask_i32gather_epi64(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpgatherdq, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_i32gather_epi64(slice: *const i64, offsets: i64x2, scale: i32) -> i64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_i32gather_epi64(::mem::transmute(slice), ::mem::transmute(offsets), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8. If mask is set, load the value from `src` in
/// that position instead.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpgatherdq, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm256_mask_i32gather_epi64(src: i64x4, slice: *const i64, offsets: i64x2, mask: i64x4, scale: i32) -> i64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_mask_i32gather_epi64(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vgatherdpd, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_i32gather_pd(slice: *const f64, offsets: __m128i, scale: i32) -> f64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_i32gather_pd(::mem::transmute(slice), ::mem::transmute(offsets), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8. If mask is set, load the value from `src` in
/// that position instead.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vgatherdpd, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm_mask_i32gather_pd(src: f64x2, slice: *const f64, offsets: __m128i, mask: f64x2, scale: i32) -> f64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_mask_i32gather_pd(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vgatherdpd, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_i32gather_pd(slice: *const f64, offsets: __m128i, scale: i32) -> f64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_i32gather_pd(::mem::transmute(slice), ::mem::transmute(offsets), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8. If mask is set, load the value from `src` in
/// that position instead.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vgatherdpd, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm256_mask_i32gather_pd(src: f64x4, slice: *const f64, offsets: __m128i, mask: f64x4, scale: i32) -> f64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_mask_i32gather_pd(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpgatherqd, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_i64gather_epi32(slice: *const i32, offsets: i32x4, scale: i32) -> i32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_i64gather_epi32(::mem::transmute(slice), ::mem::transmute(offsets), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8. If mask is set, load the value from `src` in
/// that position instead.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpgatherqd, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm_mask_i64gather_epi32(src: i32x4, slice: *const i32, offsets: i32x4, mask: i32x4, scale: i32) -> i32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_mask_i64gather_epi32(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpgatherqd, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_i64gather_epi32(slice: *const i32, offsets: i32x8, scale: i32) -> i32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_i64gather_epi32(::mem::transmute(slice), ::mem::transmute(offsets), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8. If mask is set, load the value from `src` in
/// that position instead.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpgatherqd, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm256_mask_i64gather_epi32(src: i32x4, slice: *const i32, offsets: i32x8, mask: i32x4, scale: i32) -> i32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_mask_i64gather_epi32(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vgatherqps, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_i64gather_ps(slice: *const f32, offsets: __m128i, scale: i32) -> f32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_i64gather_ps(::mem::transmute(slice), ::mem::transmute(offsets), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8. If mask is set, load the value from `src` in
/// that position instead.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vgatherqps, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm_mask_i64gather_ps(src: f32x4, slice: *const f32, offsets: __m128i, mask: f32x4, scale: i32) -> f32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_mask_i64gather_ps(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vgatherqps, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_i64gather_ps(slice: *const f32, offsets: __m256i, scale: i32) -> f32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_i64gather_ps(::mem::transmute(slice), ::mem::transmute(offsets), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8. If mask is set, load the value from `src` in
/// that position instead.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vgatherqps, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm256_mask_i64gather_ps(src: f32x4, slice: *const f32, offsets: __m256i, mask: f32x4, scale: i32) -> f32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_mask_i64gather_ps(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpgatherqq, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_i64gather_epi64(slice: *const i64, offsets: i64x2, scale: i32) -> i64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_i64gather_epi64(::mem::transmute(slice), ::mem::transmute(offsets), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8. If mask is set, load the value from `src` in
/// that position instead.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpgatherqq, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm_mask_i64gather_epi64(src: i64x2, slice: *const i64, offsets: i64x2, mask: i64x2, scale: i32) -> i64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_mask_i64gather_epi64(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpgatherqq, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_i64gather_epi64(slice: *const i64, offsets: i64x4, scale: i32) -> i64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_i64gather_epi64(::mem::transmute(slice), ::mem::transmute(offsets), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8. If mask is set, load the value from `src` in
/// that position instead.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpgatherqq, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm256_mask_i64gather_epi64(src: i64x4, slice: *const i64, offsets: i64x4, mask: i64x4, scale: i32) -> i64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_mask_i64gather_epi64(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vgatherqpd, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_i64gather_pd(slice: *const f64, offsets: __m128i, scale: i32) -> f64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_i64gather_pd(::mem::transmute(slice), ::mem::transmute(offsets), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8. If mask is set, load the value from `src` in
/// that position instead.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vgatherqpd, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm_mask_i64gather_pd(src: f64x2, slice: *const f64, offsets: __m128i, mask: f64x2, scale: i32) -> f64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_mask_i64gather_pd(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vgatherqpd, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_i64gather_pd(slice: *const f64, offsets: __m256i, scale: i32) -> f64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_i64gather_pd(::mem::transmute(slice), ::mem::transmute(offsets), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Return values from `slice` at offsets determined by `offsets * scale`,
/// where
/// `scale` is between 1 and 8. If mask is set, load the value from `src` in
/// that position instead.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vgatherqpd, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm256_mask_i64gather_pd(src: f64x4, slice: *const f64, offsets: __m256i, mask: f64x4, scale: i32) -> f64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_mask_i64gather_pd(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(scale, call))
}

/// Copy `a` to `dst`, then insert 128 bits (of integer data) from `b` at the
/// location specified by `imm8`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vinsertf128, imm8 = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_inserti128_si256(a: __m256i, b: __m128i, imm8: i32) -> __m256i {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_inserti128_si256(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Multiply packed signed 16-bit integers in `a` and `b`, producing
/// intermediate signed 32-bit integers. Horizontally add adjacent pairs
/// of intermediate 32-bit integers.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmaddwd))]
pub unsafe fn _mm256_madd_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_madd_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Vertically multiply each unsigned 8-bit integer from `a` with the
/// corresponding signed 8-bit integer from `b`, producing intermediate
/// signed 16-bit integers. Horizontally add adjacent pairs of intermediate
/// signed 16-bit integers
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmaddubsw))]
pub unsafe fn _mm256_maddubs_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_maddubs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Load packed 32-bit integers from memory pointed by `mem_addr` using `mask`
/// (elements are zeroed out when the highest bit is not set in the
/// corresponding element).
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmaskmovd))]
pub unsafe fn _mm_maskload_epi32(mem_addr: *const i32, mask: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_maskload_epi32(::mem::transmute(mem_addr), ::mem::transmute(mask)))
}

/// Load packed 32-bit integers from memory pointed by `mem_addr` using `mask`
/// (elements are zeroed out when the highest bit is not set in the
/// corresponding element).
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmaskmovd))]
pub unsafe fn _mm256_maskload_epi32(mem_addr: *const i32, mask: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_maskload_epi32(::mem::transmute(mem_addr), ::mem::transmute(mask)))
}

/// Load packed 64-bit integers from memory pointed by `mem_addr` using `mask`
/// (elements are zeroed out when the highest bit is not set in the
/// corresponding element).
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmaskmovq))]
pub unsafe fn _mm_maskload_epi64(mem_addr: *const i64, mask: i64x2) -> i64x2 {
    ::mem::transmute(::myarch::_mm_maskload_epi64(::mem::transmute(mem_addr), ::mem::transmute(mask)))
}

/// Load packed 64-bit integers from memory pointed by `mem_addr` using `mask`
/// (elements are zeroed out when the highest bit is not set in the
/// corresponding element).
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmaskmovq))]
pub unsafe fn _mm256_maskload_epi64(mem_addr: *const i64, mask: i64x4) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_maskload_epi64(::mem::transmute(mem_addr), ::mem::transmute(mask)))
}

/// Compare packed 16-bit integers in `a` and `b`, and return the packed
/// maximum values.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmaxsw))]
pub unsafe fn _mm256_max_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_max_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 32-bit integers in `a` and `b`, and return the packed
/// maximum values.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmaxsd))]
pub unsafe fn _mm256_max_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_max_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 8-bit integers in `a` and `b`, and return the packed
/// maximum values.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmaxsb))]
pub unsafe fn _mm256_max_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::myarch::_mm256_max_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed unsigned 16-bit integers in `a` and `b`, and return
/// the packed maximum values.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmaxuw))]
pub unsafe fn _mm256_max_epu16(a: u16x16, b: u16x16) -> u16x16 {
    ::mem::transmute(::myarch::_mm256_max_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed unsigned 32-bit integers in `a` and `b`, and return
/// the packed maximum values.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmaxud))]
pub unsafe fn _mm256_max_epu32(a: u32x8, b: u32x8) -> u32x8 {
    ::mem::transmute(::myarch::_mm256_max_epu32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed unsigned 8-bit integers in `a` and `b`, and return
/// the packed maximum values.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmaxub))]
pub unsafe fn _mm256_max_epu8(a: u8x32, b: u8x32) -> u8x32 {
    ::mem::transmute(::myarch::_mm256_max_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 16-bit integers in `a` and `b`, and return the packed
/// minimum values.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpminsw))]
pub unsafe fn _mm256_min_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_min_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 32-bit integers in `a` and `b`, and return the packed
/// minimum values.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpminsd))]
pub unsafe fn _mm256_min_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_min_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 8-bit integers in `a` and `b`, and return the packed
/// minimum values.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpminsb))]
pub unsafe fn _mm256_min_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::myarch::_mm256_min_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed unsigned 16-bit integers in `a` and `b`, and return
/// the packed minimum values.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpminuw))]
pub unsafe fn _mm256_min_epu16(a: u16x16, b: u16x16) -> u16x16 {
    ::mem::transmute(::myarch::_mm256_min_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed unsigned 32-bit integers in `a` and `b`, and return
/// the packed minimum values.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpminud))]
pub unsafe fn _mm256_min_epu32(a: u32x8, b: u32x8) -> u32x8 {
    ::mem::transmute(::myarch::_mm256_min_epu32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed unsigned 8-bit integers in `a` and `b`, and return
/// the packed minimum values.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpminub))]
pub unsafe fn _mm256_min_epu8(a: u8x32, b: u8x32) -> u8x32 {
    ::mem::transmute(::myarch::_mm256_min_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Create mask from the most significant bit of each 8-bit element in `a`,
/// return the result.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmovmskb))]
pub unsafe fn _mm256_movemask_epi8(a: i8x32) -> i32 {
    ::mem::transmute(::myarch::_mm256_movemask_epi8(::mem::transmute(a)))
}

/// Compute the sum of absolute differences (SADs) of quadruplets of unsigned
/// 8-bit integers in `a` compared to those in `b`, and store the 16-bit
/// results in dst. Eight SADs are performed for each 128-bit lane using one
/// quadruplet from `b` and eight quadruplets from `a`. One quadruplet is
/// selected from `b` starting at on the offset specified in `imm8`. Eight
/// quadruplets are formed from sequential 8-bit integers selected from `a`
/// starting at the offset specified in `imm8`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vmpsadbw, imm8 = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_mpsadbw_epu8(a: u8x32, b: u8x32, imm8: i32) -> u8x32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_mpsadbw_epu8(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Multiply the low 32-bit integers from each packed 64-bit element in
/// `a` and `b`
///
/// Return the 64-bit results.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmuldq))]
pub unsafe fn _mm256_mul_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_mul_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiply the low unsigned 32-bit integers from each packed 64-bit
/// element in `a` and `b`
///
/// Return the unsigned 64-bit results.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmuludq))]
pub unsafe fn _mm256_mul_epu32(a: u32x8, b: u32x8) -> u32x8 {
    ::mem::transmute(::myarch::_mm256_mul_epu32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiply the packed 16-bit integers in `a` and `b`, producing
/// intermediate 32-bit integers and returning the high 16 bits of the
/// intermediate integers.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmulhw))]
pub unsafe fn _mm256_mulhi_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_mulhi_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiply the packed unsigned 16-bit integers in `a` and `b`, producing
/// intermediate 32-bit integers and returning the high 16 bits of the
/// intermediate integers.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmulhuw))]
pub unsafe fn _mm256_mulhi_epu16(a: u16x16, b: u16x16) -> u16x16 {
    ::mem::transmute(::myarch::_mm256_mulhi_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiply the packed 16-bit integers in `a` and `b`, producing
/// intermediate 32-bit integers, and return the low 16 bits of the
/// intermediate integers
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmullw))]
pub unsafe fn _mm256_mullo_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_mullo_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiply the packed 32-bit integers in `a` and `b`, producing
/// intermediate 64-bit integers, and return the low 16 bits of the
/// intermediate integers
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmulld))]
pub unsafe fn _mm256_mullo_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_mullo_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiply packed 16-bit integers in `a` and `b`, producing
/// intermediate signed 32-bit integers. Truncate each intermediate
/// integer to the 18 most significant bits, round by adding 1, and
/// return bits [16:1]
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpmulhrsw))]
pub unsafe fn _mm256_mulhrs_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_mulhrs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise OR of 256 bits (representing integer data) in `a`
/// and `b`
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vorps))]
pub unsafe fn _mm256_or_si256(a: __m256i, b: __m256i) -> __m256i {
    ::mem::transmute(::myarch::_mm256_or_si256(::mem::transmute(a), ::mem::transmute(b)))
}

/// Convert packed 16-bit integers from `a` and `b` to packed 8-bit integers
/// using signed saturation
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpacksswb))]
pub unsafe fn _mm256_packs_epi16(a: i16x16, b: i16x16) -> u8x32 {
    ::mem::transmute(::myarch::_mm256_packs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Convert packed 32-bit integers from `a` and `b` to packed 16-bit integers
/// using signed saturation
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpackssdw))]
pub unsafe fn _mm256_packs_epi32(a: i32x8, b: i32x8) -> u16x16 {
    ::mem::transmute(::myarch::_mm256_packs_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Convert packed 16-bit integers from `a` and `b` to packed 8-bit integers
/// using unsigned saturation
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpackuswb))]
pub unsafe fn _mm256_packus_epi16(a: i16x16, b: i16x16) -> u8x32 {
    ::mem::transmute(::myarch::_mm256_packus_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Convert packed 32-bit integers from `a` and `b` to packed 16-bit integers
/// using unsigned saturation
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpackusdw))]
pub unsafe fn _mm256_packus_epi32(a: i32x8, b: i32x8) -> u16x16 {
    ::mem::transmute(::myarch::_mm256_packus_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Permutes packed 32-bit integers from `a` according to the content of `b`.
///
/// The last 3 bits of each integer of `b` are used as addresses into the 8
/// integers of `a`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpermps))]
pub unsafe fn _mm256_permutevar8x32_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_permutevar8x32_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Permutes 64-bit integers from `a` using control mask `imm8`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpermpd, imm8 = 9))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_permute4x64_epi64(a: i64x4, imm8: i32) -> i64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_permute4x64_epi64(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shuffle 128-bits of integer data selected by `imm8` from `a` and `b`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vperm2f128, imm8 = 9))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm256_permute2x128_si256(a: __m256i, b: __m256i, imm8: i32) -> __m256i {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_permute2x128_si256(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shuffle 64-bit floating-point elements in `a` across lanes using the
/// control in `imm8`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpermpd, imm8 = 1))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_permute4x64_pd(a: f64x4, imm8: i32) -> f64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_permute4x64_pd(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shuffle eight 32-bit foating-point elements in `a` across lanes using
/// the corresponding 32-bit integer index in `idx`.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpermps))]
pub unsafe fn _mm256_permutevar8x32_ps(a: f32x8, idx: __m256i) -> f32x8 {
    ::mem::transmute(::myarch::_mm256_permutevar8x32_ps(::mem::transmute(a), ::mem::transmute(idx)))
}

/// Compute the absolute differences of packed unsigned 8-bit integers in `a`
/// and `b`, then horizontally sum each consecutive 8 differences to
/// produce four unsigned 16-bit integers, and pack these unsigned 16-bit
/// integers in the low 16 bits of the 64-bit return value
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsadbw))]
pub unsafe fn _mm256_sad_epu8(a: u8x32, b: u8x32) -> u8x32 {
    ::mem::transmute(::myarch::_mm256_sad_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Shuffle bytes from `a` according to the content of `b`.
///
/// The last 4 bits of each byte of `b` are used as addresses into the 32 bytes
/// of `a`.
///
/// In addition, if the highest significant bit of a byte of `b` is set, the
/// respective destination byte is set to 0.
///
/// The low and high halves of the vectors are shuffled separately.
///
/// Picturing `a` and `b` as `[u8; 32]`, `_mm256_shuffle_epi8` is logically
/// equivalent to:
///
/// ```
/// fn mm256_shuffle_epi8(a: [u8; 32], b: [u8; 32]) -> [u8; 32] {
///     let mut r = [0; 32];
///     for i in 0..16 {
///         // if the most significant bit of b is set,
///         // then the destination byte is set to 0.
///         if b[i] & 0x80 == 0u8 {
///             r[i] = a[(b[i] % 16) as usize];
///         }
///         if b[i + 16] & 0x80 == 0u8 {
///             r[i + 16] = a[(b[i + 16] % 16 + 16) as usize];
///         }
///     }
///     r
/// }
/// ```
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpshufb))]
pub unsafe fn _mm256_shuffle_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::myarch::_mm256_shuffle_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Shuffle 32-bit integers in 128-bit lanes of `a` using the control in
/// `imm8`.
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
/// # fn main() {
/// #     if is_x86_feature_detected!("avx2") {
/// #         #[target_feature(enable = "avx2")]
/// #         unsafe fn worker() {
/// let a = _mm256_setr_epi32(0, 1, 2, 3, 4, 5, 6, 7);
///
/// let c1 = _mm256_shuffle_epi32(a, 0b00_11_10_01);
/// let c2 = _mm256_shuffle_epi32(a, 0b01_00_10_11);
///
/// let expected1 = _mm256_setr_epi32(1, 2, 3, 0, 5, 6, 7, 4);
/// let expected2 = _mm256_setr_epi32(3, 2, 0, 1, 7, 6, 4, 5);
///
/// assert_eq!(_mm256_movemask_epi8(_mm256_cmpeq_epi8(c1, expected1)), !0);
/// assert_eq!(_mm256_movemask_epi8(_mm256_cmpeq_epi8(c2, expected2)), !0);
/// #         }
/// #         unsafe { worker(); }
/// #     }
/// # }
/// ```
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpermilps, imm8 = 9))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_shuffle_epi32(a: i32x8, imm8: i32) -> i32x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_shuffle_epi32(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shuffle 16-bit integers in the high 64 bits of 128-bit lanes of `a` using
/// the control in `imm8`. The low 64 bits of 128-bit lanes of `a` are copied
/// to the output.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpshufhw, imm8 = 9))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_shufflehi_epi16(a: i16x16, imm8: i32) -> i16x16 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_shufflehi_epi16(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shuffle 16-bit integers in the low 64 bits of 128-bit lanes of `a` using
/// the control in `imm8`. The high 64 bits of 128-bit lanes of `a` are copied
/// to the output.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpshuflw, imm8 = 9))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_shufflelo_epi16(a: i16x16, imm8: i32) -> i16x16 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_shufflelo_epi16(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Negate packed 16-bit integers in `a` when the corresponding signed
/// 16-bit integer in `b` is negative, and return the results.
/// Results are zeroed out when the corresponding element in `b` is zero.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsignw))]
pub unsafe fn _mm256_sign_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_sign_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Negate packed 32-bit integers in `a` when the corresponding signed
/// 32-bit integer in `b` is negative, and return the results.
/// Results are zeroed out when the corresponding element in `b` is zero.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsignd))]
pub unsafe fn _mm256_sign_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_sign_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Negate packed 8-bit integers in `a` when the corresponding signed
/// 8-bit integer in `b` is negative, and return the results.
/// Results are zeroed out when the corresponding element in `b` is zero.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsignb))]
pub unsafe fn _mm256_sign_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::myarch::_mm256_sign_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Shift packed 16-bit integers in `a` left by `count` while
/// shifting in zeros, and return the result
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsllw))]
pub unsafe fn _mm256_sll_epi16(a: i16x16, count: i16x8) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_sll_epi16(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 32-bit integers in `a` left by `count` while
/// shifting in zeros, and return the result
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpslld))]
pub unsafe fn _mm256_sll_epi32(a: i32x8, count: i32x4) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_sll_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 64-bit integers in `a` left by `count` while
/// shifting in zeros, and return the result
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsllq))]
pub unsafe fn _mm256_sll_epi64(a: i64x4, count: i64x2) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_sll_epi64(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 16-bit integers in `a` left by `imm8` while
/// shifting in zeros, return the results;
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsllw))]
pub unsafe fn _mm256_slli_epi16(a: i16x16, imm8: i32) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_slli_epi16(::mem::transmute(a), imm8))
}

/// Shift packed 32-bit integers in `a` left by `imm8` while
/// shifting in zeros, return the results;
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpslld))]
pub unsafe fn _mm256_slli_epi32(a: i32x8, imm8: i32) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_slli_epi32(::mem::transmute(a), imm8))
}

/// Shift packed 64-bit integers in `a` left by `imm8` while
/// shifting in zeros, return the results;
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsllq))]
pub unsafe fn _mm256_slli_epi64(a: i64x4, imm8: i32) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_slli_epi64(::mem::transmute(a), imm8))
}

/// Shift 128-bit lanes in `a` left by `imm8` bytes while shifting in zeros.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpslldq, imm8 = 3))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_slli_si256(a: __m256i, imm8: i32) -> __m256i {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_slli_si256(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shift 128-bit lanes in `a` left by `imm8` bytes while shifting in zeros.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpslldq, imm8 = 3))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_bslli_epi128(a: __m256i, imm8: i32) -> __m256i {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_bslli_epi128(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shift packed 32-bit integers in `a` left by the amount
/// specified by the corresponding element in `count` while
/// shifting in zeros, and return the result.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsllvd))]
pub unsafe fn _mm_sllv_epi32(a: i32x4, count: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_sllv_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 32-bit integers in `a` left by the amount
/// specified by the corresponding element in `count` while
/// shifting in zeros, and return the result.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsllvd))]
pub unsafe fn _mm256_sllv_epi32(a: i32x8, count: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_sllv_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 64-bit integers in `a` left by the amount
/// specified by the corresponding element in `count` while
/// shifting in zeros, and return the result.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsllvq))]
pub unsafe fn _mm_sllv_epi64(a: i64x2, count: i64x2) -> i64x2 {
    ::mem::transmute(::myarch::_mm_sllv_epi64(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 64-bit integers in `a` left by the amount
/// specified by the corresponding element in `count` while
/// shifting in zeros, and return the result.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsllvq))]
pub unsafe fn _mm256_sllv_epi64(a: i64x4, count: i64x4) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_sllv_epi64(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 16-bit integers in `a` right by `count` while
/// shifting in sign bits.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsraw))]
pub unsafe fn _mm256_sra_epi16(a: i16x16, count: i16x8) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_sra_epi16(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 32-bit integers in `a` right by `count` while
/// shifting in sign bits.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsrad))]
pub unsafe fn _mm256_sra_epi32(a: i32x8, count: i32x4) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_sra_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 16-bit integers in `a` right by `imm8` while
/// shifting in sign bits.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsraw))]
pub unsafe fn _mm256_srai_epi16(a: i16x16, imm8: i32) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_srai_epi16(::mem::transmute(a), imm8))
}

/// Shift packed 32-bit integers in `a` right by `imm8` while
/// shifting in sign bits.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsrad))]
pub unsafe fn _mm256_srai_epi32(a: i32x8, imm8: i32) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_srai_epi32(::mem::transmute(a), imm8))
}

/// Shift packed 32-bit integers in `a` right by the amount specified by the
/// corresponding element in `count` while shifting in sign bits.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsravd))]
pub unsafe fn _mm_srav_epi32(a: i32x4, count: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_srav_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 32-bit integers in `a` right by the amount specified by the
/// corresponding element in `count` while shifting in sign bits.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsravd))]
pub unsafe fn _mm256_srav_epi32(a: i32x8, count: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_srav_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift 128-bit lanes in `a` right by `imm8` bytes while shifting in zeros.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsrldq, imm8 = 3))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_srli_si256(a: __m256i, imm8: i32) -> __m256i {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_srli_si256(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shift 128-bit lanes in `a` right by `imm8` bytes while shifting in zeros.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsrldq, imm8 = 3))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_bsrli_epi128(a: __m256i, imm8: i32) -> __m256i {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_bsrli_epi128(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shift packed 16-bit integers in `a` right by `count` while shifting in
/// zeros.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsrlw))]
pub unsafe fn _mm256_srl_epi16(a: i16x16, count: i16x8) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_srl_epi16(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 32-bit integers in `a` right by `count` while shifting in
/// zeros.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsrld))]
pub unsafe fn _mm256_srl_epi32(a: i32x8, count: i32x4) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_srl_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 64-bit integers in `a` right by `count` while shifting in
/// zeros.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsrlq))]
pub unsafe fn _mm256_srl_epi64(a: i64x4, count: i64x2) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_srl_epi64(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 16-bit integers in `a` right by `imm8` while shifting in
/// zeros
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsrlw))]
pub unsafe fn _mm256_srli_epi16(a: i16x16, imm8: i32) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_srli_epi16(::mem::transmute(a), imm8))
}

/// Shift packed 32-bit integers in `a` right by `imm8` while shifting in
/// zeros
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsrld))]
pub unsafe fn _mm256_srli_epi32(a: i32x8, imm8: i32) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_srli_epi32(::mem::transmute(a), imm8))
}

/// Shift packed 64-bit integers in `a` right by `imm8` while shifting in
/// zeros
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsrlq))]
pub unsafe fn _mm256_srli_epi64(a: i64x4, imm8: i32) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_srli_epi64(::mem::transmute(a), imm8))
}

/// Shift packed 32-bit integers in `a` right by the amount specified by
/// the corresponding element in `count` while shifting in zeros,
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsrlvd))]
pub unsafe fn _mm_srlv_epi32(a: i32x4, count: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_srlv_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 32-bit integers in `a` right by the amount specified by
/// the corresponding element in `count` while shifting in zeros,
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsrlvd))]
pub unsafe fn _mm256_srlv_epi32(a: i32x8, count: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_srlv_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 64-bit integers in `a` right by the amount specified by
/// the corresponding element in `count` while shifting in zeros,
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsrlvq))]
pub unsafe fn _mm_srlv_epi64(a: i64x2, count: i64x2) -> i64x2 {
    ::mem::transmute(::myarch::_mm_srlv_epi64(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 64-bit integers in `a` right by the amount specified by
/// the corresponding element in `count` while shifting in zeros,
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsrlvq))]
pub unsafe fn _mm256_srlv_epi64(a: i64x4, count: i64x4) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_srlv_epi64(::mem::transmute(a), ::mem::transmute(count)))
}

// TODO _mm256_stream_load_si256 (__m256i const* mem_addr)

/// Subtract packed 16-bit integers in `b` from packed 16-bit integers in `a`
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsubw))]
pub unsafe fn _mm256_sub_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_sub_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 32-bit integers in `b` from packed 16-bit integers in `a`
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsubd))]
pub unsafe fn _mm256_sub_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_sub_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 64-bit integers in `b` from packed 16-bit integers in `a`
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsubq))]
pub unsafe fn _mm256_sub_epi64(a: i64x4, b: i64x4) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_sub_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 8-bit integers in `b` from packed 16-bit integers in `a`
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsubb))]
pub unsafe fn _mm256_sub_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::myarch::_mm256_sub_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 16-bit integers in `b` from packed 16-bit integers in
/// `a` using saturation.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsubsw))]
pub unsafe fn _mm256_subs_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_subs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 8-bit integers in `b` from packed 8-bit integers in
/// `a` using saturation.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsubsb))]
pub unsafe fn _mm256_subs_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::myarch::_mm256_subs_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed unsigned 16-bit integers in `b` from packed 16-bit
/// integers in `a` using saturation.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsubusw))]
pub unsafe fn _mm256_subs_epu16(a: u16x16, b: u16x16) -> u16x16 {
    ::mem::transmute(::myarch::_mm256_subs_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed unsigned 8-bit integers in `b` from packed 8-bit
/// integers in `a` using saturation.
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsubusb))]
pub unsafe fn _mm256_subs_epu8(a: u8x32, b: u8x32) -> u8x32 {
    ::mem::transmute(::myarch::_mm256_subs_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave 8-bit integers from the high half of each
/// 128-bit lane in `a` and `b`.
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
/// # fn main() {
/// #     if is_x86_feature_detected!("avx2") {
/// #         #[target_feature(enable = "avx2")]
/// #         unsafe fn worker() {
/// let a = _mm256_setr_epi8(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
/// 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
/// let b = _mm256_setr_epi8(0,-1,-2,-3,-4,-5,-6,-7,-8,-9,-10,-11,-12,-13,-14,-15,
/// -16,-17,-18,-19,-20,-21,-22,-23,-24,-25,-26,-27,-28,-29,-30,-31);
///
/// let c = _mm256_unpackhi_epi8(a, b);
///
/// let expected = _mm256_setr_epi8(8,-8, 9,-9, 10,-10, 11,-11, 12,-12, 13,-13,
/// 14,-14, 15,-15, 24,-24, 25,-25, 26,-26, 27,-27, 28,-28, 29,-29, 30,-30,
/// 31,-31);
/// assert_eq!(_mm256_movemask_epi8(_mm256_cmpeq_epi8(c, expected)), !0);
///
/// #         }
/// #         unsafe { worker(); }
/// #     }
/// # }
/// ```
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpunpckhbw))]
pub unsafe fn _mm256_unpackhi_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::myarch::_mm256_unpackhi_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave 8-bit integers from the low half of each
/// 128-bit lane of `a` and `b`.
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
/// # fn main() {
/// #     if is_x86_feature_detected!("avx2") {
/// #         #[target_feature(enable = "avx2")]
/// #         unsafe fn worker() {
/// let a = _mm256_setr_epi8(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
/// 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
/// let b = _mm256_setr_epi8(0,-1,-2,-3,-4,-5,-6,-7,-8,-9,-10,-11,-12,-13,-14,-15,
/// -16,-17,-18,-19,-20,-21,-22,-23,-24,-25,-26,-27,-28,-29,-30,-31);
///
/// let c = _mm256_unpacklo_epi8(a, b);
///
/// let expected = _mm256_setr_epi8(0, 0, 1,-1, 2,-2, 3,-3, 4,-4, 5,-5, 6,-6, 7,-7,
/// 16,-16, 17,-17, 18,-18, 19,-19, 20,-20, 21,-21, 22,-22, 23,-23);
/// assert_eq!(_mm256_movemask_epi8(_mm256_cmpeq_epi8(c, expected)), !0);
///
/// #         }
/// #         unsafe { worker(); }
/// #     }
/// # }
/// ```
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpunpcklbw))]
pub unsafe fn _mm256_unpacklo_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::myarch::_mm256_unpacklo_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave 16-bit integers from the high half of each
/// 128-bit lane of `a` and `b`.
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
/// # fn main() {
/// #     if is_x86_feature_detected!("avx2") {
/// #         #[target_feature(enable = "avx2")]
/// #         unsafe fn worker() {
/// let a = _mm256_setr_epi16(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
/// let b = _mm256_setr_epi16(0,-1,-2,-3,-4,-5,-6,-7,-8,-9,-10,-11,-12,-13,-14,-15);
///
/// let c = _mm256_unpackhi_epi16(a, b);
///
/// let expected = _mm256_setr_epi16(4,-4, 5,-5, 6,-6, 7,-7, 12,-12, 13,-13, 14,-14,
/// 15,-15);
/// assert_eq!(_mm256_movemask_epi8(_mm256_cmpeq_epi8(c, expected)), !0);
///
/// #         }
/// #         unsafe { worker(); }
/// #     }
/// # }
/// ```
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpunpckhwd))]
pub unsafe fn _mm256_unpackhi_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_unpackhi_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave 16-bit integers from the low half of each
/// 128-bit lane of `a` and `b`.
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
/// # fn main() {
/// #     if is_x86_feature_detected!("avx2") {
/// #         #[target_feature(enable = "avx2")]
/// #         unsafe fn worker() {
///
/// let a = _mm256_setr_epi16(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
/// let b = _mm256_setr_epi16(0,-1,-2,-3,-4,-5,-6,-7,-8,-9,-10,-11,-12,-13,-14,-15);
///
/// let c = _mm256_unpacklo_epi16(a, b);
///
/// let expected = _mm256_setr_epi16(0, 0, 1,-1, 2,-2, 3,-3, 8,-8, 9,-9, 10,-10,
/// 11,-11);
/// assert_eq!(_mm256_movemask_epi8(_mm256_cmpeq_epi8(c, expected)), !0);
///
/// #         }
/// #         unsafe { worker(); }
/// #     }
/// # }
/// ```
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpunpcklwd))]
pub unsafe fn _mm256_unpacklo_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::myarch::_mm256_unpacklo_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave 32-bit integers from the high half of each
/// 128-bit lane of `a` and `b`.
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
/// # fn main() {
/// #     if is_x86_feature_detected!("avx2") {
/// #         #[target_feature(enable = "avx2")]
/// #         unsafe fn worker() {
/// let a = _mm256_setr_epi32(0, 1, 2, 3, 4, 5, 6, 7);
/// let b = _mm256_setr_epi32(0,-1,-2,-3,-4,-5,-6,-7);
///
/// let c = _mm256_unpackhi_epi32(a, b);
///
/// let expected = _mm256_setr_epi32(2,-2, 3,-3, 6,-6, 7,-7);
/// assert_eq!(_mm256_movemask_epi8(_mm256_cmpeq_epi8(c, expected)), !0);
///
/// #         }
/// #         unsafe { worker(); }
/// #     }
/// # }
/// ```
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vunpckhps))]
pub unsafe fn _mm256_unpackhi_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_unpackhi_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave 32-bit integers from the low half of each
/// 128-bit lane of `a` and `b`.
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
/// # fn main() {
/// #     if is_x86_feature_detected!("avx2") {
/// #         #[target_feature(enable = "avx2")]
/// #         unsafe fn worker() {
/// let a = _mm256_setr_epi32(0, 1, 2, 3, 4, 5, 6, 7);
/// let b = _mm256_setr_epi32(0,-1,-2,-3,-4,-5,-6,-7);
///
/// let c = _mm256_unpacklo_epi32(a, b);
///
/// let expected = _mm256_setr_epi32(0, 0, 1,-1, 4,-4, 5,-5);
/// assert_eq!(_mm256_movemask_epi8(_mm256_cmpeq_epi8(c, expected)), !0);
///
/// #         }
/// #         unsafe { worker(); }
/// #     }
/// # }
/// ```
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vunpcklps))]
pub unsafe fn _mm256_unpacklo_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::myarch::_mm256_unpacklo_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave 64-bit integers from the high half of each
/// 128-bit lane of `a` and `b`.
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
/// # fn main() {
/// #     if is_x86_feature_detected!("avx2") {
/// #         #[target_feature(enable = "avx2")]
/// #         unsafe fn worker() {
/// let a = _mm256_setr_epi64x(0, 1, 2, 3);
/// let b = _mm256_setr_epi64x(0,-1,-2,-3);
///
/// let c = _mm256_unpackhi_epi64(a, b);
///
/// let expected = _mm256_setr_epi64x(1,-1, 3,-3);
/// assert_eq!(_mm256_movemask_epi8(_mm256_cmpeq_epi8(c, expected)), !0);
///
/// #         }
/// #         unsafe { worker(); }
/// #     }
/// # }
/// ```
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vunpckhpd))]
pub unsafe fn _mm256_unpackhi_epi64(a: i64x4, b: i64x4) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_unpackhi_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave 64-bit integers from the low half of each
/// 128-bit lane of `a` and `b`.
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
/// # fn main() {
/// #     if is_x86_feature_detected!("avx2") {
/// #         #[target_feature(enable = "avx2")]
/// #         unsafe fn worker() {
/// let a = _mm256_setr_epi64x(0, 1, 2, 3);
/// let b = _mm256_setr_epi64x(0,-1,-2,-3);
///
/// let c = _mm256_unpacklo_epi64(a, b);
///
/// let expected = _mm256_setr_epi64x(0, 0, 2,-2);
/// assert_eq!(_mm256_movemask_epi8(_mm256_cmpeq_epi8(c, expected)), !0);
///
/// #         }
/// #         unsafe { worker(); }
/// #     }
/// # }
/// ```
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vunpcklpd))]
pub unsafe fn _mm256_unpacklo_epi64(a: i64x4, b: i64x4) -> i64x4 {
    ::mem::transmute(::myarch::_mm256_unpacklo_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise XOR of 256 bits (representing integer data)
/// in `a` and `b`
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vxorps))]
pub unsafe fn _mm256_xor_si256(a: __m256i, b: __m256i) -> __m256i {
    ::mem::transmute(::myarch::_mm256_xor_si256(::mem::transmute(a), ::mem::transmute(b)))
}

/// Extract an 8-bit integer from `a`, selected with `imm8`. Returns a 32-bit
/// integer containing the zero-extended integer data.
///
/// See [LLVM commit D20468][https://reviews.llvm.org/D20468].
#[inline]
#[target_feature(enable = "avx2")]
// This intrinsic has no corresponding instruction.
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_extract_epi8(a: i8x32, imm8: i32) -> i8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_extract_epi8(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Extract a 16-bit integer from `a`, selected with `imm8`. Returns a 32-bit
/// integer containing the zero-extended integer data.
///
/// See [LLVM commit D20468][https://reviews.llvm.org/D20468].
#[inline]
#[target_feature(enable = "avx2")]
// This intrinsic has no corresponding instruction.
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_extract_epi16(a: i16x16, imm8: i32) -> i16 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_extract_epi16(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Extract a 32-bit integer from `a`, selected with `imm8`.
#[inline]
#[target_feature(enable = "avx2")]
// This intrinsic has no corresponding instruction.
#[rustc_args_required_const(1)]
pub unsafe fn _mm256_extract_epi32(a: i32x8, imm8: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_extract_epi32(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Returns the first element of the input vector of [4 x double].
#[inline]
#[target_feature(enable = "avx2")]
//#[cfg_attr(test, assert_instr(movsd))] FIXME
pub unsafe fn _mm256_cvtsd_f64(a: f64x4) -> f64 {
    ::mem::transmute(::myarch::_mm256_cvtsd_f64(::mem::transmute(a)))
}

/// Returns the first element of the input vector of [8 x i32].
#[inline]
#[target_feature(enable = "avx2")]
//#[cfg_attr(test, assert_instr(movd))] FIXME
pub unsafe fn _mm256_cvtsi256_si32(a: __m256i) -> i32 {
    ::mem::transmute(::myarch::_mm256_cvtsi256_si32(::mem::transmute(a)))
}

