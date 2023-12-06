#![allow(confusable_idents, uncommon_codepoints)]
#![feature(array_windows, test, slice_as_chunks, array_chunks)]
extern crate test;
mod util;
pub use util::prelude::*;

const S: char = ' ';

fn solve(i: &str) -> impl Display {
    let mut lines = i.lines();
    let time = lines.Δ().μ1(':').ͷ().fold(0, |acc, n| acc * 10 + n as u64);
    let distance = lines.Δ().μ1(':').ͷ().fold(0, |acc, n| acc * 10 + n as u64);
    let mut works = 0;
    for held in 0..time {
        let gone = held * (time - held);
        if gone > distance {
            works += 1;
        }
    }
    works
}

fn main() {
    let i = include_str!("inp.txt").trim();
    println!("{}", solve(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| solve(i));
}
