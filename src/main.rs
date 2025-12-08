#![allow(
    overflowing_literals,
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
    const_cmp,
    int_roundings,
    type_alias_impl_trait,
    iter_from_coroutine,
    iterator_try_reduce,
    step_trait,
    cmp_minmax,
    custom_inner_attributes,
    pattern_types,
    pattern_type_macro,
    extend_one,
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
    try_blocks,
    portable_simd,
    test,
    slice_split_once,
    import_trait_associated_functions,
    core_intrinsics,
    gen_blocks
)]
extern crate test;
pub mod util;

use atools::Split;
// pub use atools::prelude::*;
use lower::apply;
use memchr::memmem;
use regex::bytes::Regex;
use rustc_hash::FxBuildHasher;
use std::{
    arch::x86_64::*,
    cmp::{Reverse, minmax},
    hash::Hash,
    hint::{assert_unchecked, unreachable_unchecked},
    mem::take,
    ops::{Coroutine, Deref, RangeInclusive},
    pin::Pin,
    simd::prelude::*,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};
use swizzle::array;
pub use util::prelude::*;
mod rah;
use atools::prelude::*;

use crate::util::UnionFind;
#[unsafe(no_mangle)]
#[implicit_fn::implicit_fn]
pub unsafe fn p1(x: &'static [u8; ISIZE]) -> impl Debug {
    let v = util::uints::<i64>(x).array_chunks::<3>().collect_vec();
    let k = v
        .iter()
        .copied()
        .ι::<usize>()
        .array_combinations::<2>()
        .sorted_by_key(|[(a, _), (b, _)]| {
            ((a[0] - b[0]).pow(2) + (a[1] - b[1]).pow(2) + (a[2] - b[2]).pow(2)).isqrt()
        });
    let mut uf = UnionFind::new(1000);
    for [a, b] in k.into_iter() {
        uf.union(a.1, b.1);
        // if (0..1000).map(|n| uf.find(n)).all_equal() {
        //     return v[a.1][0] * v[b.1][0];
        // }
    }
    // panic!()
    (0..1000)
        .map(|x| uf.group_size(x))
        .sorted()
        .rev()
        .take(3)
        .product::<usize>()
}

const ISIZE: usize = include_bytes!("inp.txt").len();
fn main() {
    use atools::prelude::*;
    unsafe { println!("{:?}", p1(include_bytes!("inp.txt"))) };
    // unsafe { println!("{:?}", rah::run(include_bytes!("../1"))) };
    // unsafe { println!("{:?}", rah::run(include_bytes!("../2"))) };
    // unsafe { println!("{:?}", rah::run(include_bytes!("../3"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_bytes!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
