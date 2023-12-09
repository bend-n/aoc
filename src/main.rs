#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(
    array_windows,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    byte_slice_trim_ascii
)]
extern crate test;
mod util;
use std::mem::ManuallyDrop;

pub use util::prelude::*;

pub fn p1(a: &[i32]) -> i32 {
    if !a[1..].iter().any(|&x| x != a[0]) {
        a[0]
    } else {
        *a.last().unwrap()
            + p1(&*ManuallyDrop::new(
                a.array_windows::<2>()
                    .map(|[b, a]| a - b)
                    .collect::<Box<[_]>>(),
            ))
    }
}

pub fn p2(a: &[i32]) -> i32 {
    if !a[1..].iter().any(|&x| x != a[0]) {
        a[0]
    } else {
        *a.first().unwrap()
            - p2(&*ManuallyDrop::new(
                a.array_windows::<2>()
                    .map(|[b, a]| a - b)
                    .collect::<Box<[_]>>(),
            ))
    }
}

pub fn run(i: &str) -> impl Display {
    i.行()
        .map(|x| p1(&*ManuallyDrop::new(x.κ().collect::<Box<[i32]>>())))
        .sum::<i32>()
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
