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

#[unsafe(no_mangle)]
// #[implicit_fn::implicit_fn]
pub unsafe fn p1(x: &'static [u8; ISIZE]) -> impl Debug {
    let mut p = range::<16>().map(|x| x as u8);
    // let mut set = HashMap::default();
    for (x, _) in x.μₙ(b',').cycle().ι::<usize>().take(280000) {
        match x[0] {
            b's' => {
                p.rotate_right(x[1..].λ());
            }
            b'x' => {
                let (a, b) = x[1..].μ('/').mb(|x| x.λ());
                p.swap(a, b);
            }
            b'p' => {
                let a = p.position(x[1] - b'a');
                let b = p.position(x[3] - b'a');
                p.swap(a, b);
            }
            _ => dang!(),
        }
        // if let Some(p) = set.insert(p, i) {
        //     println!(
        //         "{i} = {p}",
        //     );
        // }
    }
    p.map(|x| (x as u8 + b'a') as char)
        .iter()
        .collect::<String>()
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
