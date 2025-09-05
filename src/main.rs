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
    iter_from_coroutine,
    iterator_try_reduce,
    step_trait,
    cmp_minmax,
    custom_inner_attributes,
    pattern_types,
    pattern_type_macro,
    extend_one,
    slice_as_array,
    impl_trait_in_bindings,
    coroutines,
    stmt_expr_attributes,
    pattern_type_range_trait,
    const_trait_impl,
    coroutine_trait,
    iter_partition_in_place,
    slice_swap_unchecked,
    generic_const_exprs,
    iter_array_chunks,
    slice_from_ptr_range,
    if_let_guard,
    once_cell_get_mut,
    iter_collect_into,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    vec_into_raw_parts,
    try_blocks,
    portable_simd,
    test,
    slice_split_once,
    import_trait_associated_functions,
    core_intrinsics
)]
extern crate test;
pub mod util;

pub use atools::prelude::*;
use atools::{CollectArray, prelude::*};
use itertools::chain;
use lower::apply;
use md5::{Digest, Md5};
use memchr::memmem;
use regex::bytes::Regex;
use rustc_hash::FxBuildHasher;
use std::{
    cmp::{Reverse, minmax},
    hash::Hash,
    mem::take,
    ops::{Coroutine, Deref},
    pin::Pin,
    simd::prelude::*,
};
use swizzle::array;
pub use util::prelude::*;

#[allow(warnings)]
type u32x3 = Simd<u32, 3>;
#[unsafe(no_mangle)]
#[implicit_fn::implicit_fn]
pub unsafe fn p1(x: &'static [u8; ISIZE]) -> impl Debug {
    let mut v = util::ints(x).collect::<Vec<_>>();
    let mut p = 0i64;
    let mut s = 0;
    while v.len() > p.nat() {
        let n = v[p.nat()];
        v[p.nat()] += if n >= 3 { -1 } else { 1 };
        p += n;
        s += 1
    }
    s
}
const ISIZE: usize = include_bytes!("inp.txt").len();
fn main() {
    unsafe { println!("{:?}", p1(include_bytes!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_bytes!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
