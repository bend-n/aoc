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
    slice_swap_unchecked,
    generic_const_exprs,
    iter_array_chunks,
    if_let_guard,
    get_many_mut,
    maybe_uninit_uninit_array,
    once_cell_get_mut,
    iter_collect_into,
    hint_assert_unchecked,
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
    core_intrinsics
)]
extern crate test;
pub mod util;
use std::sync::OnceLock;

pub use util::prelude::*;

#[no_mangle]
pub unsafe fn p1(i: &str) -> impl Display {
    let i = i.as_bytes().trim_ascii_end();
    let mut o = [0u32; 8];
    const LUT: [u32; 10000000] = unsafe {
        std::mem::transmute::<[u8; 10000000 * 4], _>(*include_bytes!("../beeg2-larger-basic"))
    };
    reading::κ(i, &mut o);
    o.into_iter().map(|stone| C!{ LUT[stone as usize] }).sum::<u32>()
}

#[no_mangle]
pub unsafe fn p2(i: &str) -> impl Display {
    let i = i.as_bytes().trim_ascii_end();
    let mut o = [0u64; 8];
    const LUT: [u64; 10000000] =
        unsafe { std::mem::transmute::<[u8; 10000000 * 8], _>(*include_bytes!("../beeg-basic")) };
    reading::κ(i, &mut o);
    o.into_iter().map(|stone| C! { LUT[stone as usize] }).sum::<u64>()
}

fn main() {
    // (1..u32::MAX as u64).for_each(|a| assert_eq!(a.ilog10() + 1, digs(a)));
    // let mut s = String::new();
    // for i in 0..1280 {
    let i = include_str!("inp.txt");
    //     s.push_str(i);i
    // }
    // std::fs::write("src/inp.txt", s);
    println!("{}", unsafe { p1(i) });
    println!("{}", unsafe { p2(i) });
    // println!("{}", p1(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
