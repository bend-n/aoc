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
use atools::CollectArray;
use std::cmp::Reverse;
pub use util::prelude::*;

const WIDTH: usize = 70;
pub unsafe fn run(x: &str) -> impl Display {
    let mut grid = [[0; WIDTH + 1]; WIDTH + 1];
    let mut add = x.行().map(|x| {
        let (x, y) = x.μ(',').mb(|x| x.λ::<u8>());
        (x, y)
    });
    loop {
        let x = add.next().unwrap();
        grid[x.1.nat()][x.0.nat()] = 1;
        if util::dijkstra(
            |x: (usize, usize)| {
                [Dir::N + x, Dir::E + x, Dir::S + x, Dir::W + x]
                    .into_iter()
                    .filter(|&(x, y)| grid.get(y).and_then(|y| y.get(x)) == Some(&0))
                    .map(|x| (x, 1))
            },
            (0, 0),
            |x| x == (WIDTH, WIDTH),
        )
        .is_none()
        {
            return format!("{x:?}");
        }
    }

    // 0
}
fn main() {
    let s = include_str!("inp.txt");
    println!("{}", unsafe { run(s) });
    // dbg!(exec(&program, regi));
}
