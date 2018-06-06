#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Compute the absolute value of packed 8-bit signed integers in `a` and
/// return the unsigned results.
#[inline]
#[target_feature(enable = "ssse3")]
#[cfg_attr(test, assert_instr(pabsb))]
pub unsafe fn _mm_abs_epi8(a: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_abs_epi8(::mem::transmute(a)))
}

/// Compute the absolute value of each of the packed 16-bit signed integers in
/// `a` and
/// return the 16-bit unsigned integer
#[inline]
#[target_feature(enable = "ssse3")]
#[cfg_attr(test, assert_instr(pabsw))]
pub unsafe fn _mm_abs_epi16(a: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_abs_epi16(::mem::transmute(a)))
}

/// Compute the absolute value of each of the packed 32-bit signed integers in
/// `a` and
/// return the 32-bit unsigned integer
#[inline]
#[target_feature(enable = "ssse3")]
#[cfg_attr(test, assert_instr(pabsd))]
pub unsafe fn _mm_abs_epi32(a: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_abs_epi32(::mem::transmute(a)))
}

/// Shuffle bytes from `a` according to the content of `b`.
///
/// The last 4 bits of each byte of `b` are used as addresses
/// into the 16 bytes of `a`.
///
/// In addition, if the highest significant bit of a byte of `b`
/// is set, the respective destination byte is set to 0.
///
/// Picturing `a` and `b` as `[u8; 16]`, `_mm_shuffle_epi8` is
/// logically equivalent to:
///
/// ```
/// fn mm_shuffle_epi8(a: [u8; 16], b: [u8; 16]) -> [u8; 16] {
///     let mut r = [0u8; 16];
///     for i in 0..16 {
///         // if the most significant bit of b is set,
///         // then the destination byte is set to 0.
///         if b[i] & 0x80 == 0u8 {
///             r[i] = a[(b[i] % 16) as usize];
///         }
///     }
///     r
/// }
/// ```
#[inline]
#[target_feature(enable = "ssse3")]
#[cfg_attr(test, assert_instr(pshufb))]
pub unsafe fn _mm_shuffle_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_shuffle_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Concatenate 16-byte blocks in `a` and `b` into a 32-byte temporary result,
/// shift the result right by `n` bytes, and return the low 16 bytes.
#[inline]
#[target_feature(enable = "ssse3")]
#[cfg_attr(test, assert_instr(palignr, n = 15))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_alignr_epi8(a: u8x16, b: u8x16, n: i32) -> u8x16 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_alignr_epi8(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(n, call))
}

/// Horizontally add the adjacent pairs of values contained in 2 packed
/// 128-bit vectors of [8 x i16].
#[inline]
#[target_feature(enable = "ssse3")]
#[cfg_attr(test, assert_instr(phaddw))]
pub unsafe fn _mm_hadd_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_hadd_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontally add the adjacent pairs of values contained in 2 packed
/// 128-bit vectors of [8 x i16]. Positive sums greater than 7FFFh are
/// saturated to 7FFFh. Negative sums less than 8000h are saturated to 8000h.
#[inline]
#[target_feature(enable = "ssse3")]
#[cfg_attr(test, assert_instr(phaddsw))]
pub unsafe fn _mm_hadds_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_hadds_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontally add the adjacent pairs of values contained in 2 packed
/// 128-bit vectors of [4 x i32].
#[inline]
#[target_feature(enable = "ssse3")]
#[cfg_attr(test, assert_instr(phaddd))]
pub unsafe fn _mm_hadd_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_hadd_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontally subtract the adjacent pairs of values contained in 2
/// packed 128-bit vectors of [8 x i16].
#[inline]
#[target_feature(enable = "ssse3")]
#[cfg_attr(test, assert_instr(phsubw))]
pub unsafe fn _mm_hsub_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_hsub_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontally subtract the adjacent pairs of values contained in 2
/// packed 128-bit vectors of [8 x i16]. Positive differences greater than
/// 7FFFh are saturated to 7FFFh. Negative differences less than 8000h are
/// saturated to 8000h.
#[inline]
#[target_feature(enable = "ssse3")]
#[cfg_attr(test, assert_instr(phsubsw))]
pub unsafe fn _mm_hsubs_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_hsubs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontally subtract the adjacent pairs of values contained in 2
/// packed 128-bit vectors of [4 x i32].
#[inline]
#[target_feature(enable = "ssse3")]
#[cfg_attr(test, assert_instr(phsubd))]
pub unsafe fn _mm_hsub_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_hsub_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiply corresponding pairs of packed 8-bit unsigned integer
/// values contained in the first source operand and packed 8-bit signed
/// integer values contained in the second source operand, add pairs of
/// contiguous products with signed saturation, and writes the 16-bit sums to
/// the corresponding bits in the destination.
#[inline]
#[target_feature(enable = "ssse3")]
#[cfg_attr(test, assert_instr(pmaddubsw))]
pub unsafe fn _mm_maddubs_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_maddubs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiply packed 16-bit signed integer values, truncate the 32-bit
/// product to the 18 most significant bits by right-shifting, round the
/// truncated value by adding 1, and write bits [16:1] to the destination.
#[inline]
#[target_feature(enable = "ssse3")]
#[cfg_attr(test, assert_instr(pmulhrsw))]
pub unsafe fn _mm_mulhrs_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_mulhrs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Negate packed 8-bit integers in `a` when the corresponding signed 8-bit
/// integer in `b` is negative, and return the result.
/// Elements in result are zeroed out when the corresponding element in `b`
/// is zero.
#[inline]
#[target_feature(enable = "ssse3")]
#[cfg_attr(test, assert_instr(psignb))]
pub unsafe fn _mm_sign_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::myarch::_mm_sign_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Negate packed 16-bit integers in `a` when the corresponding signed 16-bit
/// integer in `b` is negative, and return the results.
/// Elements in result are zeroed out when the corresponding element in `b`
/// is zero.
#[inline]
#[target_feature(enable = "ssse3")]
#[cfg_attr(test, assert_instr(psignw))]
pub unsafe fn _mm_sign_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::myarch::_mm_sign_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Negate packed 32-bit integers in `a` when the corresponding signed 32-bit
/// integer in `b` is negative, and return the results.
/// Element in result are zeroed out when the corresponding element in `b`
/// is zero.
#[inline]
#[target_feature(enable = "ssse3")]
#[cfg_attr(test, assert_instr(psignd))]
pub unsafe fn _mm_sign_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::myarch::_mm_sign_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compute the absolute value of packed 8-bit integers in `a` and
/// return the unsigned results.
#[inline]
#[target_feature(enable = "ssse3,mmx")]
#[cfg_attr(test, assert_instr(pabsb))]
pub unsafe fn _mm_abs_pi8(a: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_abs_pi8(::mem::transmute(a)))
}

/// Compute the absolute value of packed 8-bit integers in `a`, and return the
/// unsigned results.
#[inline]
#[target_feature(enable = "ssse3,mmx")]
#[cfg_attr(test, assert_instr(pabsw))]
pub unsafe fn _mm_abs_pi16(a: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_abs_pi16(::mem::transmute(a)))
}

/// Compute the absolute value of packed 32-bit integers in `a`, and return the
/// unsigned results.
#[inline]
#[target_feature(enable = "ssse3,mmx")]
#[cfg_attr(test, assert_instr(pabsd))]
pub unsafe fn _mm_abs_pi32(a: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_abs_pi32(::mem::transmute(a)))
}

/// Shuffle packed 8-bit integers in `a` according to shuffle control mask in
/// the corresponding 8-bit element of `b`, and return the results
#[inline]
#[target_feature(enable = "ssse3,mmx")]
#[cfg_attr(test, assert_instr(pshufb))]
pub unsafe fn _mm_shuffle_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_shuffle_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Concatenates the two 64-bit integer vector operands, and right-shifts
/// the result by the number of bytes specified in the immediate operand.
#[inline]
#[target_feature(enable = "ssse3,mmx")]
#[cfg_attr(test, assert_instr(palignr, n = 15))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_alignr_pi8(a: __m64, b: __m64, n: i32) -> __m64 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_alignr_pi8(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(n, call))
}

/// Horizontally add the adjacent pairs of values contained in 2 packed
/// 64-bit vectors of [4 x i16].
#[inline]
#[target_feature(enable = "ssse3,mmx")]
#[cfg_attr(test, assert_instr(phaddw))]
pub unsafe fn _mm_hadd_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_hadd_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontally add the adjacent pairs of values contained in 2 packed
/// 64-bit vectors of [2 x i32].
#[inline]
#[target_feature(enable = "ssse3,mmx")]
#[cfg_attr(test, assert_instr(phaddd))]
pub unsafe fn _mm_hadd_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_hadd_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontally add the adjacent pairs of values contained in 2 packed
/// 64-bit vectors of [4 x i16]. Positive sums greater than 7FFFh are
/// saturated to 7FFFh. Negative sums less than 8000h are saturated to 8000h.
#[inline]
#[target_feature(enable = "ssse3,mmx")]
#[cfg_attr(test, assert_instr(phaddsw))]
pub unsafe fn _mm_hadds_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_hadds_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontally subtracts the adjacent pairs of values contained in 2
/// packed 64-bit vectors of [4 x i16].
#[inline]
#[target_feature(enable = "ssse3,mmx")]
#[cfg_attr(test, assert_instr(phsubw))]
pub unsafe fn _mm_hsub_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_hsub_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontally subtracts the adjacent pairs of values contained in 2
/// packed 64-bit vectors of [2 x i32].
#[inline]
#[target_feature(enable = "ssse3,mmx")]
#[cfg_attr(test, assert_instr(phsubd))]
pub unsafe fn _mm_hsub_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_hsub_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Horizontally subtracts the adjacent pairs of values contained in 2
/// packed 64-bit vectors of [4 x i16]. Positive differences greater than
/// 7FFFh are saturated to 7FFFh. Negative differences less than 8000h are
/// saturated to 8000h.
#[inline]
#[target_feature(enable = "ssse3,mmx")]
#[cfg_attr(test, assert_instr(phsubsw))]
pub unsafe fn _mm_hsubs_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_hsubs_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiplies corresponding pairs of packed 8-bit unsigned integer
/// values contained in the first source operand and packed 8-bit signed
/// integer values contained in the second source operand, adds pairs of
/// contiguous products with signed saturation, and writes the 16-bit sums to
/// the corresponding bits in the destination.
#[inline]
#[target_feature(enable = "ssse3,mmx")]
#[cfg_attr(test, assert_instr(pmaddubsw))]
pub unsafe fn _mm_maddubs_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_maddubs_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Multiplies packed 16-bit signed integer values, truncates the 32-bit
/// products to the 18 most significant bits by right-shifting, rounds the
/// truncated value by adding 1, and writes bits [16:1] to the destination.
#[inline]
#[target_feature(enable = "ssse3,mmx")]
#[cfg_attr(test, assert_instr(pmulhrsw))]
pub unsafe fn _mm_mulhrs_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_mulhrs_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Negate packed 8-bit integers in `a` when the corresponding signed 8-bit
/// integer in `b` is negative, and return the results.
/// Element in result are zeroed out when the corresponding element in `b` is
/// zero.
#[inline]
#[target_feature(enable = "ssse3,mmx")]
#[cfg_attr(test, assert_instr(psignb))]
pub unsafe fn _mm_sign_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_sign_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Negate packed 16-bit integers in `a` when the corresponding signed 16-bit
/// integer in `b` is negative, and return the results.
/// Element in result are zeroed out when the corresponding element in `b` is
/// zero.
#[inline]
#[target_feature(enable = "ssse3,mmx")]
#[cfg_attr(test, assert_instr(psignw))]
pub unsafe fn _mm_sign_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_sign_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Negate packed 32-bit integers in `a` when the corresponding signed 32-bit
/// integer in `b` is negative, and return the results.
/// Element in result are zeroed out when the corresponding element in `b` is
/// zero.
#[inline]
#[target_feature(enable = "ssse3,mmx")]
#[cfg_attr(test, assert_instr(psignd))]
pub unsafe fn _mm_sign_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_sign_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

