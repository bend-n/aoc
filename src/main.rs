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
    core_intrinsics
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
    hint::assert_unchecked,
    mem::take,
    ops::{Coroutine, Deref, RangeInclusive},
    pin::Pin,
    simd::prelude::*,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};
use swizzle::array;
pub use util::prelude::*;

use atools::prelude::*;
#[unsafe(no_mangle)]
#[implicit_fn::implicit_fn]
pub unsafe fn p1(x: &'static [u8; ISIZE]) -> impl Debug {
    let [range, nums] = x.splib(b"\n\n").carr();
    let mut ranges_: Vec<RangeInclusive<u64>> = vec![];
    for range in util::uints::<u64>(range)
        .array_chunks::<2>()
        .map(|[a, b]| a..=b)
        .sorted_by_key(|x| *x.start())
    {
        if let Some(find) = ranges_
            .iter()
            .position(|sr| sr.contains(range.start()) || sr.contains(range.end()))
        {
            let find = &mut ranges_[find];
            *find = *find.start().min(range.start())..=*find.end().max(range.end());
        } else {
            ranges_.push(range);
        }
    }
    ranges_.into_iter().map(|x| x.count()).sum::<usize>()
}

const ISIZE: usize = include_bytes!("inp.txt").len();
fn main() {
    use atools::prelude::*;
    unsafe { println!("{:?}", p1(include_bytes!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_bytes!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
