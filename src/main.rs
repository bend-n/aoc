#![allow(
    confusable_idents,
    uncommon_codepoints,
    non_upper_case_globals,
    internal_features,
    mixed_script_confusables,
    static_mut_refs,
    incomplete_features,
    unused_imports,
    unsafe_op_in_unsafe_fn,
    redundant_semicolons
)]
#![feature(
    slice_as_array,
    stdarch_x86_avx512,
    impl_trait_in_bindings,
    iter_partition_in_place,
    iter_chain,
    slice_swap_unchecked,
    generic_const_exprs,
    iter_array_chunks,
    slice_from_ptr_range,
    if_let_guard,
    once_cell_get_mut,
    iter_collect_into,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    vec_into_raw_parts,
    try_blocks,
    portable_simd,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    import_trait_associated_functions,
    core_intrinsics
)]
extern crate test;
pub mod util;

use atools::prelude::*;
use lower::apply;
use md5::Digest;
use memchr::memmem;
use regex::bytes::Regex;
use std::{cmp::Reverse, simd::prelude::*};
pub use util::prelude::*;

#[allow(warnings)]
type u32x3 = Simd<u32, 3>;

fn md5(x: &[u8]) -> [u8; 16] {
    let mut hasher = md5::Md5::new();
    hasher.update(x);
    *hasher.finalize().as_array::<16>().unwrap()
}

#[unsafe(no_mangle)]
pub fn p1(_: &'static str) -> impl Display {
    (0..)
        .map(|n| md5(&format!("ojvtpuvg{n}").as_bytes()))
        .filter(|&[a, b, c, ..]| a == 0 && b == 0 && c >> 4 == 0)
        .map(|[_, _, x, ..]| b"0123456789abcdef"[(x & 0x0f).nat()] as char)
        .take(8)
        .collect::<String>()
}

#[unsafe(no_mangle)]
pub fn p2(_: &'static str) -> impl Display {
    let mut o = [0; 8];
    for [a, b, c, d, ..] in (0..).map(|n| md5(&format!("ojvtpuvg{n}").as_bytes())) {
        if a == 0 && b == 0 && c >> 4 == 0 && o.get((c & 0x0f) as usize) == Some(&0) {
            o[(c & 0x0f).nat()] = b"0123456789abcdef"[(d >> 4).nat()]
        }
        if o.iter().all(|x| *x != 0) {
            break;
        }
    }
    o.str().to_owned()
}

fn main() {
    unsafe { println!("{}", p2(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
