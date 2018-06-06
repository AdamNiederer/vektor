#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Perform one round of an AES decryption flow on data (state) in `a`.
#[inline]
#[target_feature(enable = "aes")]
#[cfg_attr(test, assert_instr(aesdec))]
pub unsafe fn _mm_aesdec_si128(a: __m128i, round_key: __m128i) -> __m128i {
    ::mem::transmute(::myarch::_mm_aesdec_si128(::mem::transmute(a), ::mem::transmute(round_key)))
}

/// Perform the last round of an AES decryption flow on data (state) in `a`.
#[inline]
#[target_feature(enable = "aes")]
#[cfg_attr(test, assert_instr(aesdeclast))]
pub unsafe fn _mm_aesdeclast_si128(a: __m128i, round_key: __m128i) -> __m128i {
    ::mem::transmute(::myarch::_mm_aesdeclast_si128(::mem::transmute(a), ::mem::transmute(round_key)))
}

/// Perform one round of an AES encryption flow on data (state) in `a`.
#[inline]
#[target_feature(enable = "aes")]
#[cfg_attr(test, assert_instr(aesenc))]
pub unsafe fn _mm_aesenc_si128(a: __m128i, round_key: __m128i) -> __m128i {
    ::mem::transmute(::myarch::_mm_aesenc_si128(::mem::transmute(a), ::mem::transmute(round_key)))
}

/// Perform the last round of an AES encryption flow on data (state) in `a`.
#[inline]
#[target_feature(enable = "aes")]
#[cfg_attr(test, assert_instr(aesenclast))]
pub unsafe fn _mm_aesenclast_si128(a: __m128i, round_key: __m128i) -> __m128i {
    ::mem::transmute(::myarch::_mm_aesenclast_si128(::mem::transmute(a), ::mem::transmute(round_key)))
}

/// Perform the `InvMixColumns` transformation on `a`.
#[inline]
#[target_feature(enable = "aes")]
#[cfg_attr(test, assert_instr(aesimc))]
pub unsafe fn _mm_aesimc_si128(a: __m128i) -> __m128i {
    ::mem::transmute(::myarch::_mm_aesimc_si128(::mem::transmute(a)))
}

/// Assist in expanding the AES cipher key.
///
/// Assist in expanding the AES cipher key by computing steps towards
/// generating a round key for encryption cipher using data from `a` and an
/// 8-bit round constant `imm8`.
#[inline]
#[target_feature(enable = "aes")]
#[cfg_attr(test, assert_instr(aeskeygenassist, imm8 = 0))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_aeskeygenassist_si128(a: __m128i, imm8: i32) -> __m128i {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_aeskeygenassist_si128(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

