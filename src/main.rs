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
    slice_swap_unchecked,
    generic_const_exprs,
    iter_array_chunks,
    get_many_mut,
    maybe_uninit_uninit_array,
    iter_collect_into,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    try_blocks,
    slice_take,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    core_intrinsics
)]
extern crate test;
pub mod util;
use atools::prelude::*;
pub use util::prelude::*;
const SIZE: usize = 140;

#[no_mangle]
pub fn run(i: &str) -> impl Display {
    let grid = unsafe { i.as_bytes().as_chunks_unchecked::<{ SIZE + 1 }>() };
    let get = |x: isize, y: isize| -> Option<u8> {
        (x >= 0 && y >= 0 && x < SIZE as isize && y < SIZE as isize)
            .then(|| unsafe { grid.get_unchecked(y as usize)[x as usize] })
    };
    macro_rules! ck {
        ($x:expr,$y:expr, $eq:literal) => {{
            get($x, $y) == Some($eq)
        }};
    }
    let mut sum = 0;
    for (x, y) in C! { &grid[..SIZE] }.iter().enumerate().flat_map(|(y, e)| {
        memchr::memchr_iter(b'X', &e[..SIZE]).map(move |x| (x as isize, y as isize))
    }) {
        macro_rules! triple {
            ($x: literal, $y:literal) => {
                (ck!(x + ($x * 3), y + ($y * 3), b'S')
                    && ck!(x + ($x * 2), y + ($y * 2), b'A')
                    && ck!(x + $x, y + $y, b'M')) as u32
            };
        }
        sum += 0 // a
            + triple!(1, 0) + triple!(-1, 0)
            + triple!(0, 1) + triple!(0, -1)

            + triple!(1, 1) + triple!(-1, 1) + triple!(-1, -1) + triple!(1, -1);
    }
    sum
}

pub fn two(i: &str) -> impl Display {
    let grid = unsafe { i.as_bytes().as_chunks_unchecked::<{ SIZE + 1 }>() };
    let get = |x: isize, y: isize| -> Option<u8> {
        (x >= 0 && y >= 0 && x < SIZE as isize && y < SIZE as isize)
            .then(|| unsafe { grid.get_unchecked(y as usize)[x as usize] })
    };
    let mut sum = 0;
    for (x, y) in grid[..SIZE - 1]
        .iter()
        .enumerate()
        .skip(1)
        .flat_map(|(y, e)| {
            memchr::memchr_iter(b'A', &e[1..SIZE - 1]).map(move |x| (x as isize + 1, y as isize))
        })
    {
        // println!("{x} {y}");
        // for (y, x) in (0..SIZE as isize).flat_map(|x| (0..SIZE as isize).map(move |y| (x, y))) {
        // if get(x, y) == Some(b'A') {
        let n: Option<u32> = try {
            let a = get(x - 1, y - 1)?;
            let b = get(x + 1, y + 1)?;
            ((a == b'M' && b == b'S') || (a == b'S' && b == b'M')).then(|| {
                let a = get(x + 1, y - 1).ψ();
                let b = get(x - 1, y + 1).ψ();
                (a == b'M' && b == b'S') || (a == b'S' && b == b'M')
            })? as u32
        };
        sum += n.unwrap_or(0);
        // }
    }
    sum
}

fn main() {
    // let mut s = String::new();
    // for i in 0..1280 {
    let i = include_str!("inp.txt");
    //     s.push_str(i);
    // }
    // std::fs::write("src/inp.txt", s);
    // prinan!("{}", p1(i));
    println!("{}", run(i));
    println!("{}", two(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| run(i));
}
