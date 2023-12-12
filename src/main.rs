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
use std::{io::Write, ops::ControlFlow};

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
                p: &mut impl FnMut(&[u8]) -> ControlFlow<(), ()>,
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
                            ControlFlow::Continue(()) => combinations.push(thing),
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
                let mut checked = 0;
                let mut len = 0;
                for &b in x {
                    match b {
                        b'#' => len += 1,
                        b'.' if len != 0 => {
                            match c.get(checked) {
                                Some(&x) if x == len => {}
                                _ => return ControlFlow::Break(()),
                            };
                            checked += 1;
                            len = 0;
                        }
                        _ => {}
                    }
                }
                if len != 0 {
                    match c.get(checked) {
                        Some(&x) if x == len => {}
                        _ => return ControlFlow::Break(()),
                    }
                    checked += 1;
                }
                if checked != c.len() {
                    return ControlFlow::Break(());
                }
                ControlFlow::Continue(())
            })
            .len();
            std::io::stdout().flush().unwrap();
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
