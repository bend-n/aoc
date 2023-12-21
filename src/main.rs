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

pub fn search(i: &[&[u8]]) -> (u8, u8) {
    for (row, y) in i.iter().ι::<u8>() {
        if let Some((_, x)) = row.iter().ι::<u8>().find(|&(&x, _)| x == b'S') {
            return (x, y);
        }
    }
    dang!();
}

pub fn run(i: &str) -> impl Display {
    let i = i.行().collect_vec();
    let i = i.as_slice();
    let mut n = 0;
    let (x, y) = search(i);
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

fn main() {
    let i = include_str!("inp.txt").trim();
    println!("{}", run(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| run(i));
}
