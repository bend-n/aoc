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
    let mut max = 0;
    util::iterg(
        ((0, 0), Vec::<Dir>::new()),
        &mut |(po, pa): ((usize, usize), Vec<Dir>)| {
            let open = (util::md5s(
                &[
                    &b"udskfozm"[..],
                    &pa.iter().map(_.turdl()).collect::<Vec<_>>(),
                ]
                .concat(),
            ))
            .bytes()
            .carr::<4>()
            .map(|x| matches!(x, b'b' | b'c' | b'd' | b'e' | b'f'));

            ([Dir::N, Dir::S, Dir::W, Dir::E])
                .into_iter()
                .zip(open)
                .filter(_.1)
                .flat_map(move |(d, _)| d.lim_add(po, [0, 4], [0, 4]).map(|x| (x, d)))
                .map(move |(x, d)| (x, pa.iter().copied().chain([d]).collect::<Vec<_>>()))
        },
        &mut |x| x.0 == (3, 3),
        &mut |x| max = max.max(x.1.len()),
    );
    max

    // .unwrap()
    // .1
    // .1
    // .iter()
    // .map(_.turdl())
    // .collect::<Vec<_>>(),
    //    String::from_utf8_unchecked(
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
