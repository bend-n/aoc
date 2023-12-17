#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(
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
pub use util::prelude::*;

// const MUST: u8 = 3;
// const NEEDS: u8 = 0;
const MUST: u8 = 10;
const NEEDS: u8 = 4;

fn neighbor(
    (x, y, δx, δy, gone): (u8, u8, i8, i8, u8),
    v: &[[u8; 142]; 141],
) -> impl Iterator<Item = ((u8, u8, i8, i8, u8), u16)> + '_ {
    [
        Dir::W + (x, y),
        Dir::E + (x, y),
        Dir::N + (x, y),
        Dir::S + (x, y),
    ]
    .into_iter()
    .flatten()
    .filter(|&(x, _)| x < 141)
    .filter(|&(_, y)| y < 141)
    .filter_map(move |(nx, ny)| {
        let go;
        let nδx = (nx as i16 - x as i16) as i8;
        let nδy = (ny as i16 - y as i16) as i8;
        if nδx == δx && nδy == δy {
            if gone == MUST {
                return None;
            }
            go = gone + 1;
        } else if (nδx == -δx && nδx != 0) || (nδy == -δy && nδy != 0) {
            return None;
        } else {
            if gone < NEEDS {
                return None;
            }
            go = 1;
        }

        Some((
            (nx, ny, nδx, nδy, go),
            (C! { v[ny as usize][nx as usize] } - b'0') as u16,
        ))
    })
}

pub fn run(i: &str) -> impl Display {
    let v = unsafe { &*(i.as_bytes().as_ptr() as *const [[u8; 142]; 141]) };
    util::dijkstra(
        |n| neighbor(n, v),
        (0u8, 0u8, 0i8, 0i8, NEEDS),
        |(x, y, _, _, g)| x == 140 && y == 140 && g >= NEEDS,
    )
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
