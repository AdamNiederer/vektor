#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Compare packed strings with implicit lengths in `a` and `b` using the
/// control in `imm8`, and return the generated mask.
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(pcmpistrm, imm8 = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_cmpistrm(a: __m128i, b: __m128i, imm8: i32) -> __m128i {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmpistrm(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Compare packed strings with implicit lengths in `a` and `b` using the
/// control in `imm8` and return the generated index. Similar to
/// [`_mm_cmpestri`] with the exception that [`_mm_cmpestri`] requires the
/// lengths of `a` and `b` to be explicitly specified.
///
/// # Control modes
///
/// The control specified by `imm8` may be one or more of the following.
///
/// ## Data size and signedness
///
///  - [`_SIDD_UBYTE_OPS`] - Default
///  - [`_SIDD_UWORD_OPS`]
///  - [`_SIDD_SBYTE_OPS`]
///  - [`_SIDD_SWORD_OPS`]
///
/// ## Comparison options
///  - [`_SIDD_CMP_EQUAL_ANY`] - Default
///  - [`_SIDD_CMP_RANGES`]
///  - [`_SIDD_CMP_EQUAL_EACH`]
///  - [`_SIDD_CMP_EQUAL_ORDERED`]
///
/// ## Result polarity
///  - [`_SIDD_POSITIVE_POLARITY`] - Default
///  - [`_SIDD_NEGATIVE_POLARITY`]
///
/// ## Bit returned
///  - [`_SIDD_LEAST_SIGNIFICANT`] - Default
///  - [`_SIDD_MOST_SIGNIFICANT`]
///
/// # Examples
///
/// Find a substring using [`_SIDD_CMP_EQUAL_ORDERED`]
///
/// ```
/// # #![feature(cfg_target_feature)]
/// # #![feature(target_feature)]
/// # #![feature(stdsimd)]
/// # #![cfg_attr(not(dox), no_std)]
/// # #[cfg(not(dox))]
/// # #[macro_use]
/// # extern crate std as real_std;
/// # #[cfg(not(dox))]
/// # #[macro_use]
/// # extern crate stdsimd as std;
/// # #[cfg(not(dox))]
/// # use real_std::prelude::v1::*;
/// #[cfg(target_arch = "x86")]
/// use std::arch::x86::*;
/// #[cfg(target_arch = "x86_64")]
/// use std::arch::x86_64::*;
///
/// # fn main() {
/// #     if is_x86_feature_detected!("sse4.2") {
/// #         #[target_feature(enable = "sse4.2")]
/// #         unsafe fn worker() {
/// let haystack = b"This is a long string of text data\r\n\tthat extends
/// multiple lines";
/// let needle = b"\r\n\t\0\0\0\0\0\0\0\0\0\0\0\0\0";
///
/// let a = _mm_loadu_si128(needle.as_ptr() as *const _);
/// let hop = 16;
/// let mut indexes = Vec::new();
///
/// // Chunk the haystack into 16 byte chunks and find
/// // the first "\r\n\t" in the chunk.
/// for (i, chunk) in haystack.chunks(hop).enumerate() {
///     let b = _mm_loadu_si128(chunk.as_ptr() as *const _);
///     let idx = _mm_cmpistri(a, b, _SIDD_CMP_EQUAL_ORDERED);
///     if idx != 16 {
///        indexes.push((idx as usize) + (i * hop));
///     }
/// }
/// assert_eq!(indexes, vec![34]);
/// #         }
/// #         unsafe { worker(); }
/// #     }
/// # }
/// ```
///
/// The `_mm_cmpistri` intrinsic may also be used to find the existance of
/// one or more of a given set of characters in the haystack.
///
/// ```
/// # #![feature(cfg_target_feature)]
/// # #![feature(target_feature)]
/// # #![feature(stdsimd)]
/// # #![cfg_attr(not(dox), no_std)]
/// # #[cfg(not(dox))]
/// # #[macro_use]
/// # extern crate std as real_std;
/// # #[cfg(not(dox))]
/// # #[macro_use]
/// # extern crate stdsimd as std;
/// #[cfg(target_arch = "x86")]
/// use std::arch::x86::*;
/// #[cfg(target_arch = "x86_64")]
/// use std::arch::x86_64::*;
///
/// # fn main() {
/// #     if is_x86_feature_detected!("sse4.2") {
/// #         #[target_feature(enable = "sse4.2")]
/// #         unsafe fn worker() {
/// // Ensure your input is 16 byte aligned
/// let password = b"hunter2\0\0\0\0\0\0\0\0\0";
/// let special_chars = b"!@#$%^&*()[]:;<>";
///
/// // Load the input
/// let a = _mm_loadu_si128(special_chars.as_ptr() as *const _);
/// let b = _mm_loadu_si128(password.as_ptr() as *const _);
///
/// // Use _SIDD_CMP_EQUAL_ANY to find the index of any bytes in b
/// let idx = _mm_cmpistri(a.into(), b.into(), _SIDD_CMP_EQUAL_ANY);
///
/// if idx < 16 {
///     println!("Congrats! Your password contains a special character");
///     # panic!("{:?} does not contain a special character", password);
/// } else {
///     println!("Your password should contain a special character");
/// }
/// #         }
/// #         unsafe { worker(); }
/// #     }
/// # }
/// ```
///
/// Find the index of the first character in the haystack that is within a
/// range of characters.
///
/// ```
/// # #![feature(cfg_target_feature)]
/// # #![feature(target_feature)]
/// # #![feature(stdsimd)]
/// # #![cfg_attr(not(dox), no_std)]
/// # #[cfg(not(dox))]
/// # #[macro_use]
/// # extern crate std as real_std;
/// # #[cfg(not(dox))]
/// # #[macro_use]
/// # extern crate stdsimd as std;
/// #[cfg(target_arch = "x86")]
/// use std::arch::x86::*;
/// #[cfg(target_arch = "x86_64")]
/// use std::arch::x86_64::*;
///
/// # fn main() {
/// #     if is_x86_feature_detected!("sse4.2") {
/// #         #[target_feature(enable = "sse4.2")]
/// #         unsafe fn worker() {
/// # let b = b":;<=>?@[\\]^_`abc";
/// # let b = _mm_loadu_si128(b.as_ptr() as *const _);
///
/// // Specify the ranges of values to be searched for [A-Za-z0-9].
/// let a = b"AZaz09\0\0\0\0\0\0\0\0\0\0";
/// let a = _mm_loadu_si128(a.as_ptr() as *const _);
///
/// // Use _SIDD_CMP_RANGES to find the index of first byte in ranges.
/// // Which in this case will be the first alpha numeric byte found
/// // in the string.
/// let idx = _mm_cmpistri(a, b, _SIDD_CMP_RANGES);
///
/// if idx < 16 {
///     println!("Found an alpha numeric character");
///     # assert_eq!(idx, 13);
/// } else {
///     println!("Did not find an alpha numeric character");
/// }
/// #         }
/// #         unsafe { worker(); }
/// #     }
/// # }
/// ```
///
/// Working with 16-bit characters.
///
/// ```
/// # #![feature(cfg_target_feature)]
/// # #![feature(target_feature)]
/// # #![feature(stdsimd)]
/// # #![cfg_attr(not(dox), no_std)]
/// # #[cfg(not(dox))]
/// # #[macro_use]
/// # extern crate std as real_std;
/// # #[cfg(not(dox))]
/// # #[macro_use]
/// # extern crate stdsimd as std;
/// #[cfg(target_arch = "x86")]
/// use std::arch::x86::*;
/// #[cfg(target_arch = "x86_64")]
/// use std::arch::x86_64::*;
///
/// # fn main() {
/// #     if is_x86_feature_detected!("sse4.2") {
/// #         #[target_feature(enable = "sse4.2")]
/// #         unsafe fn worker() {
/// # let mut some_utf16_words = [0u16; 8];
/// # let mut more_utf16_words = [0u16; 8];
/// # '❤'.encode_utf16(&mut some_utf16_words);
/// # '𝕊'.encode_utf16(&mut more_utf16_words);
/// // Load the input
/// let a = _mm_loadu_si128(some_utf16_words.as_ptr() as *const _);
/// let b = _mm_loadu_si128(more_utf16_words.as_ptr() as *const _);
///
/// // Specify _SIDD_UWORD_OPS to compare words instead of bytes, and
/// // use _SIDD_CMP_EQUAL_EACH to compare the two strings.
/// let idx = _mm_cmpistri(a, b, _SIDD_UWORD_OPS | _SIDD_CMP_EQUAL_EACH);
///
/// if idx == 0 {
///     println!("16-bit unicode strings were equal!");
///     # panic!("Strings should not be equal!")
/// } else {
///     println!("16-bit unicode strings were not equal!");
/// }
/// #         }
/// #         unsafe { worker(); }
/// #     }
/// # }
/// ```
///
/// [`_SIDD_UBYTE_OPS`]: constant._SIDD_UBYTE_OPS.html
/// [`_SIDD_UWORD_OPS`]: constant._SIDD_UWORD_OPS.html
/// [`_SIDD_SBYTE_OPS`]: constant._SIDD_SBYTE_OPS.html
/// [`_SIDD_SWORD_OPS`]: constant._SIDD_SWORD_OPS.html
/// [`_SIDD_CMP_EQUAL_ANY`]: constant._SIDD_CMP_EQUAL_ANY.html
/// [`_SIDD_CMP_RANGES`]: constant._SIDD_CMP_RANGES.html
/// [`_SIDD_CMP_EQUAL_EACH`]: constant._SIDD_CMP_EQUAL_EACH.html
/// [`_SIDD_CMP_EQUAL_ORDERED`]: constant._SIDD_CMP_EQUAL_ORDERED.html
/// [`_SIDD_POSITIVE_POLARITY`]: constant._SIDD_POSITIVE_POLARITY.html
/// [`_SIDD_NEGATIVE_POLARITY`]: constant._SIDD_NEGATIVE_POLARITY.html
/// [`_SIDD_LEAST_SIGNIFICANT`]: constant._SIDD_LEAST_SIGNIFICANT.html
/// [`_SIDD_MOST_SIGNIFICANT`]: constant._SIDD_MOST_SIGNIFICANT.html
/// [`_mm_cmpestri`]: fn._mm_cmpestri.html
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(pcmpistri, imm8 = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_cmpistri(a: __m128i, b: __m128i, imm8: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmpistri(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Compare packed strings with implicit lengths in `a` and `b` using the
/// control in `imm8`, and return `1` if any character in `b` was null.
/// and `0` otherwise.
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(pcmpistri, imm8 = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_cmpistrz(a: __m128i, b: __m128i, imm8: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmpistrz(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Compare packed strings with implicit lengths in `a` and `b` using the
/// control in `imm8`, and return `1` if the resulting mask was non-zero,
/// and `0` otherwise.
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(pcmpistri, imm8 = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_cmpistrc(a: __m128i, b: __m128i, imm8: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmpistrc(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Compare packed strings with implicit lengths in `a` and `b` using the
/// control in `imm8`, and returns `1` if any character in `a` was null,
/// and `0` otherwise.
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(pcmpistri, imm8 = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_cmpistrs(a: __m128i, b: __m128i, imm8: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmpistrs(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Compare packed strings with implicit lengths in `a` and `b` using the
/// control in `imm8`, and return bit `0` of the resulting bit mask.
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(pcmpistri, imm8 = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_cmpistro(a: __m128i, b: __m128i, imm8: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmpistro(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Compare packed strings with implicit lengths in `a` and `b` using the
/// control in `imm8`, and return `1` if `b` did not contain a null
/// character and the resulting mask was zero, and `0` otherwise.
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(pcmpistri, imm8 = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_cmpistra(a: __m128i, b: __m128i, imm8: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmpistra(::mem::transmute(a), ::mem::transmute(b), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Compare packed strings in `a` and `b` with lengths `la` and `lb`
/// using the control in `imm8`, and return the generated mask.
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(pcmpestrm, imm8 = 0))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm_cmpestrm(a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i32) -> __m128i {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmpestrm(::mem::transmute(a), ::mem::transmute(la), ::mem::transmute(b), ::mem::transmute(lb), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Compare packed strings `a` and `b` with lengths `la` and `lb` using the
/// control in `imm8` and return the generated index. Similar to
/// [`_mm_cmpistri`] with the exception that [`_mm_cmpistri`] implicitly
/// determines the length of `a` and `b`.
///
/// # Control modes
///
/// The control specified by `imm8` may be one or more of the following.
///
/// ## Data size and signedness
///
///  - [`_SIDD_UBYTE_OPS`] - Default
///  - [`_SIDD_UWORD_OPS`]
///  - [`_SIDD_SBYTE_OPS`]
///  - [`_SIDD_SWORD_OPS`]
///
/// ## Comparison options
///  - [`_SIDD_CMP_EQUAL_ANY`] - Default
///  - [`_SIDD_CMP_RANGES`]
///  - [`_SIDD_CMP_EQUAL_EACH`]
///  - [`_SIDD_CMP_EQUAL_ORDERED`]
///
/// ## Result polarity
///  - [`_SIDD_POSITIVE_POLARITY`] - Default
///  - [`_SIDD_NEGATIVE_POLARITY`]
///
/// ## Bit returned
///  - [`_SIDD_LEAST_SIGNIFICANT`] - Default
///  - [`_SIDD_MOST_SIGNIFICANT`]
///
/// # Examples
///
/// ```
/// # #![feature(cfg_target_feature)]
/// # #![feature(target_feature)]
/// # #![feature(stdsimd)]
/// # #![cfg_attr(not(dox), no_std)]
/// # #[cfg(not(dox))]
/// # #[macro_use]
/// # extern crate std as real_std;
/// # #[cfg(not(dox))]
/// # #[macro_use]
/// # extern crate stdsimd as std;
/// #[cfg(target_arch = "x86")]
/// use std::arch::x86::*;
/// #[cfg(target_arch = "x86_64")]
/// use std::arch::x86_64::*;
///
/// # fn main() {
/// #     if is_x86_feature_detected!("sse4.2") {
/// #         #[target_feature(enable = "sse4.2")]
/// #         unsafe fn worker() {
///
/// // The string we want to find a substring in
/// let haystack = b"Split \r\n\t line  ";
///
/// // The string we want to search for with some
/// // extra bytes we do not want to search for.
/// let needle = b"\r\n\t ignore this ";
///
/// let a = _mm_loadu_si128(needle.as_ptr() as *const _);
/// let b = _mm_loadu_si128(haystack.as_ptr() as *const _);
///
/// // Note: We explicitly specify we only want to search `b` for the
/// // first 3 characters of a.
/// let idx = _mm_cmpestri(a, 3, b, 15, _SIDD_CMP_EQUAL_ORDERED);
///
/// assert_eq!(idx, 6);
/// #         }
/// #         unsafe { worker(); }
/// #     }
/// # }
/// ```
///
/// [`_SIDD_UBYTE_OPS`]: constant._SIDD_UBYTE_OPS.html
/// [`_SIDD_UWORD_OPS`]: constant._SIDD_UWORD_OPS.html
/// [`_SIDD_SBYTE_OPS`]: constant._SIDD_SBYTE_OPS.html
/// [`_SIDD_SWORD_OPS`]: constant._SIDD_SWORD_OPS.html
/// [`_SIDD_CMP_EQUAL_ANY`]: constant._SIDD_CMP_EQUAL_ANY.html
/// [`_SIDD_CMP_RANGES`]: constant._SIDD_CMP_RANGES.html
/// [`_SIDD_CMP_EQUAL_EACH`]: constant._SIDD_CMP_EQUAL_EACH.html
/// [`_SIDD_CMP_EQUAL_ORDERED`]: constant._SIDD_CMP_EQUAL_ORDERED.html
/// [`_SIDD_POSITIVE_POLARITY`]: constant._SIDD_POSITIVE_POLARITY.html
/// [`_SIDD_NEGATIVE_POLARITY`]: constant._SIDD_NEGATIVE_POLARITY.html
/// [`_SIDD_LEAST_SIGNIFICANT`]: constant._SIDD_LEAST_SIGNIFICANT.html
/// [`_SIDD_MOST_SIGNIFICANT`]: constant._SIDD_MOST_SIGNIFICANT.html
/// [`_mm_cmpistri`]: fn._mm_cmpistri.html
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(pcmpestri, imm8 = 0))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm_cmpestri(a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmpestri(::mem::transmute(a), ::mem::transmute(la), ::mem::transmute(b), ::mem::transmute(lb), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Compare packed strings in `a` and `b` with lengths `la` and `lb`
/// using the control in `imm8`, and return `1` if any character in
/// `b` was null, and `0` otherwise.
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(pcmpestri, imm8 = 0))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm_cmpestrz(a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmpestrz(::mem::transmute(a), ::mem::transmute(la), ::mem::transmute(b), ::mem::transmute(lb), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Compare packed strings in `a` and `b` with lengths `la` and `lb`
/// using the control in `imm8`, and return `1` if the resulting mask
/// was non-zero, and `0` otherwise.
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(pcmpestri, imm8 = 0))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm_cmpestrc(a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmpestrc(::mem::transmute(a), ::mem::transmute(la), ::mem::transmute(b), ::mem::transmute(lb), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Compare packed strings in `a` and `b` with lengths `la` and `lb`
/// using the control in `imm8`, and return `1` if any character in
/// a was null, and `0` otherwise.
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(pcmpestri, imm8 = 0))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm_cmpestrs(a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmpestrs(::mem::transmute(a), ::mem::transmute(la), ::mem::transmute(b), ::mem::transmute(lb), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Compare packed strings in `a` and `b` with lengths `la` and `lb`
/// using the control in `imm8`, and return bit `0` of the resulting
/// bit mask.
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(pcmpestri, imm8 = 0))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm_cmpestro(a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmpestro(::mem::transmute(a), ::mem::transmute(la), ::mem::transmute(b), ::mem::transmute(lb), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Compare packed strings in `a` and `b` with lengths `la` and `lb`
/// using the control in `imm8`, and return `1` if `b` did not
/// contain a null character and the resulting mask was zero, and `0`
/// otherwise.
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(pcmpestri, imm8 = 0))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm_cmpestra(a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i32) -> i32 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_cmpestra(::mem::transmute(a), ::mem::transmute(la), ::mem::transmute(b), ::mem::transmute(lb), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Starting with the initial value in `crc`, return the accumulated
/// CRC32 value for unsigned 8-bit integer `v`.
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(crc32))]
pub unsafe fn _mm_crc32_u8(crc: u32, v: u8) -> u32 {
    ::mem::transmute(::myarch::_mm_crc32_u8(::mem::transmute(crc), ::mem::transmute(v)))
}

/// Starting with the initial value in `crc`, return the accumulated
/// CRC32 value for unsigned 16-bit integer `v`.
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(crc32))]
pub unsafe fn _mm_crc32_u16(crc: u32, v: u16) -> u32 {
    ::mem::transmute(::myarch::_mm_crc32_u16(::mem::transmute(crc), ::mem::transmute(v)))
}

/// Starting with the initial value in `crc`, return the accumulated
/// CRC32 value for unsigned 32-bit integer `v`.
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(crc32))]
pub unsafe fn _mm_crc32_u32(crc: u32, v: u32) -> u32 {
    ::mem::transmute(::myarch::_mm_crc32_u32(::mem::transmute(crc), ::mem::transmute(v)))
}

/// Compare packed 64-bit integers in `a` and `b` for greater-than,
/// return the results.
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(pcmpgtq))]
pub unsafe fn _mm_cmpgt_epi64(a: i64x2, b: i64x2) -> i64x2 {
    ::mem::transmute(::myarch::_mm_cmpgt_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

