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

pub use util::prelude::*;

pub fn run(i: &str) -> impl Display {
    let mut sum = 0;
    for lines in i.split("\n\n") {
        let mut this = [&b""[..]; 17];
        let put = lines.行().ν(&mut this);
        let this = &this[..put];
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
        let t = &this;
        'yes: for ((a, b), i) in (0..this[0].len())
            .map(|x| (0..this.len()).map(move |y| t[y][x]))
            .tuple_windows()
            .ι::<usize>()
        {
            if a.zip(b).all(|(a, b)| a == b) {
                let mut α = i;
                let mut β = i + 1;
                while α > 0 && β < this[0].len() - 1 {
                    α -= 1;
                    β += 1;
                    for i in 0..this.len() {
                        if this[i][β] != this[i][α] {
                            continue 'yes;
                        }
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

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| run(i));
}
