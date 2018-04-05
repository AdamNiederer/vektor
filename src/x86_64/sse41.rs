use ::arch::x86_64::*;
use ::simd::*;

// #[inline]
// #[target_feature(enable = "sse4.1")]
// pub unsafe fn _mm_extract_epi64(a: i64x2, imm8: i32) -> i64 {
//     ::mem::transmute(::arch::x86_64::_mm_extract_epi64(::mem::transmute(a), ::mem::transmute(imm8)))
// }

// #[inline]
// #[target_feature(enable = "sse4.1")]
// pub unsafe fn _mm_insert_epi64(a: i64x2, i: i64, imm8: i32) -> i64x2 {
//     ::mem::transmute(::arch::x86_64::_mm_insert_epi64(::mem::transmute(a), ::mem::transmute(i), ::mem::transmute(imm8)))
// }
