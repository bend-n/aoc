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
    iter_chain,
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
    array_chunks,
    slice_split_once,
    import_trait_associated_functions,
    core_intrinsics
)]
extern crate test;
pub mod util;

use atools::{CollectArray, prelude::*};
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
#[implicit_fn::implicit_fn]
pub unsafe fn p1(i: &'static str) -> impl Display {
    let x = std::iter::successors(Some("01111010110010011".as_bytes().to_vec()), |a| {
        Some(
            a.iter()
                .copied()
                .chain([b'0'])
                .chain(a.iter().rev().copied().map(|x| match x {
                    b'0' => b'1',
                    _ => b'0',
                }))
                .collect::<Vec<u8>>(),
        )
    })
    .find(_.len() > 35651584)
    .unwrap();

    String::from_utf8_unchecked(
        successors(Some(x[..35651584].to_vec()), |x| {
            Some(
                x.array_chunks::<2>()
                    .map(|x| match x {
                        b"00" | b"11" => b'1',
                        _ => b'0',
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .skip(1)
        .find(_.len() % 2 != 0)
        .unwrap(),
    )
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
