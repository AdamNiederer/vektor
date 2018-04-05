use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _blcfill_u32(x: u32) -> u32 {
    ::mem::transmute(::arch::x86::_blcfill_u32(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _blcfill_u64(x: u64) -> u64 {
    ::mem::transmute(::arch::x86::_blcfill_u64(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _blci_u32(x: u32) -> u32 {
    ::mem::transmute(::arch::x86::_blci_u32(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _blci_u64(x: u64) -> u64 {
    ::mem::transmute(::arch::x86::_blci_u64(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _blcic_u32(x: u32) -> u32 {
    ::mem::transmute(::arch::x86::_blcic_u32(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _blcic_u64(x: u64) -> u64 {
    ::mem::transmute(::arch::x86::_blcic_u64(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _blcmsk_u32(x: u32) -> u32 {
    ::mem::transmute(::arch::x86::_blcmsk_u32(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _blcmsk_u64(x: u64) -> u64 {
    ::mem::transmute(::arch::x86::_blcmsk_u64(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _blcs_u32(x: u32) -> u32 {
    ::mem::transmute(::arch::x86::_blcs_u32(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _blcs_u64(x: u64) -> u64 {
    ::mem::transmute(::arch::x86::_blcs_u64(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _blsfill_u32(x: u32) -> u32 {
    ::mem::transmute(::arch::x86::_blsfill_u32(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _blsfill_u64(x: u64) -> u64 {
    ::mem::transmute(::arch::x86::_blsfill_u64(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _blsic_u32(x: u32) -> u32 {
    ::mem::transmute(::arch::x86::_blsic_u32(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _blsic_u64(x: u64) -> u64 {
    ::mem::transmute(::arch::x86::_blsic_u64(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _t1mskc_u32(x: u32) -> u32 {
    ::mem::transmute(::arch::x86::_t1mskc_u32(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _t1mskc_u64(x: u64) -> u64 {
    ::mem::transmute(::arch::x86::_t1mskc_u64(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _tzmsk_u32(x: u32) -> u32 {
    ::mem::transmute(::arch::x86::_tzmsk_u32(::mem::transmute(x)))
}

#[inline]
#[target_feature(enable = "tbm")]
pub unsafe fn _tzmsk_u64(x: u64) -> u64 {
    ::mem::transmute(::arch::x86::_tzmsk_u64(::mem::transmute(x)))
}

