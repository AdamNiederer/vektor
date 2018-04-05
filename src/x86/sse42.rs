use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_cmpistrm(a: __m128i, b: __m128i, imm8: i32) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_cmpistrm(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_cmpistri(a: __m128i, b: __m128i, imm8: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cmpistri(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_cmpistrz(a: __m128i, b: __m128i, imm8: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cmpistrz(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_cmpistrc(a: __m128i, b: __m128i, imm8: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cmpistrc(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_cmpistrs(a: __m128i, b: __m128i, imm8: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cmpistrs(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_cmpistro(a: __m128i, b: __m128i, imm8: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cmpistro(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_cmpistra(a: __m128i, b: __m128i, imm8: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cmpistra(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_cmpestrm(a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i32) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_cmpestrm(::mem::transmute(a), ::mem::transmute(la), ::mem::transmute(b), ::mem::transmute(lb), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_cmpestri(a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cmpestri(::mem::transmute(a), ::mem::transmute(la), ::mem::transmute(b), ::mem::transmute(lb), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_cmpestrz(a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cmpestrz(::mem::transmute(a), ::mem::transmute(la), ::mem::transmute(b), ::mem::transmute(lb), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_cmpestrc(a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cmpestrc(::mem::transmute(a), ::mem::transmute(la), ::mem::transmute(b), ::mem::transmute(lb), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_cmpestrs(a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cmpestrs(::mem::transmute(a), ::mem::transmute(la), ::mem::transmute(b), ::mem::transmute(lb), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_cmpestro(a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cmpestro(::mem::transmute(a), ::mem::transmute(la), ::mem::transmute(b), ::mem::transmute(lb), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_cmpestra(a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cmpestra(::mem::transmute(a), ::mem::transmute(la), ::mem::transmute(b), ::mem::transmute(lb), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_crc32_u8(crc: u32, v: u8) -> u32 {
    ::mem::transmute(::arch::x86::_mm_crc32_u8(::mem::transmute(crc), ::mem::transmute(v)))
}

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_crc32_u16(crc: u32, v: u16) -> u32 {
    ::mem::transmute(::arch::x86::_mm_crc32_u16(::mem::transmute(crc), ::mem::transmute(v)))
}

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_crc32_u32(crc: u32, v: u32) -> u32 {
    ::mem::transmute(::arch::x86::_mm_crc32_u32(::mem::transmute(crc), ::mem::transmute(v)))
}

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_cmpgt_epi64(a: i64x2, b: i64x2) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpgt_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

