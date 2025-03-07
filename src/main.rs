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
use std::simd::prelude::*;
pub use util::prelude::*;

#[no_mangle]
pub fn p1(x: &str) -> impl Display {
    let x = [
        [2i64, 0, -2, 0], //3],
        [0, 5, -3, 0],    //3],
        [0, 0, 5, -1],    //8],
        [0, -1, 0, 5],    //8],
    ]
    .map(i64x4::from);
    let cals = [3i64, 3, 8, 8];

    itertools::iproduct!(0i64..=100, 0..=100, 0..=100, 0..=100)
        .map(|x| x.array())
        .filter(|x| x.sum() == 100)
        .filter(|x| x.zip(range()).map(|(n, i)| n * cals[i]).sum() == 500)
        .map(|y| {
            y.zip(range())
                .map(|(n, i)| x[i] * i64x4::splat(n))
                .into_iter()
                // add vertically
                .sum::<i64x4>()
                // max(0)
                .simd_max(i64x4::splat(0))
                // multiply horizontally
                .reduce_product()
        })
        .max()
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
