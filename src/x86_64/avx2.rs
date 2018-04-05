use ::arch::x86_64::*;
use ::simd::*;

// #[inline]
// #[target_feature(enable = "avx2")]
// pub unsafe fn _mm256_extract_epi64(a: i64x4, imm8: i32) -> i64 {
//     ::mem::transmute(::arch::x86_64::_mm256_extract_epi64(::mem::transmute(a), ::mem::transmute(imm8)))
// }
