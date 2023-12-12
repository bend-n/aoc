#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(
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
use std::ops::ControlFlow;

pub use util::prelude::*;

pub fn run(i: &str) -> impl Display {
    i.行()
        .map(|x| {
            // AAA?
            let (r, c) = x
                .μ(' ')
                .mr(|x| x.split(|&x| x == b',').κ::<u8>().collect::<Box<_>>());
            fn kill_me(
                s: Vec<u8>,
                n: usize,
                max: usize,
                p: &mut impl FnMut(&[u8]) -> ControlFlow<(), bool>,
            ) -> Vec<Vec<u8>> {
                if n == max {
                    return vec![s];
                }
                let mut combinations = vec![];
                for c in [b'.', b'#'] {
                    let mut s = s.clone();
                    s.push(c);
                    for thing in kill_me(s, n + 1, max, p) {
                        match p(&thing) {
                            ControlFlow::Break(()) => break,
                            ControlFlow::Continue(true) => combinations.push(thing),
                            _ => {}
                        }
                    }
                }
                combinations
            }
            let why = kill_me(vec![], 0, r.len(), &mut |x| {
                for (&x, &y) in r.iter().zip(x.iter()) {
                    if matches!(x, b'.' | b'#' if y != x) {
                        return ControlFlow::Break(());
                    }
                }
                let mut huh = vec![];
                let mut len = 0;
                for &x in x {
                    match x {
                        b'#' => len += 1,
                        b'.' if len != 0 => {
                            huh.push(len);
                            len = 0;
                        }
                        _ => {}
                    }
                }
                if len != 0 {
                    huh.push(len);
                }
                ControlFlow::Continue(&*huh == &*c)
            })
            .len();
            why
        })
        .sum::<usize>()
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
