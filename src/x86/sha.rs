#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Perform an intermediate calculation for the next four SHA1 message values
/// (unsigned 32-bit integers) using previous message values from `a` and `b`,
/// and returning the result.
#[inline]
#[target_feature(enable = "sha")]
#[cfg_attr(test, assert_instr(sha1msg1))]
pub unsafe fn _mm_sha1msg1_epu32(a: u32x4, b: u32x4) -> u32x4 {
    crate::mem::transmute(crate::myarch::_mm_sha1msg1_epu32(crate::mem::transmute(a), crate::mem::transmute(b)))
}

/// Perform the final calculation for the next four SHA1 message values
/// (unsigned 32-bit integers) using the intermediate result in `a` and the
/// previous message values in `b`, and returns the result.
#[inline]
#[target_feature(enable = "sha")]
#[cfg_attr(test, assert_instr(sha1msg2))]
pub unsafe fn _mm_sha1msg2_epu32(a: u32x4, b: u32x4) -> u32x4 {
    crate::mem::transmute(crate::myarch::_mm_sha1msg2_epu32(crate::mem::transmute(a), crate::mem::transmute(b)))
}

/// Calculate SHA1 state variable E after four rounds of operation from the
/// current SHA1 state variable `a`, add that value to the scheduled values
/// (unsigned 32-bit integers) in `b`, and returns the result.
#[inline]
#[target_feature(enable = "sha")]
#[cfg_attr(test, assert_instr(sha1nexte))]
pub unsafe fn _mm_sha1nexte_epu32(a: u32x4, b: u32x4) -> u32x4 {
    crate::mem::transmute(crate::myarch::_mm_sha1nexte_epu32(crate::mem::transmute(a), crate::mem::transmute(b)))
}

/// Perform four rounds of SHA1 operation using an initial SHA1 state (A,B,C,D)
/// from `a` and some pre-computed sum of the next 4 round message values
/// (unsigned 32-bit integers), and state variable E from `b`, and return the
/// updated SHA1 state (A,B,C,D). `func` contains the logic functions and round
/// constants.
#[inline]
#[target_feature(enable = "sha")]
#[cfg_attr(test, assert_instr(sha1rnds4, func = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_sha1rnds4_epu32(a: u32x4, b: u32x4, func: i32) -> u32x4 {

    macro_rules! call {
        ($imm8:expr) => {
            crate::myarch::_mm_sha1rnds4_epu32(crate::mem::transmute(a), crate::mem::transmute(b), $imm8)
        };
    }

    crate::mem::transmute(constify_imm8!(func, call))
}

/// Perform an intermediate calculation for the next four SHA256 message values
/// (unsigned 32-bit integers) using previous message values from `a` and `b`,
/// and return the result.
#[inline]
#[target_feature(enable = "sha")]
#[cfg_attr(test, assert_instr(sha256msg1))]
pub unsafe fn _mm_sha256msg1_epu32(a: u32x4, b: u32x4) -> u32x4 {
    crate::mem::transmute(crate::myarch::_mm_sha256msg1_epu32(crate::mem::transmute(a), crate::mem::transmute(b)))
}

/// Perform the final calculation for the next four SHA256 message values
/// (unsigned 32-bit integers) using previous message values from `a` and `b`,
/// and return the result.
#[inline]
#[target_feature(enable = "sha")]
#[cfg_attr(test, assert_instr(sha256msg2))]
pub unsafe fn _mm_sha256msg2_epu32(a: u32x4, b: u32x4) -> u32x4 {
    crate::mem::transmute(crate::myarch::_mm_sha256msg2_epu32(crate::mem::transmute(a), crate::mem::transmute(b)))
}

/// Perform 2 rounds of SHA256 operation using an initial SHA256 state
/// (C,D,G,H) from `a`, an initial SHA256 state (A,B,E,F) from `b`, and a
/// pre-computed sum of the next 2 round message values (unsigned 32-bit
/// integers) and the corresponding round constants from `k`, and store the
/// updated SHA256 state (A,B,E,F) in dst.
#[inline]
#[target_feature(enable = "sha")]
#[cfg_attr(test, assert_instr(sha256rnds2))]
pub unsafe fn _mm_sha256rnds2_epu32(a: u32x4, b: u32x4, k: u32x4) -> u32x4 {
    crate::mem::transmute(crate::myarch::_mm_sha256rnds2_epu32(crate::mem::transmute(a), crate::mem::transmute(b), crate::mem::transmute(k)))
}

