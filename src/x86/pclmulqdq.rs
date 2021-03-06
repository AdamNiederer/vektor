// Autogenerated by `scrape.py`.
// See https://github.com/AdamNiederer/vektor-gen

#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Perform a carry-less multiplication of two 64-bit polynomials over the
/// finite field GF(2^k).
///
/// The immediate byte is used for determining which halves of `a` and `b`
/// should be used. Immediate bits other than 0 and 4 are ignored.
#[inline]
#[target_feature(enable = "pclmulqdq")]
#[cfg_attr(all(test, not(target_os = "linux")),
           assert_instr(pclmulqdq, imm8 = 0))]
#[cfg_attr(all(test, target_os = "linux"),
           assert_instr(pclmullqlqdq, imm8 = 0))]
#[cfg_attr(all(test, target_os = "linux"),
           assert_instr(pclmulhqlqdq, imm8 = 1))]
#[cfg_attr(all(test, target_os = "linux"),
           assert_instr(pclmullqhqdq, imm8 = 16))]
#[cfg_attr(all(test, target_os = "linux"),
           assert_instr(pclmulhqhqdq, imm8 = 17))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_clmulepi64_si128(a: i64x2, b: i64x2, imm8: i32) -> i64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            crate::myarch::_mm_clmulepi64_si128(crate::mem::transmute(a), crate::mem::transmute(b), $imm8)
        };
    }

    crate::mem::transmute(constify_imm8!(imm8, call))
}

