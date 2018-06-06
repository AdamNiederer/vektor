#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Constructs a 64-bit integer vector initialized to zero.
#[inline]
#[target_feature(enable = "mmx")]
// FIXME: this produces a movl instead of xorps on x86
// FIXME: this produces a xor intrinsic instead of xorps on x86_64
#[cfg_attr(all(test, target_arch = "x86_64"), assert_instr(xor))]
pub unsafe fn _mm_setzero_si64() -> __m64 {
    ::mem::transmute(::myarch::_mm_setzero_si64())
}

/// Add packed 8-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(paddb))]
pub unsafe fn _mm_add_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_add_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 8-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(paddb))]
pub unsafe fn _m_paddb(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_paddb(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 16-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(paddw))]
pub unsafe fn _mm_add_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_add_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 16-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(paddw))]
pub unsafe fn _m_paddw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_paddw(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 32-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(paddd))]
pub unsafe fn _mm_add_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_add_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 32-bit integers in `a` and `b`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(paddd))]
pub unsafe fn _m_paddd(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_paddd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 8-bit integers in `a` and `b` using saturation.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(paddsb))]
pub unsafe fn _mm_adds_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_adds_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 8-bit integers in `a` and `b` using saturation.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(paddsb))]
pub unsafe fn _m_paddsb(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_paddsb(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 16-bit integers in `a` and `b` using saturation.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(paddsw))]
pub unsafe fn _mm_adds_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_adds_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed 16-bit integers in `a` and `b` using saturation.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(paddsw))]
pub unsafe fn _m_paddsw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_paddsw(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed unsigned 8-bit integers in `a` and `b` using saturation.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(paddusb))]
pub unsafe fn _mm_adds_pu8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_adds_pu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed unsigned 8-bit integers in `a` and `b` using saturation.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(paddusb))]
pub unsafe fn _m_paddusb(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_paddusb(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed unsigned 16-bit integers in `a` and `b` using saturation.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(paddusw))]
pub unsafe fn _mm_adds_pu16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_adds_pu16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Add packed unsigned 16-bit integers in `a` and `b` using saturation.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(paddusw))]
pub unsafe fn _m_paddusw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_paddusw(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 8-bit integers in `b` from packed 8-bit integers in `a`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(psubb))]
pub unsafe fn _mm_sub_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_sub_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 8-bit integers in `b` from packed 8-bit integers in `a`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(psubb))]
pub unsafe fn _m_psubb(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_psubb(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 16-bit integers in `b` from packed 16-bit integers in `a`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(psubw))]
pub unsafe fn _mm_sub_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_sub_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 16-bit integers in `b` from packed 16-bit integers in `a`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(psubw))]
pub unsafe fn _m_psubw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_psubw(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 32-bit integers in `b` from packed 32-bit integers in `a`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(psubd))]
pub unsafe fn _mm_sub_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_sub_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 32-bit integers in `b` from packed 32-bit integers in `a`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(psubd))]
pub unsafe fn _m_psubd(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_psubd(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 8-bit integers in `b` from packed 8-bit integers in `a`
/// using saturation.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(psubsb))]
pub unsafe fn _mm_subs_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_subs_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 8-bit integers in `b` from packed 8-bit integers in `a`
/// using saturation.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(psubsb))]
pub unsafe fn _m_psubsb(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_psubsb(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 16-bit integers in `b` from packed 16-bit integers in `a`
/// using saturation.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(psubsw))]
pub unsafe fn _mm_subs_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_subs_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed 16-bit integers in `b` from packed 16-bit integers in `a`
/// using saturation.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(psubsw))]
pub unsafe fn _m_psubsw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_psubsw(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed unsigned 8-bit integers in `b` from packed unsigned 8-bit
/// integers in `a` using saturation.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(psubusb))]
pub unsafe fn _mm_subs_pu8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_subs_pu8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed unsigned 8-bit integers in `b` from packed unsigned 8-bit
/// integers in `a` using saturation.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(psubusb))]
pub unsafe fn _m_psubusb(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_psubusb(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed unsigned 16-bit integers in `b` from packed unsigned
/// 16-bit integers in `a` using saturation.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(psubusw))]
pub unsafe fn _mm_subs_pu16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_subs_pu16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Subtract packed unsigned 16-bit integers in `b` from packed unsigned
/// 16-bit integers in `a` using saturation.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(psubusw))]
pub unsafe fn _m_psubusw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_m_psubusw(::mem::transmute(a), ::mem::transmute(b)))
}

/// Convert packed 16-bit integers from `a` and `b` to packed 8-bit integers
/// using signed saturation.
///
/// Positive values greater than 0x7F are saturated to 0x7F. Negative values
/// less than 0x80 are saturated to 0x80.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(packsswb))]
pub unsafe fn _mm_packs_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_packs_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Convert packed 32-bit integers from `a` and `b` to packed 16-bit integers
/// using signed saturation.
///
/// Positive values greater than 0x7F are saturated to 0x7F. Negative values
/// less than 0x80 are saturated to 0x80.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(packssdw))]
pub unsafe fn _mm_packs_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_packs_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compares whether each element of `a` is greater than the corresponding
/// element of `b` returning `0` for `false` and `-1` for `true`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(pcmpgtb))]
pub unsafe fn _mm_cmpgt_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_cmpgt_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compares whether each element of `a` is greater than the corresponding
/// element of `b` returning `0` for `false` and `-1` for `true`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(pcmpgtw))]
pub unsafe fn _mm_cmpgt_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_cmpgt_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Compares whether each element of `a` is greater than the corresponding
/// element of `b` returning `0` for `false` and `-1` for `true`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(pcmpgtd))]
pub unsafe fn _mm_cmpgt_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_cmpgt_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

 // FIXME punpcklbw expected
pub unsafe fn _mm_unpackhi_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_unpackhi_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpacks the upper four elements from two `i8x8` vectors and interleaves
/// them into the result: `[a.4, b.4, a.5, b.5, a.6, b.6, a.7, b.7]`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(punpckhbw))]
pub unsafe fn _mm_unpackhi_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_unpackhi_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpacks the lower four elements from two `i8x8` vectors and interleaves
/// them into the result: `[a.0, b.0, a.1, b.1, a.2, b.2, a.3, b.3]`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(punpcklbw))]
pub unsafe fn _mm_unpacklo_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_unpacklo_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpacks the lower two elements from two `i16x4` vectors and interleaves
/// them into the result: `[a.0 b.0 a.1 b.1]`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(punpcklwd))]
pub unsafe fn _mm_unpacklo_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_unpacklo_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpacks the upper element from two `i32x2` vectors and interleaves them
/// into the result: `[a.1, b.1]`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(punpckhdq))]
pub unsafe fn _mm_unpackhi_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_unpackhi_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Unpacks the lower element from two `i32x2` vectors and interleaves them
/// into the result: `[a.0, b.0]`.
#[inline]
#[target_feature(enable = "mmx")]
#[cfg_attr(test, assert_instr(punpckldq))]
pub unsafe fn _mm_unpacklo_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::myarch::_mm_unpacklo_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

/// Set packed 16-bit integers in dst with the supplied values.
#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_set_pi16(e3: i16, e2: i16, e1: i16, e0: i16) -> __m64 {
    ::mem::transmute(::myarch::_mm_set_pi16(::mem::transmute(e3), ::mem::transmute(e2), ::mem::transmute(e1), ::mem::transmute(e0)))
}

/// Set packed 32-bit integers in dst with the supplied values.
#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_set_pi32(e1: i32, e0: i32) -> __m64 {
    ::mem::transmute(::myarch::_mm_set_pi32(::mem::transmute(e1), ::mem::transmute(e0)))
}

/// Set packed 8-bit integers in dst with the supplied values.
#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_set_pi8(e7: i8, e6: i8, e5: i8, e4: i8, e3: i8, e2: i8, e1: i8, e0: i8) -> __m64 {
    ::mem::transmute(::myarch::_mm_set_pi8(::mem::transmute(e7), ::mem::transmute(e6), ::mem::transmute(e5), ::mem::transmute(e4), ::mem::transmute(e3), ::mem::transmute(e2), ::mem::transmute(e1), ::mem::transmute(e0)))
}

/// Broadcast 16-bit integer a to all all elements of dst.
#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_set1_pi16(a: i16) -> __m64 {
    ::mem::transmute(::myarch::_mm_set1_pi16(::mem::transmute(a)))
}

/// Broadcast 32-bit integer a to all all elements of dst.
#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_set1_pi32(a: i32) -> __m64 {
    ::mem::transmute(::myarch::_mm_set1_pi32(::mem::transmute(a)))
}

/// Broadcast 8-bit integer a to all all elements of dst.
#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_set1_pi8(a: i8) -> __m64 {
    ::mem::transmute(::myarch::_mm_set1_pi8(::mem::transmute(a)))
}

/// Set packed 16-bit integers in dst with the supplied values in reverse
/// order.
#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_setr_pi16(e0: i16, e1: i16, e2: i16, e3: i16) -> __m64 {
    ::mem::transmute(::myarch::_mm_setr_pi16(::mem::transmute(e0), ::mem::transmute(e1), ::mem::transmute(e2), ::mem::transmute(e3)))
}

/// Set packed 32-bit integers in dst with the supplied values in reverse
/// order.
#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_setr_pi32(e0: i32, e1: i32) -> __m64 {
    ::mem::transmute(::myarch::_mm_setr_pi32(::mem::transmute(e0), ::mem::transmute(e1)))
}

/// Set packed 8-bit integers in dst with the supplied values in reverse order.
#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_setr_pi8(e0: i8, e1: i8, e2: i8, e3: i8, e4: i8, e5: i8, e6: i8, e7: i8) -> __m64 {
    ::mem::transmute(::myarch::_mm_setr_pi8(::mem::transmute(e0), ::mem::transmute(e1), ::mem::transmute(e2), ::mem::transmute(e3), ::mem::transmute(e4), ::mem::transmute(e5), ::mem::transmute(e6), ::mem::transmute(e7)))
}

