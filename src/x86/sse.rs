use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_add_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_add_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_add_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_add_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_sub_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_sub_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_sub_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_sub_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_mul_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_mul_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_mul_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_mul_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_div_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_div_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_div_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_div_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_sqrt_ss(a: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_sqrt_ss(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_sqrt_ps(a: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_sqrt_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_rcp_ss(a: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_rcp_ss(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_rcp_ps(a: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_rcp_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_rsqrt_ss(a: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_rsqrt_ss(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_rsqrt_ps(a: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_rsqrt_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_min_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_min_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_min_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_min_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_max_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_max_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_max_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_max_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_and_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_and_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_andnot_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_andnot_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_or_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_or_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_xor_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_xor_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpeq_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpeq_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmplt_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmplt_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmple_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmple_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpgt_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpgt_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpge_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpge_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpneq_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpneq_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpnlt_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpnlt_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpnle_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpnle_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpngt_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpngt_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpnge_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpnge_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpord_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpord_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpunord_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpunord_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpeq_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpeq_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmplt_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmplt_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmple_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmple_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpgt_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpgt_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpge_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpge_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpneq_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpneq_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpnlt_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpnlt_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpnle_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpnle_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpngt_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpngt_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpnge_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpnge_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpord_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpord_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cmpunord_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpunord_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_comieq_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_comieq_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_comilt_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_comilt_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_comile_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_comile_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_comigt_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_comigt_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_comige_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_comige_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_comineq_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_comineq_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_ucomieq_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_ucomieq_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_ucomilt_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_ucomilt_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_ucomile_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_ucomile_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_ucomigt_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_ucomigt_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_ucomige_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_ucomige_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_ucomineq_ss(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_ucomineq_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cvtss_si32(a: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cvtss_si32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cvt_ss2si(a: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cvt_ss2si(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cvttss_si32(a: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cvttss_si32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cvtt_ss2si(a: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cvtt_ss2si(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cvtss_f32(a: f32x4) -> f32 {
    ::mem::transmute(::arch::x86::_mm_cvtss_f32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cvtsi32_ss(a: f32x4, b: i32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cvtsi32_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cvt_si2ss(a: f32x4, b: i32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cvt_si2ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_set_ss(a: f32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_set_ss(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_set1_ps(a: f32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_set1_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_set_ps1(a: f32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_set_ps1(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_set_ps(a: f32, b: f32, c: f32, d: f32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_set_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c), ::mem::transmute(d)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_setr_ps(a: f32, b: f32, c: f32, d: f32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_setr_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c), ::mem::transmute(d)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_setzero_ps() -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_setzero_ps())
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_shuffle_ps(a: f32x4, b: f32x4, mask: u32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_shuffle_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_unpackhi_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_unpackhi_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_unpacklo_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_unpacklo_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_movehl_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_movehl_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_movelh_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_movelh_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_movemask_ps(a: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_movemask_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_loadh_pi(a: f32x4, p: *const __m64) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_loadh_pi(::mem::transmute(a), ::mem::transmute(p)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_loadl_pi(a: f32x4, p: *const __m64) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_loadl_pi(::mem::transmute(a), ::mem::transmute(p)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_load_ss(p: *const f32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_load_ss(::mem::transmute(p)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_load1_ps(p: *const f32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_load1_ps(::mem::transmute(p)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_load_ps1(p: *const f32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_load_ps1(::mem::transmute(p)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_load_ps(p: *const f32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_load_ps(::mem::transmute(p)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_loadu_ps(p: *const f32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_loadu_ps(::mem::transmute(p)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_loadr_ps(p: *const f32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_loadr_ps(::mem::transmute(p)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_move_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_move_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_getcsr() -> u32 {
    ::mem::transmute(::arch::x86::_mm_getcsr())
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _MM_GET_EXCEPTION_MASK() -> u32 {
    ::mem::transmute(::arch::x86::_MM_GET_EXCEPTION_MASK())
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _MM_GET_EXCEPTION_STATE() -> u32 {
    ::mem::transmute(::arch::x86::_MM_GET_EXCEPTION_STATE())
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _MM_GET_FLUSH_ZERO_MODE() -> u32 {
    ::mem::transmute(::arch::x86::_MM_GET_FLUSH_ZERO_MODE())
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _MM_GET_ROUNDING_MODE() -> u32 {
    ::mem::transmute(::arch::x86::_MM_GET_ROUNDING_MODE())
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_undefined_ps() -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_undefined_ps())
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_max_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_max_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _m_pmaxsw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_pmaxsw(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_max_pu8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_max_pu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _m_pmaxub(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_pmaxub(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_min_pi16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_min_pi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _m_pminsw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_pminsw(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_min_pu8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_min_pu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _m_pminub(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_pminub(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_mulhi_pu16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_mulhi_pu16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _m_pmulhuw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_pmulhuw(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_avg_pu8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_avg_pu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _m_pavgb(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_pavgb(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_avg_pu16(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_avg_pu16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _m_pavgw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_pavgw(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_sad_pu8(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_sad_pu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _m_psadbw(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_m_psadbw(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_cvtpi32_ps(a: f32x4, b: __m64) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cvtpi32_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_cvt_pi2ps(a: f32x4, b: __m64) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cvt_pi2ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_cvtpi8_ps(a: __m64) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cvtpi8_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_cvtpu8_ps(a: __m64) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cvtpu8_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_cvtpi16_ps(a: __m64) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cvtpi16_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_cvtpu16_ps(a: __m64) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cvtpu16_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_cvtpi32x2_ps(a: __m64, b: __m64) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cvtpi32x2_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_extract_pi16(a: __m64, imm2: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_extract_pi16(::mem::transmute(a), ::mem::transmute(imm2)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _m_pextrw(a: __m64, imm2: i32) -> i32 {
    ::mem::transmute(::arch::x86::_m_pextrw(::mem::transmute(a), ::mem::transmute(imm2)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_insert_pi16(a: __m64, d: i32, imm2: i32) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_insert_pi16(::mem::transmute(a), ::mem::transmute(d), ::mem::transmute(imm2)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _m_pinsrw(a: __m64, d: i32, imm2: i32) -> __m64 {
    ::mem::transmute(::arch::x86::_m_pinsrw(::mem::transmute(a), ::mem::transmute(d), ::mem::transmute(imm2)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_movemask_pi8(a: __m64) -> i32 {
    ::mem::transmute(::arch::x86::_mm_movemask_pi8(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _m_pmovmskb(a: __m64) -> i32 {
    ::mem::transmute(::arch::x86::_m_pmovmskb(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_shuffle_pi16(a: __m64, imm8: i32) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_shuffle_pi16(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _m_pshufw(a: __m64, imm8: i32) -> __m64 {
    ::mem::transmute(::arch::x86::_m_pshufw(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_cvttps_pi32(a: f32x4) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_cvttps_pi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_cvtt_ps2pi(a: f32x4) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_cvtt_ps2pi(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_cvtps_pi32(a: f32x4) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_cvtps_pi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_cvt_ps2pi(a: f32x4) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_cvt_ps2pi(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_cvtps_pi16(a: f32x4) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_cvtps_pi16(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse,mmx")]
pub unsafe fn _mm_cvtps_pi8(a: f32x4) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_cvtps_pi8(::mem::transmute(a)))
}

