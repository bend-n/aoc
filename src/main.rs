#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(array_windows, test, slice_as_chunks, array_chunks)]
extern crate test;
mod util;
pub use explicit_cast::prelude::*;
pub use util::prelude::*;

fn solve(i: &str) -> impl Display {
    let mut lines = i.lines();
    let line = lines.by_ref().Δ().as_bytes();
    let map = lines
        .skip(1)
        .map(|x| {
            x.μ('=')
                .mr(|x| x.trim()[1..x.len() - 2].μ(',').ml(str::trim).mr(str::trim))
                .ml(str::trim)
        })
        .collect::<HashMap<_, _>>();
    let mut position = "AAA";
    let mut steps = 1;
    for &instruction in line.iter().cycle() {
        println!("{position}");
        let at = map[position];
        position = match instruction {
            b'L' => at.0,
            b'R' => at.1,
            _ => dang!(),
        };
        if position == "ZZZ" {
            return steps;
        }
        steps += 1;
    }
    dang!();
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
