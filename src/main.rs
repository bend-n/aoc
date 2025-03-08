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
    const S: usize = 100;
    let mut g = unsafe { x.as_bytes().as_chunks_unchecked::<{ S + 1 }>() }.to_vec();
    for _ in 0..100 {
        [(0, 0), (99, 0), (0, 99), (99, 99)].map(|(x, y)| g[y][x] = b'#');
        let mut o = g.clone();
        for y in 0..S {
            for x in 0..S {
                let nb = util::nb(x, y)
                    .into_iter()
                    .filter_map(|(x, y)| g.get(y).and_then(|g| g.get(x)))
                    .filter(|&&e| e == b'#')
                    .count();
                match nb {
                    2 | 3 if g[y][x] == b'#' => continue,
                    3 if g[y][x] == b'.' => o[y][x] = b'#',
                    _ => o[y][x] = b'.',
                }
            }
        }
        g = o;
    }
    [(0, 0), (99, 0), (0, 99), (99, 99)].map(|(x, y)| g[y][x] = b'#');
    g.as_flattened().iter().filter(|&&x| x == b'#').count()
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
