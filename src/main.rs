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
#[unsafe(no_mangle)]
#[implicit_fn::implicit_fn]
pub unsafe fn p1(x: &'static [u8; ISIZE]) -> impl Debug {
    let i = x.chunked::<{ 141 + 1 }>();
    assert_eq!(i.len(), 142);
    let start = i.find(b'S');

    util::memo_countg(
        start,
        |&(x, y)| {
            if i.get(y).is_none() {
                vec![]
            } else if i[y].get(x) == Some(&b'^') {
                vec![(x - 1, y + 1), (x + 1, y + 1)]
            } else {
                vec![(x, y + 1)]
            }
            .into_iter()
        },
        |&(_, y)| y == 141,
    )
}

const ISIZE: usize = include_bytes!("inp.txt").len();
fn main() {
    use atools::prelude::*;
    unsafe { println!("{:?}", rah::run(include_bytes!("inp.txt"))) };
    unsafe { println!("{:?}", rah::run(include_bytes!("../1"))) };
    unsafe { println!("{:?}", rah::run(include_bytes!("../2"))) };
    unsafe { println!("{:?}", rah::run(include_bytes!("../3"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_bytes!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
