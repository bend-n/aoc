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
    iterator_try_reduce,
    cmp_minmax,
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
use collar::CollectArray;
use lower::apply;
use memchr::memmem;
use regex::bytes::Regex;
use std::{
    cmp::{Reverse, minmax},
    mem::take,
    simd::prelude::*,
};
pub use util::prelude::*;

fn status(x: u32, y: u32) -> bool {
    (x * x + 3 * x + 2 * x * y + y + y * y + 1350).count_ones() % 2 == 0
}

#[unsafe(no_mangle)]
#[implicit_fn::implicit_fn]
pub unsafe fn p1(x: &'static str) -> impl Display {
    //    dijkstra
    util::reachable(
        ((1u32, 1u32), 0),
        |((x, y), s)| {
            Dir::ALL
                .into_iter()
                .flat_map(move |d| d + (x, y))
                .filter(|&(x, y)| status(x, y))
                .map(move |x| (x, s + 1))
                .filter(_.1 <= 50)
        },
        // |x| (x == (31, 39)),
    )
    .len()
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
