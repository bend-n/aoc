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
pub use util::prelude::*;

#[no_mangle]
pub fn p1(x: &str) -> impl Display {
    let x = [
        [22usize, 8, 165],
        [8, 17, 114],
        [18, 6, 103],
        [25, 6, 145],
        [11, 12, 125],
        [21, 6, 121],
        [18, 3, 50],
        [20, 4, 75],
        [7, 20, 119],
    ];
    let mut deer = x.map(|deer| {
        repeat_n(deer[0], deer[1])
            .chain(repeat_n(0, deer[2]))
            .cycle()
    });
    dbg!(deer
        .clone()
        .into_iter()
        .map(|x| x.take(2503).sum::<usize>())
        .max()
        .unwrap());
    let mut states = [0; 9];
    let mut points = [0; 9];
    for _ in 0..2503 {
        deer.iter_mut()
            .ι::<usize>()
            .for_each(|(deer, i)| states[i] += deer.next().unwrap());
        let max = states.into_iter().max().unwrap();
        states
            .into_iter()
            .ι::<usize>()
            .filter(|&(x, _)| x == max)
            .for_each(|(_, i)| points[i] += 1);
    }
    points.into_iter().max().unwrap()
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
