#![allow(
    unexpected_cfgs,
    confusable_idents,
    uncommon_codepoints,
    non_upper_case_globals,
    internal_features,
    mixed_script_confusables,
    static_mut_refs,
    incomplete_features,
    unused_imports,
    unsafe_op_in_unsafe_fn,
    redundant_semicolons
)]
#![feature(
    extend_one,
    slice_as_array,
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
use std::{cmp::Reverse, simd::prelude::*};
pub use util::prelude::*;

#[allow(warnings)]
type u32x3 = Simd<u32, 3>;

#[unsafe(no_mangle)]
#[implicit_fn::implicit_fn]
pub unsafe fn p1(x: &'static str) -> impl Display {
    let mut grid = [[false; 50]; 6];
    x.行().for_each(|x| {
        match () {
            () if x.starts_with(b"rect") => {
                let (w, h) = x.μ1(' ').μ('x').mb(_.λ::<usize>());
                (0..h).for_each(grid[_][..w].fill(true));
            }
            () if x.starts_with(b"rotate") => {
                let [_, axis, point, _, amount] = x.μₙ(b' ').carr();
                let point = point.μ1('=').λ::<usize>();
                let amount = amount.λ::<usize>();
                match axis {
                    b"row" => grid[point].rotate_right(amount),
                    b"column" => {
                        let mut row: [bool; 6] = from_fn(grid[_][point]);
                        row.rotate_right(amount);
                        (0..6).for_each(|y| grid[y][point] = row[y])
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unreachable!(),
        }
        for element in grid {
            for x in element {
                print!("{}", b" #"[x as usize] as char);
            }
            println!()
        }
        println!()
    });
    grid.as_flattened().iter().filter(**_).count()
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
