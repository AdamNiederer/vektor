#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

// FIXME(blocked on #248): _mm_extracti_si64(x, len, idx) // EXTRQ
// FIXME(blocked on #248): _mm_inserti_si64(x, y, len, idx) // INSERTQ

/// Extracts the bit range specified by `y` from the lower 64 bits of `x`.
///
/// The [13:8] bits of `y` specify the index of the bit-range to extract. The
/// [5:0] bits of `y` specify the length of the bit-range to extract. All other
/// bits are ignored.
///
/// If the length is zero, it is interpreted as `64`. If the length and index
/// are zero, the lower 64 bits of `x` are extracted.
///
/// If `length == 0 && index > 0` or `lenght + index > 64` the result is
/// undefined.
#[inline]
#[target_feature(enable = "sse4a")]
#[cfg_attr(test, assert_instr(extrq))]
pub unsafe fn _mm_extract_si64(x: __m128i, y: __m128i) -> __m128i {
    crate::mem::transmute(crate::myarch::_mm_extract_si64(crate::mem::transmute(x), crate::mem::transmute(y)))
}

/// Inserts the `[length:0]` bits of `y` into `x` at `index`.
///
/// The bits of `y`:
///
/// - `[69:64]` specify the `length`,
/// - `[77:72]` specify the index.
///
/// If the `length` is zero it is interpreted as `64`. If `index + length > 64`
/// or `index > 0 && length == 0` the result is undefined.
#[inline]
#[target_feature(enable = "sse4a")]
#[cfg_attr(test, assert_instr(insertq))]
pub unsafe fn _mm_insert_si64(x: __m128i, y: __m128i) -> __m128i {
    crate::mem::transmute(crate::myarch::_mm_insert_si64(crate::mem::transmute(x), crate::mem::transmute(y)))
}

