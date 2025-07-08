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
use md5::{Digest, Md5};
use memchr::memmem;
use regex::bytes::Regex;
use rustc_hash::FxBuildHasher;
use std::{
    cmp::{Reverse, minmax},
    mem::take,
    simd::prelude::*,
};

pub use util::prelude::*;
#[unsafe(no_mangle)]
#[implicit_fn::implicit_fn]
pub unsafe fn p1(i: &'static str) -> impl Display {
    // let hash = |x: u32| util::md5s(format!("{i}{x}").as_bytes());
    let mut memo =
        HashMap::<u32, String>::with_capacity_and_hasher(92859, FxBuildHasher::default());
    macro_rules! hash {
        ($x:expr) => {{
            let x = $x;
            memo.entry(x)
                .or_insert_with(|| {
                    successors(format!("{i}{x}").into(), |x| Some(util::md5s(x.as_bytes())))
                        .nth(2017)
                        .unwrap()
                })
                .as_bytes()
        }};
    }
    // dbg!(hash(0));
    (0u32..)
        .filter(|&x| {
            let Some(&c) = hash!(x)
                .array_windows::<3>()
                .find(|x| x.iter().all(|&y| y == x[0]))
            else {
                return false;
            };

            (x..=x + 1000).any(|x| memmem::find(hash!(x), &[c[0]; 5]).is_some())
        })
        .nth(74)
        .unwrap()
}
fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
