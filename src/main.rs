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
    type_alias_impl_trait,
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
use itertools::chain;
use lower::apply;
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

use crate::util::{MapF, UnionFind};
#[implicit_fn::implicit_fn]
fn kh(x: impl Iterator<Item = u8> + Clone) -> [u8; 32] {
    let lengths = x.map(_ as usize).chain([17, 31, 73, 47, 23]);

    let mut x = range::<256>();
    let mut c = 0;
    let mut s = 0;

    for _ in 0..64 {
        for l in lengths.clone() {
            if (c + l) > 256 {
                let mut i = x
                    .into_iter()
                    .cycle()
                    .skip(c)
                    .take(l)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev();
                x.get_mut(c..(c + l).min(256))
                    .map(|s| s.copy_from_slice(&i.by_ref().take(s.len()).collect::<Vec<_>>()));
                x[..c + l - 256].copy_from_slice(&i.by_ref().take(c + l - 256).collect::<Vec<_>>());
                assert_eq!(i.collect::<Vec<_>>(), &[]);
            } else {
                x.get_mut(c..(c + l).min(256)).map(_.reverse());
            }

            c += l + s;
            c %= 256;
            s += 1;
        }
    }

    x.chunked::<16>()
        .map(|x| x.into_iter().reduce(_ ^ _).ψ() as u8)
        .map(|x| [x >> 4, x & 0x0f])
        .flatten()
}

#[unsafe(no_mangle)]
#[implicit_fn::implicit_fn]
pub unsafe fn p1(x: &'static [u8; ISIZE]) -> impl Debug {
    let g = [618, 814u64];
    let a = infinite_successors(g[0], |x| (x * 16807) % 2147483647).filter(|x| x % 4 == 0);
    let b = infinite_successors(g[1], |x| (x * 48271) % 2147483647).filter(|x| x % 8 == 0);

    a.zip(b)
        .take(5_000_000)
        .filter(|&(a, b)| a & 0xffff == b & 0xffff)
        .count()
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
