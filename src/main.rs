#![allow(
    confusable_idents,
    uncommon_codepoints,
    non_upper_case_globals,
    internal_features,
    mixed_script_confusables,
    static_mut_refs,
    incomplete_features,
    unused_imports,
    redundant_semicolons
)]
#![feature(
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
use lower::apply;
use memchr::memmem;
use regex::bytes::Regex;
use std::simd::prelude::*;
pub use util::prelude::*;

#[allow(warnings)]
type u32x3 = Simd<u32, 3>;

#[no_mangle]
pub fn p1(x: &'static str) -> impl Display {
    let (ro, co) = (2947u64, 3029u64);
    // count diagonals
    let point = (0..ro + co - 1).sum::<u64>() + co;
    infinite_successors(20151125u64, |x| (x * 252533).rem_euclid(33554393))
        .nth(point as usize - 1)
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
