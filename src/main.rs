#![allow(
    confusable_idents,
    uncommon_codepoints,
    non_upper_case_globals,
    internal_features,
    mixed_script_confusables,
    static_mut_refs,
    incomplete_features
)]
#![feature(
    slice_swap_unchecked,
    generic_const_exprs,
    iter_array_chunks,
    get_many_mut,
    maybe_uninit_uninit_array,
    iter_collect_into,
    hint_assert_unchecked,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    try_blocks,
    slice_take,
    portable_simd,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    core_intrinsics
)]
extern crate test;
pub mod util;
pub use util::prelude::*;

const SIZE: usize = 50;
fn split(x: usize) -> (isize, isize) {
    (
        x as isize / (SIZE as isize + 1),
        x as isize % (SIZE as isize + 1),
    )
}
#[no_mangle]
pub fn run(i: &str) -> impl Display {
    let i = i.as_bytes();
    let mut anti = [0u64; SIZE];
    for character in (b'A'..=b'Z').chain(b'a'..=b'z').chain(b'0'..=b'9') {
        let mut memchr = memchr::Memchr::new(character, i);
        let a = match memchr.next() {
            None => continue,
            Some(a) => a,
        };
        let b = memchr.Δ();
        let c = memchr.Δ();
        let d = memchr.next();
        let mut anti = |(x1, y1), (x2, y2)| {
            let mut x3 = x2 + (x2 - x1);
            let mut y3 = y2 + (y2 - y1);
            *C! { &mut anti[y2 as usize] } |= 1 << x2 as u64;
            while (x3 >= 0) & (x3 < SIZE as isize) & (y3 >= 0) & (y3 < SIZE as isize) {
                anti[y3 as usize] |= 1 << x3 as u64;
                x3 += x2 - x1;
                y3 += y2 - y1;
            }
        };
        match d {
            Some(d) => {
                let [a, b, c, d] = [a, b, c, d].map(split);
                let mut one = |i, j| {
                    anti(i, j);
                    anti(j, i);
                };
                one(b, a);
                one(c, a);
                one(c, b);
                one(d, a);
                one(d, b);
                one(d, c);
            }
            None => {
                let [a, b, c] = [a, b, c].map(split);
                let mut one = |i: (isize, isize), j: (isize, isize)| {
                    anti(i, j);
                    anti(j, i);
                };
                one(b, a);
                one(c, a);
                one(c, b);
            }
        }
    }
    anti.into_iter().map(u64::count_ones).sum::<u32>()
}
#[no_mangle]
pub fn p1(i: &str) -> u32 {
    let i = i.as_bytes();
    let mut anti = [0u64; SIZE];
    for character in (b'A'..=b'Z').chain(b'a'..=b'z').chain(b'0'..=b'9') {
        let mut memchr = memchr::Memchr::new(character, i);
        let a = match memchr.next() {
            None => continue,
            Some(a) => a,
        };
        let b = memchr.Δ();
        let c = memchr.Δ();
        let d = memchr.next();
        let mut anti = |(x1, y1), (x2, y2)| {
            let x3 = x2 + (x2 - x1);
            let y3 = y2 + (y2 - y1);
            if (x3 >= 0) & (x3 < SIZE as isize) & (y3 >= 0) & (y3 < SIZE as isize) {
                anti[y3 as usize] |= 1 << x3 as u64;
            }
        };
        match d {
            Some(d) => {
                for i in 0..4 {
                    for j in 0..i {
                        let i = split([a, b, c, d][i]);
                        let j = split([a, b, c, d][j]);
                        anti(i, j);
                        anti(j, i);
                    }
                }
            }
            None => {
                for i in 0..3 {
                    for j in 0..i {
                        let i = split([a, b, c][i]);
                        let j = split([a, b, c][j]);
                        anti(i, j);
                        anti(j, i);
                    }
                }
            }
        }
    }
    anti.into_iter().map(u64::count_ones).sum::<u32>()
}

fn main() {
    // (1..u32::MAX as u64).for_each(|a| assert_eq!(a.ilog10() + 1, digs(a)));
    // let mut s = String::new();
    // for i in 0..1280 {
    let i = include_str!("inp.txt");
    //     s.push_str(i);
    // }
    // std::fs::write("src/inp.txt", s);
    // println!("{}", p2(i));
    println!("{}", run(i));
    println!("{}", p1(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| run(i));
}
