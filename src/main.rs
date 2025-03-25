#![allow(
    unexpected_cfgs,
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
    custom_inner_attributes,
    extend_one,
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
use memchr::memmem;
use regex::bytes::Regex;
use std::{cmp::Reverse, simd::prelude::*};
pub use util::prelude::*;

#[allow(warnings)]
type u32x3 = Simd<u32, 3>;

#[implicit_fn::implicit_fn]
fn expand(x: &[u8]) -> usize {
    let mut o = 0;
    let mut t = vec![];
    let mut i = 0;
    while i != x.len() {
        let c = x[i];
        i += 1;
        match c {
            b'(' => t.push(c),
            b')' => {
                let (range, amount) = t[1..].μ('x').mb(_.λ::<usize>());
                let n = expand(&x[i..i + range]);
                o += n * amount;
                i += range;
                t.clear();
            }
            c if !t.is_empty() => t.push(c),
            _ => o += 1,
        }
    }
    o
}

#[unsafe(no_mangle)]
#[implicit_fn::implicit_fn]
pub unsafe fn p1(x: &'static str) -> impl Display {
    let x = x.as_bytes();
    expand(x)
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
