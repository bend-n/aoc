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
    step_trait,
    cmp_minmax,
    custom_inner_attributes,
    extend_one,
    slice_as_array,
    impl_trait_in_bindings,
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

use atools::{CollectArray, prelude::*};
use itertools::chain;
use lower::apply;
use md5::{Digest, Md5};
use memchr::memmem;
use regex::bytes::Regex;
use rustc_hash::FxBuildHasher;
use std::{
    cmp::{Reverse, minmax},
    mem::take,
    simd::prelude::*,
};
use swizzle::array;

pub use util::prelude::*;
#[unsafe(no_mangle)]

pub unsafe fn p1(i: &'static str) -> impl Display {
    let l = i
        .行()
        .skip(2)
        .map(|x| util::ints(x).carr::<6>().drop::<2>())
        // .inspect(|x| println!("{x:?}"))
        .carr::<{ 25 * 32 }>()
        .chunked::<25>();

    for y in 0..25 {
        for x in 0..32 {
            let element = l[x][y];
            print!(
                "{}",
                if element[3] == 0 {
                    "_"
                } else if element[0] > 120 {
                    "#"
                } else {
                    "."
                }
            );
        }
        println!()
    }
    0
    // l.iter()
    //     .flat_map(|x| l.iter().map(move |y| (x, y)))
    //     .filter(|(a, _)| a[1] != 0)
    //     .filter(|(a, b)| a != b)
    //     .filter(|(a, b)| a[1] <= b[2])
    //     .count()
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
