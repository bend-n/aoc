#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(
    maybe_uninit_uninit_array,
    inline_const,
    slice_flatten,
    iter_collect_into,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    unchecked_math,
    array_windows,
    slice_take,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    byte_slice_trim_ascii
)]
extern crate test;
pub mod util;

use std::mem::MaybeUninit;

pub use util::prelude::*;

pub fn p2(i: &str) -> usize {
    let i = i.as_bytes();
    // RNG src
    let mut map: HashSet<(i64, i64)> = HashSet::from_iter([(65, 65)]);
    let mut p = MaybeUninit::uninit_array::<3>();
    let mut n = 0;
    for s in 0..328u64 {
        if s % 131 == 65 {
            p[n].write(map.len());
            n += 1;
        }
        map = [Dir::N, Dir::E, Dir::S, Dir::W]
            .into_iter()
            .flat_map(|x| map.iter().map(move |&y| x + y))
            .filter(|&(x, y)| {
                i[y.rem_euclid(131) as usize * 132 + x.rem_euclid(131) as usize] != b'#'
            })
            .collect();
    }
    let n = 26501365 / 131;
    let [a, b, c]: [usize; 3] = unsafe { rint(p) };
    a + n * (b - a) + n * (n - 1) / 2 * ((c - b) - (b - a))
}

pub fn p1(i: &str) -> usize {
    let i = i.行().collect_vec();
    let i = i.as_slice();
    let mut n = 0;
    let (x, y) = (65u8, 65u8);
    util::countg(
        (x, y, 0),
        &mut |(x, y, n)| {
            [
                Dir::N + (x, y),
                Dir::E + (x, y),
                Dir::S + (x, y),
                Dir::W + (x, y),
            ]
            .into_iter()
            .flatten()
            .filter(|&(x, _)| x < i.len() as u8)
            .filter(|&(_, y)| y < i.len() as u8)
            .filter(|(x, y)| i[y.nat()][x.nat()] != b'#')
            .map(move |(x, y)| (x, y, n + 1))
        },
        &mut n,
        &mut |(_, _, n)| n == 64,
        &mut HashSet::new(),
    );
    n
}

pub fn run(i: &str) -> impl Display {
    p2(i)
}

fn main() {
    let i = include_str!("inp.txt").trim();
    println!("{}", run(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| run(i));
}
