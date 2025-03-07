#![allow(
    confusable_idents,
    uncommon_codepoints,
    non_upper_case_globals,
    internal_features,
    mixed_script_confusables,
    static_mut_refs,
    incomplete_features
)]
#![feature(
    stdarch_x86_avx512,
    iter_partition_in_place,
    iter_chain,
    slice_swap_unchecked,
    generic_const_exprs,
    ptr_sub_ptr,
    iter_array_chunks,
    slice_from_ptr_range,
    if_let_guard,
    maybe_uninit_uninit_array,
    once_cell_get_mut,
    iter_collect_into,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    vec_into_raw_parts,
    try_blocks,
    slice_take,
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
use regex::bytes::Regex;
use std::simd::prelude::*;
pub use util::prelude::*;

#[no_mangle]
pub fn p1(x: &str) -> impl Display {
    let input1 = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1];
    #[rustfmt::skip]
    let res = ["children","cats","samoyeds","pomeranians","akitas","vizslas","goldfish","trees","cars","perfumes",];
    let res = res.map(|x| &*Box::leak(Box::new(Regex::new(&format!("{x}: ([0-9]+)")).unwrap())));
    x.行()
        .ι1::<usize>()
        .map_l(move |inp| {
            res.map(|x| {
                x.captures(inp)
                    .map(|x| x.get(1).unwrap().as_bytes().λ::<u64>())
            })
        })
        .fl(move |x| x[1].is_none_or(|y| y > input1[1]))
        .fl(move |x| x[7].is_none_or(|y| y > input1[7]))
        .fl(move |x| x[3].is_none_or(|y| y < input1[3]))
        .fl(move |x| x[6].is_none_or(|y| y < input1[6]))
        .fl(move |x| {
            [0, 2, 4, 5, 8, 9]
                .into_iter()
                .all(|i| x[i].is_none_or(|x| input1[i] == x))
        })
        .next()
        .ψ()
        .1
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
