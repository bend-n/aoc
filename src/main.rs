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
    iter_repeat_n,
    stdarch_x86_avx512,
    iter_partition_in_place,
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
pub use util::prelude::*;
fn area([l, w, h]: [u32; 3]) -> u32 {
    2 * l * w + 2 * w * h + 2 * h * l
}
#[no_mangle]
pub unsafe fn p1(i: &str) -> impl Display {
    for n in 0.. {
        let x = md5::compute(format!("iwrupvqb{n}"));
        if x.0.take::<3>() == [0x00; 3] {
            return n;
        }
    }
    0
}

#[no_mangle]
pub unsafe fn p2(i: &str) -> impl Display {
    i.行()
        .map(|x| x.str().split('x').map(|x| x.λ::<u32>()).carr::<3>())
        .map(|x| x.sort().take::<2>().sum() * 2 + x.product())
        .sum::<u32>()
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}

#[bench]
fn benc_sort(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p2(i) });
}
