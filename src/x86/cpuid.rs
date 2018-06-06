#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Returns the result of the `cpuid` instruction for a given `leaf` (`EAX`)
/// and
/// `sub_leaf` (`ECX`).
///
/// The highest-supported leaf value is returned by the first tuple argument of
/// [`__get_cpuid_max(0)`](fn.__get_cpuid_max.html). For leaves containung
/// sub-leaves, the second tuple argument returns the highest-supported
/// sub-leaf
/// value.
///
/// The [CPUID Wikipedia page][wiki_cpuid] contains how to query which
/// information using the `EAX` and `ECX` registers, and the interpretation of
/// the results returned in `EAX`, `EBX`, `ECX`, and `EDX`.
///
/// The references are:
/// - [Intel 64 and IA-32 Architectures Software Developer's Manual Volume 2:
///   Instruction Set Reference, A-Z][intel64_ref].
/// - [AMD64 Architecture Programmer's Manual, Volume 3: General-Purpose and
///   System Instructions][amd64_ref].
///
/// [wiki_cpuid]: https://en.wikipedia.org/wiki/CPUID
/// [intel64_ref]: http://www.intel.de/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-instruction-set-reference-manual-325383.pdf
/// [amd64_ref]: http://support.amd.com/TechDocs/24594.pdf
#[inline]
#[cfg_attr(test, assert_instr(cpuid))]
pub unsafe fn __cpuid_count(leaf: u32, sub_leaf: u32) -> CpuidResult {
    ::mem::transmute(::myarch::__cpuid_count(::mem::transmute(leaf), ::mem::transmute(sub_leaf)))
}

/// See [`__cpuid_count`](fn.__cpuid_count.html).
#[inline]
#[cfg_attr(test, assert_instr(cpuid))]
pub unsafe fn __cpuid(leaf: u32) -> CpuidResult {
    ::mem::transmute(::myarch::__cpuid(::mem::transmute(leaf)))
}

