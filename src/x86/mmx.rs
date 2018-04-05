use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_setzero_si64() -> __m64 {
    ::mem::transmute(::arch::x86::_mm_setzero_si64())
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_add_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_add_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _m_paddb(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_paddb(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_add_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_add_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _m_paddw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_paddw(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_add_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_add_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _m_paddd(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_paddd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_adds_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_adds_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _m_paddsb(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_paddsb(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_adds_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_adds_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _m_paddsw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_paddsw(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_adds_pu8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_adds_pu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _m_paddusb(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_paddusb(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_adds_pu16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_adds_pu16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _m_paddusw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_paddusw(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_sub_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_sub_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _m_psubb(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_psubb(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_sub_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_sub_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _m_psubw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_psubw(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_sub_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_sub_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _m_psubd(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_psubd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_subs_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_subs_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _m_psubsb(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_psubsb(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_subs_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_subs_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _m_psubsw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_psubsw(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_subs_pu8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_subs_pu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _m_psubusb(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_psubusb(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_subs_pu16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_subs_pu16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _m_psubusw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_psubusw(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_packs_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_packs_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_packs_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_packs_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_cmpgt_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_cmpgt_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_cmpgt_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_cmpgt_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_cmpgt_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_cmpgt_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_unpackhi_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_unpackhi_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_unpackhi_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_unpackhi_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_unpacklo_pi8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_unpacklo_pi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_unpacklo_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_unpacklo_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_unpackhi_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_unpackhi_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_unpacklo_pi32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_unpacklo_pi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_set_pi16(e3: i16, e2: i16, e1: i16, e0: i16) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_set_pi16(::mem::transmute(e3), ::mem::transmute(e2), ::mem::transmute(e1), ::mem::transmute(e0)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_set_pi32(e1: i32, e0: i32) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_set_pi32(::mem::transmute(e1), ::mem::transmute(e0)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_set_pi8(e7: i8, e6: i8, e5: i8, e4: i8, e3: i8, e2: i8, e1: i8, e0: i8) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_set_pi8(::mem::transmute(e7), ::mem::transmute(e6), ::mem::transmute(e5), ::mem::transmute(e4), ::mem::transmute(e3), ::mem::transmute(e2), ::mem::transmute(e1), ::mem::transmute(e0)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_set1_pi16(a: i16) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_set1_pi16(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_set1_pi32(a: i32) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_set1_pi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_set1_pi8(a: i8) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_set1_pi8(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_setr_pi16(e0: i16, e1: i16, e2: i16, e3: i16) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_setr_pi16(::mem::transmute(e0), ::mem::transmute(e1), ::mem::transmute(e2), ::mem::transmute(e3)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_setr_pi32(e0: i32, e1: i32) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_setr_pi32(::mem::transmute(e0), ::mem::transmute(e1)))
}

#[inline]
#[target_feature(enable = "mmx")]
pub unsafe fn _mm_setr_pi8(e0: i8, e1: i8, e2: i8, e3: i8, e4: i8, e5: i8, e6: i8, e7: i8) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_setr_pi8(::mem::transmute(e0), ::mem::transmute(e1), ::mem::transmute(e2), ::mem::transmute(e3), ::mem::transmute(e4), ::mem::transmute(e5), ::mem::transmute(e6), ::mem::transmute(e7)))
}

