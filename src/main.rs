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
use std::cmp::Reverse;
pub use util::prelude::*;

const SIZE: usize = 141;

#[no_mangle]
pub unsafe fn run(i: &str) -> impl Display {
    let i = i.as_bytes();
    let j = memchr::memchr(b'S', i).ψ();
    let (x, y) = (j % (SIZE + 1), j / (SIZE + 1));
    let grid = i.to_vec().leak().as_chunks_unchecked_mut::<{ SIZE + 1 }>();
    grid[y][x] = b'.';
    // let mut q = std::collections::BinaryHeap::with_capacity(1 << 10);
    // let mut s = HashSet::with_capacity_and_hasher(1 << 16, rustc_hash::FxBuildHasher::default());
    // q.push(Reverse((0u64, ((x, y), Dir::E))));
    // while let Some(Reverse((c, n @ ((x, y), dir)))) = q.pop() {
    //     if grid[y][x] == b'E' {
    //         return c;
    //     }
    //     if !s.insert(n) {
    //         continue;
    //     }
    //     for (n, d) in [
    //         ((dir + n.0, dir), 1),
    //         ((n.0, dir.turn_90()), 1000),
    //         ((n.0, dir.turn_90ccw()), 1000),
    //     ]
    //     .into_iter()
    //     .filter(|(((x, y), ..), _)| grid[*y][*x] != b'#')
    //     {
    //         if s.contains(&n) {
    //             continue;
    //         }
    //         q.push(Reverse((c + d, n)));
    //     }
    // }

    let (path, _) = pathfinding::directed::astar::astar_bag(
        &((x, y), Dir::E),
        |&(p, dir)| {
            let dir: Dir = dir;
            [
                ((dir + p, dir), 1),
                ((p, dir.turn_90()), 1000u64),
                ((p, dir.turn_90ccw()), 1000u64),
            ]
            .into_iter()
            .filter(|(((x, y), ..), _)| grid[*y][*x] != b'#')
        },
        |_| 0,
        |((x, y), ..)| grid[*y][*x] == b'E',
    )
    .unwrap();
    path.into_iter()
        .flat_map(|x| x.into_iter().map(|((x, y), _)| (x, y)))
        .collect::<HashSet<_>>()
        .len()
}

fn main() {
    // (1..u32::MAX as u64).for_each(|a| assert_eq!(a.ilog10() + 1, digs(a)));
    // let mut s = String::new();
    // for i in 0..1280 {
    let i = include_str!("inp.txt");
    //     s.push_str(i);i
    // }w
    // std::fs::write("src/inp.txt", s);
    #[allow(unused_unsafe)]
    println!("{}", unsafe { run(i) });
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p2(i) });
}
