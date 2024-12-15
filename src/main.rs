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
pub use util::prelude::*;

const W: i32 = 101;
const H: i32 = 103;
#[no_mangle]
pub fn run(i: &str) -> impl Display {
    let mut grids = [0; 4];
    i.行()
        .map(|x| {
            let ((px, py), (vx, vy)) = x.μ(' ').mb(|x| x.μ1('=').μκ::<i32>(',').Δ());
            let x = (px + vx * 100).rem_euclid(W);
            let y = (py + vy * 100).rem_euclid(H);
            let w = W / 2;
            let h = H / 2;
            if x < w && y < h {
                grids[0] += 1;
            } else if x < w && y > h {
                grids[1] += 1;
            } else if x > w && y < h {
                grids[2] += 1;
            } else if x > w && y > h {
                grids[3] += 1;
            }
        })
        .Θ();
    grids.iter().product::<u32>()
}

#[no_mangle]
pub fn p2(i: &str) -> impl Display {
    let mut positions = Vec::<((i32, i32), (i32, i32))>::with_capacity(500);
    const W: i32 = 101;
    const H: i32 = 103;
    i.行().for_each(|x| {
        positions.push(x.μ(' ').mb(|x| x.μ1('=').μκ::<i32>(',').Δ()));
    });
    let bx = (0..W)
        .map(|seconds| {
            positions
                .iter()
                .map(move |&((x, _), (vx, _))| (x + vx * seconds).rem_euclid(W).abs_diff(W / 2))
                .sum::<u32>()
        })
        .enumerate()
        .min_by_key(|&(_, x)| x)
        .unwrap()
        .0 as i32;
    let by = (0..H)
        .map(|seconds| {
            positions
                .iter()
                .map(move |&((_, x), (_, vx))| (x + vx * seconds).rem_euclid(H).abs_diff(H / 2))
                .sum::<u32>()
        })
        .enumerate()
        .min_by_key(|&(_, x)| x)
        .unwrap()
        .0 as i32;
    bx + ((51 * (by - bx)) % H) * W
}

fn main() {
    // (1..u32::MAX as u64).for_each(|a| assert_eq!(a.ilog10() + 1, digs(a)));
    // let mut s = String::new();
    // for i in 0..1280 {
    let i = include_str!("inp.txt");
    //     s.push_str(i);i
    // }
    // std::fs::write("src/inp.txt", s);
    #[allow(unused_unsafe)]
    println!("{}", unsafe { p2(i) });
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { run(i) });
}
