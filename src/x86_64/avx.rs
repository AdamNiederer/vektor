// Autogenerated by `scrape.py`.
// See https://github.com/AdamNiederer/vektor-gen

#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Copy `a` to result, and insert the 64-bit integer `i` into result
/// at the location specified by `index`.
#[inline]
#[rustc_args_required_const(2)]
#[target_feature(enable = "avx")]
// This intrinsic has no corresponding instruction.
pub unsafe fn _mm256_insert_epi64(a: i64x4, i: i64, index: i32) -> i64x4 {

    macro_rules! call {
        ($imm8:expr) => {
            crate::myarch::_mm256_insert_epi64(crate::mem::transmute(a), crate::mem::transmute(i), $imm8)
        };
    }

    crate::mem::transmute(constify_imm8!(index, call))
}

