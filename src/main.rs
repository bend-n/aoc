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
    get_many_mut,
    maybe_uninit_uninit_array,
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
use util::countg;
pub use util::prelude::*;

const SIZE: usize = "30010966763987014589210001456565589765432".len();

#[no_mangle]
pub fn run(i: &str) -> impl Display {
    let grid = unsafe { i.as_bytes().as_chunks_unchecked::<{ SIZE + 1 }>() };
    let get =
        |x: u8, y: u8| (x < SIZE as u8 && y < SIZE as u8).then(|| grid[y as usize][x as usize]);

    // fimg::Image::<Vec<u8>, 1>::build(SIZE as _, SIZE as _)
    //     .buf(
    //         grid.iter()
    //             .flat_map(|x| &x[..SIZE])
    //             .map(|x| (((*x - b'0') as f32 / 10.0) * 255.0) as u8)
    //             .collect_vec(),
    //     )
    //     .scale::<fimg::scale::Nearest>(SIZE as u32 * 8, SIZE as u32 * 8)
    //     .save("hmm.png");
    let mut tot = 0;
    for y in 0..SIZE as u8 {
        for x in 0..SIZE as u8 {
            if get(x, y) == Some(b'0') {
                util::countg_with_check(
                    (x, y),
                    &mut |(x, y)| {
                        [
                            (x.wrapping_add(1), y),
                            (x.wrapping_sub(1), y),
                            (x, y.wrapping_add(1)),
                            (x, y.wrapping_sub(1)),
                        ]
                        .into_iter()
                    },
                    &mut |(x1, y1), (x2, y2)| {
                        let x: Option<bool> = try { get(x2, y2)?.checked_sub(get(x1, y1)?)? == 1 };
                        x.unwrap_or(false)
                    },
                    &mut tot,
                    &mut |(x, y)| get(x, y) == Some(b'9'),
                    // &mut HashSet::default(),
                );
            }
        }
    }
    tot
    // count
}

fn main() {
    // (1..u32::MAX as u64).for_each(|a| assert_eq!(a.ilog10() + 1, digs(a)));
    // let mut s = String::new();
    // for i in 0..1280 {
    let i = include_str!("inp.txt");
    //     s.push_str(i);
    // }
    // std::fs::write("src/inp.txt", s);
    // println!("{}", p2(i));
    println!("{}", run(i));
    // println!("{}", p1(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| run(i));
}
