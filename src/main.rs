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
use atools::ArrayTools::sort;
pub use util::prelude::*;

#[no_mangle]
pub unsafe fn p1(x: &str) -> impl Display {
    x.行()
        .map(|x| x.κ::<u16>().carr::<3>())
        .filter(|tri| (2 * tri.iter().max().unwrap()) < tri.sum())
        .count()
}

#[no_mangle]
pub unsafe fn p2(x: &str) -> impl Display {
    x.行()
        .map(|x| x.κ::<u16>().carr::<3>())
        .array_chunks::<3>()
        .map(|x| mattr::transposed::<_, 3, 3>(x.flatten()).chunked::<3>())
        .flatten()
        .map(sort)
        .filter(|&[a, b, c]| a + b > c)
        .count()
}

fn main() {
    unsafe { println!("{}", p2(include_str!("inp.txt"))) };
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
