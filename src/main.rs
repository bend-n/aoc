#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(
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
mod util;
pub use util::prelude::*;

fn weigh(mat: Vec<[u8; 100]>) -> usize {
    mat.iter()
        .ι::<usize>()
        .map(|(row, i)| util::count::<100>(&row, b'O') * (100 - i))
        .sum()
}

#[no_mangle]
pub fn run(i: &str) -> impl Display {
    let mut v = Vec::with_capacity(100);
    let mut i = i.as_bytes();
    for _ in 0..100 {
        v.push(unsafe { <&[u8; 100]>::try_from(i.get_unchecked(..100)).unwrap() });
        i = unsafe { i.get_unchecked(100..) };
        if i.len() != 0 {
            i = unsafe { i.get_unchecked(1..) };
        }
    }
    weigh(pushhh(v))
}

fn pushhh(mat: Vec<&[u8; 100]>) -> Vec<[u8; 100]> {
    let mut new = vec![[0; 100]; 100];
    for j in 0..100 {
        let mut count = 0;
        for i in 0..100 {
            if *unsafe { mat.get_unchecked(i).get_unchecked(j) } == b'O' {
                *unsafe { new.get_unchecked_mut(count).get_unchecked_mut(j) } = b'O';
                count += 1;
            } else if mat[i][j] == b'#' {
                count = i + 1;
            }
        }
    }
    new
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
