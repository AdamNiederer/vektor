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
            ::myarch::_mm256_insert_epi64(::mem::transmute(a), ::mem::transmute(i), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(index, call))
}

