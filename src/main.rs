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
    let input1 = [
        50, 44, 11, 49, 42, 46, 18, 32, 26, 40, 21, 7, 18, 43, 10, 47, 36, 24, 22, 40,
    ];
    for n in 0.. {
        let z = input1
            .iter()
            .permutations(n)
            .any(|x| x.iter().copied().sum::<usize>() == 150);
        if z {
            return input1
                .iter()
                .combinations(n)
                .filter(|x| x.iter().copied().sum::<usize>() == 150)
                .count();
        }
    }

    let mut c = vec![0; 151];
    c[0] = 1;
    input1
        .into_iter()
        .for_each(|n| (n..=150).rev().for_each(|i| c[i] += c[i - n]));
    return c[150];
    0
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
