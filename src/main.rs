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
    const_mut_refs,
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
pub use util::prelude::*;

const SIZE: usize = 141;
pub fn run(x: &str) -> impl Display {
    let i = x.as_bytes();
    let g = unsafe { i.as_chunks_unchecked::<{ SIZE + 1 }>() };
    let start = memchr::memchr(b'S', i).ψ();
    let (y, x) = (start / (SIZE + 1), start % (SIZE + 1));
    let mut cost = [[0; SIZE]; SIZE];
    let base = pathfinding::directed::bfs::bfs(
        &(x, y),
        |&p| {
            [Dir::N + p, Dir::E + p, Dir::S + p, Dir::W + p]
                .into_iter()
                .filter(|&(x, y)| g[y][x] != b'#')
        },
        |&(x, y)| g[y][x] == b'E',
    )
    .unwrap();
    let n = base.len() as u32;
    for (&(x, y), i) in base.iter().ι::<u32>() {
        cost[y][x] = i;
    }
    let dst = n - 1;
    // use rayon::prelude::*;
    base.into_iter()
        .filter(|&(x, y)| g[y][x] != b'#')
        // .par_bridge()
        .filter_map(|(x, y)| {
            for (y_offset, x_offset) in include!("../offsets") {
                let (x2, y2) = (
                    (x as i32 + x_offset) as usize,
                    (y as i32 + y_offset) as usize,
                );
                if x2 < SIZE
                    && y2 < SIZE
                    && ((cost[y][x] + n - cost[y2][x2] - 1
                        + x_offset.abs() as u32
                        + y_offset.abs() as u32)
                        + 100)
                        <= dst
                {
                    return Some(());
                }
            }
            None
        })
        .count()
}

fn main() {
    let s = include_str!("inp.txt");
    println!("{}", unsafe { run(s) });
    // dbg!(exec(&program, regi));
}
#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { run(i) });
}
