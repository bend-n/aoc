#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(
    iter_collect_into,
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
mod util;

pub use util::prelude::*;

fn trans(v: Box<[&[u8]]>) -> Box<[Box<[u8]>]> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .copied()
                .collect::<Box<[u8]>>()
        })
        .collect()
}

pub fn run(i: &str) -> impl Display {
    let mut sum = 0;
    for lines in i.split("\n\n") {
        let this = lines.行().collect::<Box<_>>();
        'huh: for (&[a, b], i) in this.array_windows::<2>().ι::<usize>() {
            if a == b {
                let mut α = i;
                let mut β = i + 1;
                while α > 0 && β < this.len() - 1 {
                    α -= 1;
                    β += 1;
                    if this[β] != this[α] {
                        continue 'huh;
                    }
                }
                sum += (i + 1) * 100;
            }
        }
        let this = trans(this);
        'huh: for (&[ref a, ref b], i) in this.array_windows::<2>().ι::<usize>() {
            if a == b {
                let mut α = i;
                let mut β = i + 1;
                while α > 0 && β < this.len() - 1 {
                    α -= 1;
                    β += 1;
                    if this[β] != this[α] {
                        continue 'huh;
                    }
                }
                sum += i + 1;
            }
        }
    }
    sum
}

fn main() {
    let i = include_str!("inp.txt").trim();
    println!("{}", run(i));
}

// #[bench]
// fn bench(b: &mut test::Bencher) {
//     let i = boxd(include_str!("inp.txt").trim());
//     b.iter(|| run(i));
// }
