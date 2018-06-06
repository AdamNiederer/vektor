#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Extract a 64-bit integer from `a`, selected with `imm8`.
#[inline]
#[target_feature(enable = "avx2")]
#[rustc_args_required_const(1)]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_extract_epi64(a: i64x4, imm8: i32) -> i64 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm256_extract_epi64(::mem::transmute(a), $imm8)
        };
    }

   ::mem::transmute(constify_imm8!(imm8, call))
}

