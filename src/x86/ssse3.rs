use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn _mm_abs_epi8(a: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_abs_epi8(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn _mm_abs_epi16(a: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_abs_epi16(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn _mm_abs_epi32(a: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_abs_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn _mm_shuffle_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_shuffle_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn _mm_alignr_epi8(a: u8x16, b: u8x16, n: i32) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_alignr_epi8(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(n)))
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn _mm_hadd_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_hadd_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn _mm_hadds_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_hadds_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn _mm_hadd_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_hadd_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn _mm_hsub_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_hsub_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn _mm_hsubs_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_hsubs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn _mm_hsub_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_hsub_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn _mm_maddubs_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_maddubs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn _mm_mulhrs_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_mulhrs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn _mm_sign_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_sign_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn _mm_sign_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_sign_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn _mm_sign_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_sign_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3,mmx")]
pub unsafe fn _mm_abs_pi8(a: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_abs_pi8(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "ssse3,mmx")]
pub unsafe fn _mm_abs_pi16(a: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_abs_pi16(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "ssse3,mmx")]
pub unsafe fn _mm_abs_pi32(a: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_abs_pi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "ssse3,mmx")]
pub unsafe fn _mm_shuffle_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_shuffle_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3,mmx")]
pub unsafe fn _mm_alignr_pi8(a: __m64, b: __m64, n: i32) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_alignr_pi8(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(n)))
}

#[inline]
#[target_feature(enable = "ssse3,mmx")]
pub unsafe fn _mm_hadd_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_hadd_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3,mmx")]
pub unsafe fn _mm_hadd_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_hadd_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3,mmx")]
pub unsafe fn _mm_hadds_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_hadds_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3,mmx")]
pub unsafe fn _mm_hsub_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_hsub_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3,mmx")]
pub unsafe fn _mm_hsub_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_hsub_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3,mmx")]
pub unsafe fn _mm_hsubs_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_hsubs_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3,mmx")]
pub unsafe fn _mm_maddubs_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_maddubs_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3,mmx")]
pub unsafe fn _mm_mulhrs_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_mulhrs_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3,mmx")]
pub unsafe fn _mm_sign_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_sign_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3,mmx")]
pub unsafe fn _mm_sign_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_sign_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "ssse3,mmx")]
pub unsafe fn _mm_sign_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_sign_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

