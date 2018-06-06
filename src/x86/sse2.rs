#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Add packed 8-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(paddb))]
pub unsafe fn _mm_add_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_add_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 16-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(paddw))]
pub unsafe fn _mm_add_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_add_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 32-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(paddd))]
pub unsafe fn _mm_add_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_add_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 64-bit integers in `a` and "b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(paddq))]
pub unsafe fn _mm_add_epi64(a: i64x2, b: i64x2) -> i64x2 {
    ::mem::transmute(::myarch::_mm_add_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 8-bit integers in `a` and `b` using saturation.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(paddsb))]
pub unsafe fn _mm_adds_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_adds_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 16-bit integers in `a` and `b` using saturation.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(paddsw))]
pub unsafe fn _mm_adds_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_adds_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed unsigned 8-bit integers in `a` and `b` using saturation.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(paddusb))]
pub unsafe fn _mm_adds_epu8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_adds_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed unsigned 16-bit integers in `a` and `b` using saturation.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(paddusw))]
pub unsafe fn _mm_adds_epu16(a: u16x8, b: u16x8) -> u16x8 {
    ::mem::transmute(::myarch::_mm_adds_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Average packed unsigned 8-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pavgb))]
pub unsafe fn _mm_avg_epu8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_avg_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Average packed unsigned 16-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pavgw))]
pub unsafe fn _mm_avg_epu16(a: u16x8, b: u16x8) -> u16x8 {
    ::mem::transmute(::myarch::_mm_avg_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiply and then horizontally add signed 16 bit integers in `a` and `b`.
///
/// Multiply packed signed 16-bit integers in `a` and `b`, producing
/// intermediate signed 32-bit integers. Horizontally add adjacent pairs of
/// intermediate 32-bit integers.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pmaddwd))]
pub unsafe fn _mm_madd_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_madd_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 16-bit integers in `a` and `b`, and return the packed
/// maximum values.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pmaxsw))]
pub unsafe fn _mm_max_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_max_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed unsigned 8-bit integers in `a` and `b`, and return the
/// packed maximum values.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pmaxub))]
pub unsafe fn _mm_max_epu8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_max_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 16-bit integers in `a` and `b`, and return the packed
/// minimum values.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pminsw))]
pub unsafe fn _mm_min_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_min_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed unsigned 8-bit integers in `a` and `b`, and return the
/// packed minimum values.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pminub))]
pub unsafe fn _mm_min_epu8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_min_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiply the packed 16-bit integers in `a` and `b`.
///
/// The multiplication produces intermediate 32-bit integers, and returns the
/// high 16 bits of the intermediate integers.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pmulhw))]
pub unsafe fn _mm_mulhi_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_mulhi_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiply the packed unsigned 16-bit integers in `a` and `b`.
///
/// The multiplication produces intermediate 32-bit integers, and returns the
/// high 16 bits of the intermediate integers.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pmulhuw))]
pub unsafe fn _mm_mulhi_epu16(a: u16x8, b: u16x8) -> u16x8 {
    ::mem::transmute(::myarch::_mm_mulhi_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiply the packed 16-bit integers in `a` and `b`.
///
/// The multiplication produces intermediate 32-bit integers, and returns the
/// low 16 bits of the intermediate integers.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pmullw))]
pub unsafe fn _mm_mullo_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_mullo_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiply the low unsigned 32-bit integers from each packed 64-bit element
/// in `a` and `b`.
///
/// Return the unsigned 64-bit results.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pmuludq))]
pub unsafe fn _mm_mul_epu32(a: u32x4, b: u32x4) -> u32x4 {
    ::mem::transmute(::myarch::_mm_mul_epu32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Sum the absolute differences of packed unsigned 8-bit integers.
///
/// Compute the absolute differences of packed unsigned 8-bit integers in `a`
/// and `b`, then horizontally sum each consecutive 8 differences to produce
/// two unsigned 16-bit integers, and pack these unsigned 16-bit integers in
/// the low 16 bits of 64-bit elements returned.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psadbw))]
pub unsafe fn _mm_sad_epu8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_sad_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 8-bit integers in `b` from packed 8-bit integers in `a`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psubb))]
pub unsafe fn _mm_sub_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_sub_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 16-bit integers in `b` from packed 16-bit integers in `a`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psubw))]
pub unsafe fn _mm_sub_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_sub_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 32-bit integers in `b` from packed 32-bit integers in `a`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psubd))]
pub unsafe fn _mm_sub_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_sub_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 64-bit integers in `b` from packed 64-bit integers in `a`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psubq))]
pub unsafe fn _mm_sub_epi64(a: i64x2, b: i64x2) -> i64x2 {
    ::mem::transmute(::myarch::_mm_sub_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 8-bit integers in `b` from packed 8-bit integers in `a`
/// using saturation.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psubsb))]
pub unsafe fn _mm_subs_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_subs_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 16-bit integers in `b` from packed 16-bit integers in `a`
/// using saturation.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psubsw))]
pub unsafe fn _mm_subs_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_subs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed unsigned 8-bit integers in `b` from packed unsigned 8-bit
/// integers in `a` using saturation.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psubusb))]
pub unsafe fn _mm_subs_epu8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_subs_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed unsigned 16-bit integers in `b` from packed unsigned 16-bit
/// integers in `a` using saturation.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psubusw))]
pub unsafe fn _mm_subs_epu16(a: u16x8, b: u16x8) -> u16x8 {
    ::mem::transmute(::myarch::_mm_subs_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Shift `a` left by `imm8` bytes while shifting in zeros.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pslldq, imm8 = 1))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_slli_si128(a: __m128i, imm8: i32) -> __m128i {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_slli_si128(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shift `a` left by `imm8` bytes while shifting in zeros.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pslldq, imm8 = 1))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_bslli_si128(a: __m128i, imm8: i32) -> __m128i {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_bslli_si128(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shift `a` right by `imm8` bytes while shifting in zeros.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psrldq, imm8 = 1))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_bsrli_si128(a: __m128i, imm8: i32) -> __m128i {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_bsrli_si128(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shift packed 16-bit integers in `a` left by `imm8` while shifting in zeros.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psllw, imm8 = 7))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_slli_epi16(a: i16x8, imm8: i32) -> i16x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_slli_epi16(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shift packed 16-bit integers in `a` left by `count` while shifting in
/// zeros.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psllw))]
pub unsafe fn _mm_sll_epi16(a: i16x8, count: __m128i) -> i16x8 {
    ::mem::transmute(::myarch::_mm_sll_epi16(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 32-bit integers in `a` left by `imm8` while shifting in zeros.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pslld, imm8 = 7))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_slli_epi32(a: i32x4, imm8: i32) -> i32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_slli_epi32(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shift packed 32-bit integers in `a` left by `count` while shifting in
/// zeros.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pslld))]
pub unsafe fn _mm_sll_epi32(a: i32x4, count: __m128i) -> i32x4 {
    ::mem::transmute(::myarch::_mm_sll_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 64-bit integers in `a` left by `imm8` while shifting in zeros.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psllq, imm8 = 7))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_slli_epi64(a: i64x2, imm8: i32) -> i64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_slli_epi64(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shift packed 64-bit integers in `a` left by `count` while shifting in
/// zeros.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psllq))]
pub unsafe fn _mm_sll_epi64(a: i64x2, count: __m128i) -> i64x2 {
    ::mem::transmute(::myarch::_mm_sll_epi64(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 16-bit integers in `a` right by `imm8` while shifting in sign
/// bits.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psraw, imm8 = 1))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_srai_epi16(a: i16x8, imm8: i32) -> i16x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_srai_epi16(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shift packed 16-bit integers in `a` right by `count` while shifting in sign
/// bits.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psraw))]
pub unsafe fn _mm_sra_epi16(a: i16x8, count: __m128i) -> i16x8 {
    ::mem::transmute(::myarch::_mm_sra_epi16(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 32-bit integers in `a` right by `imm8` while shifting in sign
/// bits.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psrad, imm8 = 1))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_srai_epi32(a: i32x4, imm8: i32) -> i32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_srai_epi32(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shift packed 32-bit integers in `a` right by `count` while shifting in sign
/// bits.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psrad))]
pub unsafe fn _mm_sra_epi32(a: i32x4, count: __m128i) -> i32x4 {
    ::mem::transmute(::myarch::_mm_sra_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift `a` right by `imm8` bytes while shifting in zeros.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psrldq, imm8 = 1))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_srli_si128(a: __m128i, imm8: i32) -> __m128i {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_srli_si128(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shift packed 16-bit integers in `a` right by `imm8` while shifting in
/// zeros.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psrlw, imm8 = 1))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_srli_epi16(a: i16x8, imm8: i32) -> i16x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_srli_epi16(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shift packed 16-bit integers in `a` right by `count` while shifting in
/// zeros.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psrlw))]
pub unsafe fn _mm_srl_epi16(a: i16x8, count: __m128i) -> i16x8 {
    ::mem::transmute(::myarch::_mm_srl_epi16(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 32-bit integers in `a` right by `imm8` while shifting in
/// zeros.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psrld, imm8 = 8))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_srli_epi32(a: i32x4, imm8: i32) -> i32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_srli_epi32(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shift packed 32-bit integers in `a` right by `count` while shifting in
/// zeros.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psrld))]
pub unsafe fn _mm_srl_epi32(a: i32x4, count: __m128i) -> i32x4 {
    ::mem::transmute(::myarch::_mm_srl_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

/// Shift packed 64-bit integers in `a` right by `imm8` while shifting in
/// zeros.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psrlq, imm8 = 1))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_srli_epi64(a: i64x2, imm8: i32) -> i64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_srli_epi64(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shift packed 64-bit integers in `a` right by `count` while shifting in
/// zeros.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(psrlq))]
pub unsafe fn _mm_srl_epi64(a: i64x2, count: __m128i) -> i64x2 {
    ::mem::transmute(::myarch::_mm_srl_epi64(::mem::transmute(a), ::mem::transmute(count)))
}

/// Compute the bitwise AND of 128 bits (representing integer data) in `a` and
/// `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(andps))]
pub unsafe fn _mm_and_si128(a: __m128i, b: __m128i) -> __m128i {
    ::mem::transmute(::myarch::_mm_and_si128(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise NOT of 128 bits (representing integer data) in `a` and
/// then AND with `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(andnps))]
pub unsafe fn _mm_andnot_si128(a: __m128i, b: __m128i) -> __m128i {
    ::mem::transmute(::myarch::_mm_andnot_si128(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise OR of 128 bits (representing integer data) in `a` and
/// `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(orps))]
pub unsafe fn _mm_or_si128(a: __m128i, b: __m128i) -> __m128i {
    ::mem::transmute(::myarch::_mm_or_si128(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise XOR of 128 bits (representing integer data) in `a` and
/// `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(xorps))]
pub unsafe fn _mm_xor_si128(a: __m128i, b: __m128i) -> __m128i {
    ::mem::transmute(::myarch::_mm_xor_si128(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 8-bit integers in `a` and `b` for equality.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pcmpeqb))]
pub unsafe fn _mm_cmpeq_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_cmpeq_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 16-bit integers in `a` and `b` for equality.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pcmpeqw))]
pub unsafe fn _mm_cmpeq_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_cmpeq_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 32-bit integers in `a` and `b` for equality.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pcmpeqd))]
pub unsafe fn _mm_cmpeq_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_cmpeq_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 8-bit integers in `a` and `b` for greater-than.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pcmpgtb))]
pub unsafe fn _mm_cmpgt_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_cmpgt_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 16-bit integers in `a` and `b` for greater-than.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pcmpgtw))]
pub unsafe fn _mm_cmpgt_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_cmpgt_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 32-bit integers in `a` and `b` for greater-than.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pcmpgtd))]
pub unsafe fn _mm_cmpgt_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_cmpgt_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 8-bit integers in `a` and `b` for less-than.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pcmpgtb))]
pub unsafe fn _mm_cmplt_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_cmplt_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 16-bit integers in `a` and `b` for less-than.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pcmpgtw))]
pub unsafe fn _mm_cmplt_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_cmplt_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare packed 32-bit integers in `a` and `b` for less-than.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pcmpgtd))]
pub unsafe fn _mm_cmplt_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_cmplt_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Convert the lower two packed 32-bit integers in `a` to packed
/// double-precision (64-bit) floating-point elements.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtdq2pd))]
pub unsafe fn _mm_cvtepi32_pd(a: __m128i) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cvtepi32_pd(::mem::transmute(a)))
}

/// Return `a` with its lower element replaced by `b` after converting it to
/// an `f64`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtsi2sd))]
pub unsafe fn _mm_cvtsi32_sd(a: f64x2, b: i32) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cvtsi32_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Convert packed 32-bit integers in `a` to packed single-precision (32-bit)
/// floating-point elements.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtdq2ps))]
pub unsafe fn _mm_cvtepi32_ps(a: __m128i) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cvtepi32_ps(::mem::transmute(a)))
}

/// Convert packed single-precision (32-bit) floating-point elements in `a`
/// to packed 32-bit integers.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtps2dq))]
pub unsafe fn _mm_cvtps_epi32(a: f32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_cvtps_epi32(::mem::transmute(a)))
}

/// Return a vector whose lowest element is `a` and all higher elements are
/// `0`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(all(test, target_arch = "x86_64"), assert_instr(movd))]
pub unsafe fn _mm_cvtsi32_si128(a: i32) -> __m128i {
    ::mem::transmute(::myarch::_mm_cvtsi32_si128(::mem::transmute(a)))
}

/// Return the lowest element of `a`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(movd))]
pub unsafe fn _mm_cvtsi128_si32(a: __m128i) -> i32 {
    ::mem::transmute(::myarch::_mm_cvtsi128_si32(::mem::transmute(a)))
}

/// Set packed 64-bit integers with the supplied values, from highest to
/// lowest.
#[inline]
#[target_feature(enable = "sse2")]
// no particular instruction to test
pub unsafe fn _mm_set_epi64x(e1: i64, e0: i64) -> __m128i {
    ::mem::transmute(::myarch::_mm_set_epi64x(::mem::transmute(e1), ::mem::transmute(e0)))
}

/// Set packed 32-bit integers with the supplied values.
#[inline]
#[target_feature(enable = "sse2")]
// no particular instruction to test
pub unsafe fn _mm_set_epi32(e3: i32, e2: i32, e1: i32, e0: i32) -> i32x4 {
    ::mem::transmute(::myarch::_mm_set_epi32(::mem::transmute(e3), ::mem::transmute(e2), ::mem::transmute(e1), ::mem::transmute(e0)))
}

/// Set packed 16-bit integers with the supplied values.
#[inline]
#[target_feature(enable = "sse2")]
// no particular instruction to test
pub unsafe fn _mm_set_epi16(e7: i16, e6: i16, e5: i16, e4: i16, e3: i16, e2: i16, e1: i16, e0: i16) -> i16x8 {
    ::mem::transmute(::myarch::_mm_set_epi16(::mem::transmute(e7), ::mem::transmute(e6), ::mem::transmute(e5), ::mem::transmute(e4), ::mem::transmute(e3), ::mem::transmute(e2), ::mem::transmute(e1), ::mem::transmute(e0)))
}

/// Set packed 8-bit integers with the supplied values.
#[inline]
#[target_feature(enable = "sse2")]
// no particular instruction to test
pub unsafe fn _mm_set_epi8(e15: i8, e14: i8, e13: i8, e12: i8, e11: i8, e10: i8, e9: i8, e8: i8, e7: i8, e6: i8, e5: i8, e4: i8, e3: i8, e2: i8, e1: i8, e0: i8) -> u8x16 {
    ::mem::transmute(::myarch::_mm_set_epi8(::mem::transmute(e15), ::mem::transmute(e14), ::mem::transmute(e13), ::mem::transmute(e12), ::mem::transmute(e11), ::mem::transmute(e10), ::mem::transmute(e9), ::mem::transmute(e8), ::mem::transmute(e7), ::mem::transmute(e6), ::mem::transmute(e5), ::mem::transmute(e4), ::mem::transmute(e3), ::mem::transmute(e2), ::mem::transmute(e1), ::mem::transmute(e0)))
}

/// Broadcast 64-bit integer `a` to all elements.
#[inline]
#[target_feature(enable = "sse2")]
// no particular instruction to test
pub unsafe fn _mm_set1_epi64x(a: i64) -> __m128i {
    ::mem::transmute(::myarch::_mm_set1_epi64x(::mem::transmute(a)))
}

/// Broadcast 32-bit integer `a` to all elements.
#[inline]
#[target_feature(enable = "sse2")]
// no particular instruction to test
pub unsafe fn _mm_set1_epi32(a: i32) -> i32x4 {
    ::mem::transmute(::myarch::_mm_set1_epi32(::mem::transmute(a)))
}

/// Broadcast 16-bit integer `a` to all elements.
#[inline]
#[target_feature(enable = "sse2")]
// no particular instruction to test
pub unsafe fn _mm_set1_epi16(a: i16) -> i16x8 {
    ::mem::transmute(::myarch::_mm_set1_epi16(::mem::transmute(a)))
}

/// Broadcast 8-bit integer `a` to all elements.
#[inline]
#[target_feature(enable = "sse2")]
// no particular instruction to test
pub unsafe fn _mm_set1_epi8(a: i8) -> u8x16 {
    ::mem::transmute(::myarch::_mm_set1_epi8(::mem::transmute(a)))
}

/// Set packed 32-bit integers with the supplied values in reverse order.
#[inline]
#[target_feature(enable = "sse2")]
// no particular instruction to test
pub unsafe fn _mm_setr_epi32(e3: i32, e2: i32, e1: i32, e0: i32) -> i32x4 {
    ::mem::transmute(::myarch::_mm_setr_epi32(::mem::transmute(e3), ::mem::transmute(e2), ::mem::transmute(e1), ::mem::transmute(e0)))
}

/// Set packed 16-bit integers with the supplied values in reverse order.
#[inline]
#[target_feature(enable = "sse2")]
// no particular instruction to test
pub unsafe fn _mm_setr_epi16(e7: i16, e6: i16, e5: i16, e4: i16, e3: i16, e2: i16, e1: i16, e0: i16) -> i16x8 {
    ::mem::transmute(::myarch::_mm_setr_epi16(::mem::transmute(e7), ::mem::transmute(e6), ::mem::transmute(e5), ::mem::transmute(e4), ::mem::transmute(e3), ::mem::transmute(e2), ::mem::transmute(e1), ::mem::transmute(e0)))
}

/// Set packed 8-bit integers with the supplied values in reverse order.
#[inline]
#[target_feature(enable = "sse2")]
// no particular instruction to test
pub unsafe fn _mm_setr_epi8(e15: i8, e14: i8, e13: i8, e12: i8, e11: i8, e10: i8, e9: i8, e8: i8, e7: i8, e6: i8, e5: i8, e4: i8, e3: i8, e2: i8, e1: i8, e0: i8) -> u8x16 {
    ::mem::transmute(::myarch::_mm_setr_epi8(::mem::transmute(e15), ::mem::transmute(e14), ::mem::transmute(e13), ::mem::transmute(e12), ::mem::transmute(e11), ::mem::transmute(e10), ::mem::transmute(e9), ::mem::transmute(e8), ::mem::transmute(e7), ::mem::transmute(e6), ::mem::transmute(e5), ::mem::transmute(e4), ::mem::transmute(e3), ::mem::transmute(e2), ::mem::transmute(e1), ::mem::transmute(e0)))
}

/// Returns a vector with all elements set to zero.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(xorps))]
pub unsafe fn _mm_setzero_si128() -> __m128i {
    ::mem::transmute(::myarch::_mm_setzero_si128())
}

/// Load 64-bit integer from memory into first element of returned vector.
#[inline]
#[target_feature(enable = "sse2")]
// FIXME movsd on windows
#[cfg_attr(all(test, not(windows),
               not(all(target_os = "linux", target_arch = "x86_64")),
               target_arch = "x86_64"),
           assert_instr(movq))]
pub unsafe fn _mm_loadl_epi64(mem_addr: *const __m128i) -> i64x2 {
    ::mem::transmute(::myarch::_mm_loadl_epi64(::mem::transmute(mem_addr)))
}

/// Load 128-bits of integer data from memory into a new vector.
///
/// `mem_addr` must be aligned on a 16-byte boundary.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(movaps))]
pub unsafe fn _mm_load_si128(mem_addr: *const __m128i) -> __m128i {
    ::mem::transmute(::myarch::_mm_load_si128(::mem::transmute(mem_addr)))
}

/// Load 128-bits of integer data from memory into a new vector.
///
/// `mem_addr` does not need to be aligned on any particular boundary.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(movups))]
pub unsafe fn _mm_loadu_si128(mem_addr: *const __m128i) -> __m128i {
    ::mem::transmute(::myarch::_mm_loadu_si128(::mem::transmute(mem_addr)))
}

/// Return a vector where the low element is extracted from `a` and its upper
/// element is zero.
#[inline]
#[target_feature(enable = "sse2")]
// FIXME movd on windows, movd on i686
#[cfg_attr(all(test, not(windows), target_arch = "x86_64"),
           assert_instr(movq))]
pub unsafe fn _mm_move_epi64(a: i64x2) -> i64x2 {
    ::mem::transmute(::myarch::_mm_move_epi64(::mem::transmute(a)))
}

/// Convert packed 16-bit integers from `a` and `b` to packed 8-bit integers
/// using signed saturation.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(packsswb))]
pub unsafe fn _mm_packs_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_packs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Convert packed 32-bit integers from `a` and `b` to packed 16-bit integers
/// using signed saturation.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(packssdw))]
pub unsafe fn _mm_packs_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_packs_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Convert packed 16-bit integers from `a` and `b` to packed 8-bit integers
/// using unsigned saturation.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(packuswb))]
pub unsafe fn _mm_packus_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_packus_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return the `imm8` element of `a`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pextrw, imm8 = 9))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_extract_epi16(a: i16x8, imm8: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_extract_epi16(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Return a new vector where the `imm8` element of `a` is replaced with `i`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pinsrw, imm8 = 9))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_insert_epi16(a: i16x8, i: i32, imm8: i32) -> i16x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_insert_epi16(::mem::transmute(a), ::mem::transmute(i), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Return a mask of the most significant bit of each element in `a`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pmovmskb))]
pub unsafe fn _mm_movemask_epi8(a: u8x16) -> i32 {
    ::mem::transmute(::myarch::_mm_movemask_epi8(::mem::transmute(a)))
}

/// Shuffle 32-bit integers in `a` using the control in `imm8`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pshufd, imm8 = 9))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_shuffle_epi32(a: i32x4, imm8: i32) -> i32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_shuffle_epi32(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shuffle 16-bit integers in the high 64 bits of `a` using the control in
/// `imm8`.
///
/// Put the results in the high 64 bits of the returned vector, with the low 64
/// bits being copied from from `a`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pshufhw, imm8 = 9))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_shufflehi_epi16(a: i16x8, imm8: i32) -> i16x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_shufflehi_epi16(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Shuffle 16-bit integers in the low 64 bits of `a` using the control in
/// `imm8`.
///
/// Put the results in the low 64 bits of the returned vector, with the high 64
/// bits being copied from from `a`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(pshuflw, imm8 = 9))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_shufflelo_epi16(a: i16x8, imm8: i32) -> i16x8 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_shufflelo_epi16(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Unpack and interleave 8-bit integers from the high half of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(punpckhbw))]
pub unsafe fn _mm_unpackhi_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_unpackhi_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave 16-bit integers from the high half of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(punpckhwd))]
pub unsafe fn _mm_unpackhi_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_unpackhi_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave 32-bit integers from the high half of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(unpckhps))]
pub unsafe fn _mm_unpackhi_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_unpackhi_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave 64-bit integers from the high half of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(unpckhpd))]
pub unsafe fn _mm_unpackhi_epi64(a: i64x2, b: i64x2) -> i64x2 {
    ::mem::transmute(::myarch::_mm_unpackhi_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave 8-bit integers from the low half of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(punpcklbw))]
pub unsafe fn _mm_unpacklo_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_unpacklo_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave 16-bit integers from the low half of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(punpcklwd))]
pub unsafe fn _mm_unpacklo_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_unpacklo_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave 32-bit integers from the low half of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(unpcklps))]
pub unsafe fn _mm_unpacklo_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_unpacklo_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpack and interleave 64-bit integers from the low half of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(movlhps))]
pub unsafe fn _mm_unpacklo_epi64(a: i64x2, b: i64x2) -> i64x2 {
    ::mem::transmute(::myarch::_mm_unpacklo_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by the sum of the
/// low elements of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(addsd))]
pub unsafe fn _mm_add_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_add_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed double-precision (64-bit) floating-point elements in `a` and
/// `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(addpd))]
pub unsafe fn _mm_add_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_add_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by the result of
/// diving the lower element of `a` by the lower element of `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(divsd))]
pub unsafe fn _mm_div_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_div_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Divide packed double-precision (64-bit) floating-point elements in `a` by
/// packed elements in `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(divpd))]
pub unsafe fn _mm_div_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_div_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by the maximum
/// of the lower elements of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(maxsd))]
pub unsafe fn _mm_max_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_max_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the maximum values from corresponding elements in
/// `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(maxpd))]
pub unsafe fn _mm_max_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_max_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by the minimum
/// of the lower elements of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(minsd))]
pub unsafe fn _mm_min_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_min_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the minimum values from corresponding elements in
/// `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(minpd))]
pub unsafe fn _mm_min_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_min_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by multiplying the
/// low elements of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(mulsd))]
pub unsafe fn _mm_mul_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_mul_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiply packed double-precision (64-bit) floating-point elements in `a`
/// and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(mulpd))]
pub unsafe fn _mm_mul_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_mul_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by the square
/// root of the lower element `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(sqrtsd))]
pub unsafe fn _mm_sqrt_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_sqrt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the square root of each of the values in `a`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(sqrtpd))]
pub unsafe fn _mm_sqrt_pd(a: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_sqrt_pd(::mem::transmute(a)))
}

/// Return a new vector with the low element of `a` replaced by subtracting the
/// low element by `b` from the low element of `a`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(subsd))]
pub unsafe fn _mm_sub_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_sub_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed double-precision (64-bit) floating-point elements in `b`
/// from `a`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(subpd))]
pub unsafe fn _mm_sub_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_sub_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise AND of packed double-precision (64-bit) floating-point
/// elements in `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(andps))]
pub unsafe fn _mm_and_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_and_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise NOT of `a` and then AND with `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(andnps))]
pub unsafe fn _mm_andnot_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_andnot_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise OR of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(orps))]
pub unsafe fn _mm_or_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_or_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the bitwise OR of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(xorps))]
pub unsafe fn _mm_xor_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_xor_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by the equality
/// comparison of the lower elements of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpeqsd))]
pub unsafe fn _mm_cmpeq_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpeq_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by the less-than
/// comparison of the lower elements of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpltsd))]
pub unsafe fn _mm_cmplt_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmplt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by the
/// less-than-or-equal comparison of the lower elements of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmplesd))]
pub unsafe fn _mm_cmple_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmple_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by the
/// greater-than comparison of the lower elements of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpltsd))]
pub unsafe fn _mm_cmpgt_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpgt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by the
/// greater-than-or-equal comparison of the lower elements of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmplesd))]
pub unsafe fn _mm_cmpge_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpge_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by the result
/// of comparing both of the lower elements of `a` and `b` to `NaN`. If
/// neither are equal to `NaN` then `0xFFFFFFFFFFFFFFFF` is used and `0`
/// otherwise.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpordsd))]
pub unsafe fn _mm_cmpord_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpord_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by the result of
/// comparing both of the lower elements of `a` and `b` to `NaN`. If either is
/// equal to `NaN` then `0xFFFFFFFFFFFFFFFF` is used and `0` otherwise.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpunordsd))]
pub unsafe fn _mm_cmpunord_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpunord_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by the not-equal
/// comparison of the lower elements of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpneqsd))]
pub unsafe fn _mm_cmpneq_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpneq_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by the
/// not-less-than comparison of the lower elements of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpnltsd))]
pub unsafe fn _mm_cmpnlt_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpnlt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by the
/// not-less-than-or-equal comparison of the lower elements of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpnlesd))]
pub unsafe fn _mm_cmpnle_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpnle_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by the
/// not-greater-than comparison of the lower elements of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpnltsd))]
pub unsafe fn _mm_cmpngt_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpngt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return a new vector with the low element of `a` replaced by the
/// not-greater-than-or-equal comparison of the lower elements of `a` and `b`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpnlesd))]
pub unsafe fn _mm_cmpnge_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpnge_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare corresponding elements in `a` and `b` for equality.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpeqpd))]
pub unsafe fn _mm_cmpeq_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpeq_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare corresponding elements in `a` and `b` for less-than.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpltpd))]
pub unsafe fn _mm_cmplt_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmplt_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare corresponding elements in `a` and `b` for less-than-or-equal
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmplepd))]
pub unsafe fn _mm_cmple_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmple_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare corresponding elements in `a` and `b` for greater-than.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpltpd))]
pub unsafe fn _mm_cmpgt_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpgt_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare corresponding elements in `a` and `b` for greater-than-or-equal.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmplepd))]
pub unsafe fn _mm_cmpge_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpge_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare corresponding elements in `a` and `b` to see if neither is `NaN`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpordpd))]
pub unsafe fn _mm_cmpord_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpord_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare corresponding elements in `a` and `b` to see if either is `NaN`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpunordpd))]
pub unsafe fn _mm_cmpunord_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpunord_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare corresponding elements in `a` and `b` for not-equal.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpneqpd))]
pub unsafe fn _mm_cmpneq_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpneq_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare corresponding elements in `a` and `b` for not-less-than.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpnltpd))]
pub unsafe fn _mm_cmpnlt_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpnlt_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare corresponding elements in `a` and `b` for not-less-than-or-equal.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpnlepd))]
pub unsafe fn _mm_cmpnle_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpnle_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare corresponding elements in `a` and `b` for not-greater-than.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpnltpd))]
pub unsafe fn _mm_cmpngt_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpngt_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare corresponding elements in `a` and `b` for
/// not-greater-than-or-equal.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cmpnlepd))]
pub unsafe fn _mm_cmpnge_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cmpnge_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lower element of `a` and `b` for equality.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(comisd))]
pub unsafe fn _mm_comieq_sd(a: f64x2, b: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_comieq_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lower element of `a` and `b` for less-than.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(comisd))]
pub unsafe fn _mm_comilt_sd(a: f64x2, b: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_comilt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lower element of `a` and `b` for less-than-or-equal.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(comisd))]
pub unsafe fn _mm_comile_sd(a: f64x2, b: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_comile_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lower element of `a` and `b` for greater-than.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(comisd))]
pub unsafe fn _mm_comigt_sd(a: f64x2, b: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_comigt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lower element of `a` and `b` for greater-than-or-equal.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(comisd))]
pub unsafe fn _mm_comige_sd(a: f64x2, b: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_comige_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lower element of `a` and `b` for not-equal.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(comisd))]
pub unsafe fn _mm_comineq_sd(a: f64x2, b: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_comineq_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lower element of `a` and `b` for equality.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(ucomisd))]
pub unsafe fn _mm_ucomieq_sd(a: f64x2, b: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_ucomieq_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lower element of `a` and `b` for less-than.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(ucomisd))]
pub unsafe fn _mm_ucomilt_sd(a: f64x2, b: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_ucomilt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lower element of `a` and `b` for less-than-or-equal.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(ucomisd))]
pub unsafe fn _mm_ucomile_sd(a: f64x2, b: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_ucomile_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lower element of `a` and `b` for greater-than.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(ucomisd))]
pub unsafe fn _mm_ucomigt_sd(a: f64x2, b: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_ucomigt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lower element of `a` and `b` for greater-than-or-equal.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(ucomisd))]
pub unsafe fn _mm_ucomige_sd(a: f64x2, b: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_ucomige_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compare the lower element of `a` and `b` for not-equal.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(ucomisd))]
pub unsafe fn _mm_ucomineq_sd(a: f64x2, b: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_ucomineq_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Convert packed double-precision (64-bit) floating-point elements in "a" to
/// packed single-precision (32-bit) floating-point elements
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtpd2ps))]
pub unsafe fn _mm_cvtpd_ps(a: f64x2) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cvtpd_ps(::mem::transmute(a)))
}

/// Convert packed single-precision (32-bit) floating-point elements in `a` to
/// packed
/// double-precision (64-bit) floating-point elements.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtps2pd))]
pub unsafe fn _mm_cvtps_pd(a: f32x4) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cvtps_pd(::mem::transmute(a)))
}

/// Convert packed double-precision (64-bit) floating-point elements in `a` to
/// packed 32-bit integers.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtpd2dq))]
pub unsafe fn _mm_cvtpd_epi32(a: f64x2) -> i32x4 {
    ::mem::transmute(::myarch::_mm_cvtpd_epi32(::mem::transmute(a)))
}

/// Convert the lower double-precision (64-bit) floating-point element in a to
/// a 32-bit integer.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtsd2si))]
pub unsafe fn _mm_cvtsd_si32(a: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_cvtsd_si32(::mem::transmute(a)))
}

/// Convert the lower double-precision (64-bit) floating-point element in `b`
/// to a single-precision (32-bit) floating-point element, store the result in
/// the lower element of the return value, and copy the upper element from `a`
/// to the upper element the return value.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtsd2ss))]
pub unsafe fn _mm_cvtsd_ss(a: f32x4, b: f64x2) -> f32x4 {
    ::mem::transmute(::myarch::_mm_cvtsd_ss(::mem::transmute(a), ::mem::transmute(b)))
}

/// Return the lower double-precision (64-bit) floating-point element of "a".
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtsd_f64(a: f64x2) -> f64 {
    ::mem::transmute(::myarch::_mm_cvtsd_f64(::mem::transmute(a)))
}

/// Convert the lower single-precision (32-bit) floating-point element in `b`
/// to a double-precision (64-bit) floating-point element, store the result in
/// the lower element of the return value, and copy the upper element from `a`
/// to the upper element the return value.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtss2sd))]
pub unsafe fn _mm_cvtss_sd(a: f64x2, b: f32x4) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cvtss_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Convert packed double-precision (64-bit) floating-point elements in `a` to
/// packed 32-bit integers with truncation.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvttpd2dq))]
pub unsafe fn _mm_cvttpd_epi32(a: f64x2) -> i32x4 {
    ::mem::transmute(::myarch::_mm_cvttpd_epi32(::mem::transmute(a)))
}

/// Convert the lower double-precision (64-bit) floating-point element in `a`
/// to a 32-bit integer with truncation.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvttsd2si))]
pub unsafe fn _mm_cvttsd_si32(a: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_cvttsd_si32(::mem::transmute(a)))
}

/// Convert packed single-precision (32-bit) floating-point elements in `a` to
/// packed 32-bit integers with truncation.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvttps2dq))]
pub unsafe fn _mm_cvttps_epi32(a: f32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_cvttps_epi32(::mem::transmute(a)))
}

/// Copy double-precision (64-bit) floating-point element `a` to the lower
/// element of the packed 64-bit return value.
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_set_sd(a: f64) -> f64x2 {
    ::mem::transmute(::myarch::_mm_set_sd(::mem::transmute(a)))
}

/// Broadcast double-precision (64-bit) floating-point value a to all elements
/// of the return value.
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_set1_pd(a: f64) -> f64x2 {
    ::mem::transmute(::myarch::_mm_set1_pd(::mem::transmute(a)))
}

/// Broadcast double-precision (64-bit) floating-point value a to all elements
/// of the return value.
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_set_pd1(a: f64) -> f64x2 {
    ::mem::transmute(::myarch::_mm_set_pd1(::mem::transmute(a)))
}

/// Set packed double-precision (64-bit) floating-point elements in the return
/// value with the supplied values.
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_set_pd(a: f64, b: f64) -> f64x2 {
    ::mem::transmute(::myarch::_mm_set_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Set packed double-precision (64-bit) floating-point elements in the return
/// value with the supplied values in reverse order.
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_setr_pd(a: f64, b: f64) -> f64x2 {
    ::mem::transmute(::myarch::_mm_setr_pd(::mem::transmute(a), ::mem::transmute(b)))
}

 // FIXME xorpd expected
pub unsafe fn _mm_setzero_pd() -> f64x2 {
    ::mem::transmute(::myarch::_mm_setzero_pd())
}

/// Return a mask of the most significant bit of each element in `a`.
///
/// The mask is stored in the 2 least significant bits of the return value.
/// All other bits are set to `0`.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(movmskpd))]
pub unsafe fn _mm_movemask_pd(a: f64x2) -> i32 {
    ::mem::transmute(::myarch::_mm_movemask_pd(::mem::transmute(a)))
}

/// Load 128-bits (composed of 2 packed double-precision (64-bit)
/// floating-point elements) from memory into the returned vector.
/// `mem_addr` must be aligned on a 16-byte boundary or a general-protection
/// exception may be generated.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(movaps))]
pub unsafe fn _mm_load_pd(mem_addr: *const f64) -> f64x2 {
    ::mem::transmute(::myarch::_mm_load_pd(::mem::transmute(mem_addr)))
}

/// Loads a 64-bit double-precision value to the low element of a
/// 128-bit integer vector and clears the upper element.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(movsd))]
pub unsafe fn _mm_load_sd(mem_addr: *const f64) -> f64x2 {
    ::mem::transmute(::myarch::_mm_load_sd(::mem::transmute(mem_addr)))
}

/// Loads a double-precision value into the high-order bits of a 128-bit
/// vector of [2 x double]. The low-order bits are copied from the low-order
/// bits of the first operand.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(movhpd))]
pub unsafe fn _mm_loadh_pd(a: f64x2, mem_addr: *const f64) -> f64x2 {
    ::mem::transmute(::myarch::_mm_loadh_pd(::mem::transmute(a), ::mem::transmute(mem_addr)))
}

/// Loads a double-precision value into the low-order bits of a 128-bit
/// vector of [2 x double]. The high-order bits are copied from the
/// high-order bits of the first operand.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(movlpd))]
pub unsafe fn _mm_loadl_pd(a: f64x2, mem_addr: *const f64) -> f64x2 {
    ::mem::transmute(::myarch::_mm_loadl_pd(::mem::transmute(a), ::mem::transmute(mem_addr)))
}

/// Load a double-precision (64-bit) floating-point element from memory
/// into both elements of returned vector.
#[inline]
#[target_feature(enable = "sse2")]
// #[cfg_attr(test, assert_instr(movapd))] // FIXME LLVM uses different codegen
pub unsafe fn _mm_load1_pd(mem_addr: *const f64) -> f64x2 {
    ::mem::transmute(::myarch::_mm_load1_pd(::mem::transmute(mem_addr)))
}

/// Load a double-precision (64-bit) floating-point element from memory
/// into both elements of returned vector.
#[inline]
#[target_feature(enable = "sse2")]
// #[cfg_attr(test, assert_instr(movapd))] // FIXME same as _mm_load1_pd
pub unsafe fn _mm_load_pd1(mem_addr: *const f64) -> f64x2 {
    ::mem::transmute(::myarch::_mm_load_pd1(::mem::transmute(mem_addr)))
}

/// Load 2 double-precision (64-bit) floating-point elements from memory into
/// the returned vector in reverse order. `mem_addr` must be aligned on a
/// 16-byte boundary or a general-protection exception may be generated.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(movapd))]
pub unsafe fn _mm_loadr_pd(mem_addr: *const f64) -> f64x2 {
    ::mem::transmute(::myarch::_mm_loadr_pd(::mem::transmute(mem_addr)))
}

/// Load 128-bits (composed of 2 packed double-precision (64-bit)
/// floating-point elements) from memory into the returned vector.
/// `mem_addr` does not need to be aligned on any particular boundary.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(movups))]
pub unsafe fn _mm_loadu_pd(mem_addr: *const f64) -> f64x2 {
    ::mem::transmute(::myarch::_mm_loadu_pd(::mem::transmute(mem_addr)))
}

/// Constructs a 128-bit floating-point vector of [2 x double] from two
/// 128-bit vector parameters of [2 x double], using the immediate-value
/// parameter as a specifier.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(shufpd, imm8 = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_shuffle_pd(a: f64x2, b: f64x2, imm8: i32) -> f64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_shuffle_pd(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

/// Constructs a 128-bit floating-point vector of [2 x double]. The lower
/// 64 bits are set to the lower 64 bits of the second parameter. The upper
/// 64 bits are set to the upper 64 bits of the first parameter.
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(movsd))]
pub unsafe fn _mm_move_sd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_move_sd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Casts a 128-bit floating-point vector of [2 x double] into a 128-bit
/// floating-point vector of [4 x float].
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_castpd_ps(a: f64x2) -> f32x4 {
    ::mem::transmute(::myarch::_mm_castpd_ps(::mem::transmute(a)))
}

/// Casts a 128-bit floating-point vector of [2 x double] into a 128-bit
/// integer vector.
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_castpd_si128(a: f64x2) -> __m128i {
    ::mem::transmute(::myarch::_mm_castpd_si128(::mem::transmute(a)))
}

/// Casts a 128-bit floating-point vector of [4 x float] into a 128-bit
/// floating-point vector of [2 x double].
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_castps_pd(a: f32x4) -> f64x2 {
    ::mem::transmute(::myarch::_mm_castps_pd(::mem::transmute(a)))
}

/// Casts a 128-bit floating-point vector of [4 x float] into a 128-bit
/// integer vector.
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_castps_si128(a: f32x4) -> __m128i {
    ::mem::transmute(::myarch::_mm_castps_si128(::mem::transmute(a)))
}

/// Casts a 128-bit integer vector into a 128-bit floating-point vector
/// of [2 x double].
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_castsi128_pd(a: __m128i) -> f64x2 {
    ::mem::transmute(::myarch::_mm_castsi128_pd(::mem::transmute(a)))
}

/// Casts a 128-bit integer vector into a 128-bit floating-point vector
/// of [4 x float].
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_castsi128_ps(a: __m128i) -> f32x4 {
    ::mem::transmute(::myarch::_mm_castsi128_ps(::mem::transmute(a)))
}

/// Return vector of type __m128d with undefined elements.
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_undefined_pd() -> f64x2 {
    ::mem::transmute(::myarch::_mm_undefined_pd())
}

/// Return vector of type __m128i with undefined elements.
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_undefined_si128() -> __m128i {
    ::mem::transmute(::myarch::_mm_undefined_si128())
}

/// The resulting `__m128d` element is composed by the low-order values of
/// the two `__m128d` interleaved input elements, i.e.:
///
/// * The [127:64] bits are copied from the [127:64] bits of the second input
/// * The [63:0] bits are copied from the [127:64] bits of the first input
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(unpckhpd))]
pub unsafe fn _mm_unpackhi_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_unpackhi_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// The resulting `__m128d` element is composed by the high-order values of
/// the two `__m128d` interleaved input elements, i.e.:
///
/// * The [127:64] bits are copied from the [63:0] bits of the second input
/// * The [63:0] bits are copied from the [63:0] bits of the first input
#[inline]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(movlhps))]
pub unsafe fn _mm_unpacklo_pd(a: f64x2, b: f64x2) -> f64x2 {
    ::mem::transmute(::myarch::_mm_unpacklo_pd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Adds two signed or unsigned 64-bit integer values, returning the
/// lower 64 bits of the sum.
#[inline]
#[target_feature(enable = "sse2,mmx")]
#[cfg_attr(test, assert_instr(paddq))]
pub unsafe fn _mm_add_si64(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_add_si64(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiplies 32-bit unsigned integer values contained in the lower bits
/// of the two 64-bit integer vectors and returns the 64-bit unsigned
/// product.
#[inline]
#[target_feature(enable = "sse2,mmx")]
#[cfg_attr(test, assert_instr(pmuludq))]
pub unsafe fn _mm_mul_su32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_mul_su32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtracts signed or unsigned 64-bit integer values and writes the
/// difference to the corresponding bits in the destination.
#[inline]
#[target_feature(enable = "sse2,mmx")]
#[cfg_attr(test, assert_instr(psubq))]
pub unsafe fn _mm_sub_si64(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_sub_si64(::mem::transmute(a), ::mem::transmute(b)))
}

/// Converts the two signed 32-bit integer elements of a 64-bit vector of
/// [2 x i32] into two double-precision floating-point values, returned in a
/// 128-bit vector of [2 x double].
#[inline]
#[target_feature(enable = "sse2,mmx")]
#[cfg_attr(test, assert_instr(cvtpi2pd))]
pub unsafe fn _mm_cvtpi32_pd(a: __m64) -> f64x2 {
    ::mem::transmute(::myarch::_mm_cvtpi32_pd(::mem::transmute(a)))
}

/// Initializes both 64-bit values in a 128-bit vector of [2 x i64] with
/// the specified 64-bit integer values.
#[inline]
#[target_feature(enable = "sse2,mmx")]
// no particular instruction to test
pub unsafe fn _mm_set_epi64(e1: __m64, e0: __m64) -> i64x2 {
    ::mem::transmute(::myarch::_mm_set_epi64(::mem::transmute(e1), ::mem::transmute(e0)))
}

/// Initializes both values in a 128-bit vector of [2 x i64] with the
/// specified 64-bit value.
#[inline]
#[target_feature(enable = "sse2,mmx")]
// no particular instruction to test
pub unsafe fn _mm_set1_epi64(a: __m64) -> i64x2 {
    ::mem::transmute(::myarch::_mm_set1_epi64(::mem::transmute(a)))
}

/// Constructs a 128-bit integer vector, initialized in reverse order
/// with the specified 64-bit integral values.
#[inline]
#[target_feature(enable = "sse2,mmx")]
// no particular instruction to test
pub unsafe fn _mm_setr_epi64(e1: __m64, e0: __m64) -> i64x2 {
    ::mem::transmute(::myarch::_mm_setr_epi64(::mem::transmute(e1), ::mem::transmute(e0)))
}

/// Returns the lower 64 bits of a 128-bit integer vector as a 64-bit
/// integer.
#[inline]
#[target_feature(enable = "sse2,mmx")]
// #[cfg_attr(test, assert_instr(movdq2q))] // FIXME: llvm codegens wrong
// instr?
pub unsafe fn _mm_movepi64_pi64(a: __m128i) -> __m64 {
    ::mem::transmute(::myarch::_mm_movepi64_pi64(::mem::transmute(a)))
}

/// Moves the 64-bit operand to a 128-bit integer vector, zeroing the
/// upper bits.
#[inline]
#[target_feature(enable = "sse2,mmx")]
// #[cfg_attr(test, assert_instr(movq2dq))] // FIXME: llvm codegens wrong
// instr?
pub unsafe fn _mm_movpi64_epi64(a: __m64) -> i64x2 {
    ::mem::transmute(::myarch::_mm_movpi64_epi64(::mem::transmute(a)))
}

/// Converts the two double-precision floating-point elements of a
/// 128-bit vector of [2 x double] into two signed 32-bit integer values,
/// returned in a 64-bit vector of [2 x i32].
#[inline]
#[target_feature(enable = "sse2,mmx")]
#[cfg_attr(test, assert_instr(cvtpd2pi))]
pub unsafe fn _mm_cvtpd_pi32(a: f64x2) -> __m64 {
    ::mem::transmute(::myarch::_mm_cvtpd_pi32(::mem::transmute(a)))
}

/// Converts the two double-precision floating-point elements of a
/// 128-bit vector of [2 x double] into two signed 32-bit integer values,
/// returned in a 64-bit vector of [2 x i32].
/// If the result of either conversion is inexact, the result is truncated
/// (rounded towards zero) regardless of the current MXCSR setting.
#[inline]
#[target_feature(enable = "sse2,mmx")]
#[cfg_attr(test, assert_instr(cvttpd2pi))]
pub unsafe fn _mm_cvttpd_pi32(a: f64x2) -> __m64 {
    ::mem::transmute(::myarch::_mm_cvttpd_pi32(::mem::transmute(a)))
}

