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
fn split(x: usize) -> (u8, u8) {
    ((x / (SIZE + 1)) as u8, (x % (SIZE + 1)) as u8)
}
#[no_mangle]
pub fn run(i: &str) -> impl Display {
    let i = i.as_bytes();
    let mut big: [[(u8, u8); 4]; 123] = [[(0, 0); 4]; 123];
    let mut lengths = [0; 123];
    for (row_, i) in unsafe { i.as_chunks_unchecked::<{ SIZE + 1 }>() }
        .iter()
        .ι::<u8>()
    {
        let row = u8x64::load_or_default(row_);
        let mut row = row.simd_ne(Simd::splat(b'.')).to_bitmask() & ((1 << 50) - 1);
        while row != 0 {
            let x = row.trailing_zeros();
            row &= !(1 << x);
            let &el = &row_[x as usize];
            let l = unsafe { lengths.get_unchecked_mut(el as usize) };
            C! { big[el as usize][*l] = (i, x as u8) };
            *l += 1;
        }
    }
    let mut anti = [0u64; SIZE];
    for char in (b'A'..=b'Z').chain(b'a'..=b'z').chain(b'0'..=b'9') {
        // let memchr = memchr::Memchr::new(char, i);
        // let wat = memchr.map(split).collect_vec();
        let all = unsafe {
            big.get_unchecked(char as usize)
                .get_unchecked(..*lengths.get_unchecked(char as usize))
        };
        let mut anti = |(x1, y1), (x2, y2)| {
            let mut x3 = x2 + (x2 - x1);
            let mut y3 = y2 + (y2 - y1);
            *C! { &mut anti[y2 as usize] } |= 1 << x2 as u64;
            while (x3 < SIZE as u8) & (y3 < SIZE as u8) {
                anti[y3 as usize] |= 1 << x3 as u64;
                x3 += x2 - x1;
                y3 += y2 - y1;
            }
        };
        match all {
            &[] => continue,
            &[a, b, c, d] => {
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
            &[a, b, c] => {
                let mut one = |i: (u8, u8), j: (u8, u8)| {
                    anti(i, j);
                    anti(j, i);
                };
                one(b, a);
                one(c, a);
                one(c, b);
            }
            _ => shucks!(),
        }
    }
    anti.into_iter().map(u64::count_ones).sum::<u32>()
}
use std::simd::prelude::*;
#[no_mangle]
pub fn p1(i: &str) -> u32 {
    let mut big: [[(u8, u8); 4]; 123] = [[(0, 0); 4]; 123];
    let mut lengths = [0; 123];
    let i = i.as_bytes();
    for (row_, i) in unsafe { i.as_chunks_unchecked::<{ SIZE + 1 }>() }
        .iter()
        .ι::<u8>()
    {
        let row = u8x64::load_or_default(row_);
        let mut row = row.simd_ne(Simd::splat(b'.')).to_bitmask() & ((1 << 50) - 1);
        while row != 0 {
            let x = row.trailing_zeros();
            row &= !(1 << x);
            let &el = &row_[x as usize];
            let l = unsafe { lengths.get_unchecked_mut(el as usize) };
            C! { big[el as usize][*l] = (i, x as u8) };
            *l += 1;
        }
    }
    let mut anti = [0u64; SIZE];
    for char in (b'A'..=b'Z').chain(b'a'..=b'z').chain(b'0'..=b'9') {
        // let memchr = memchr::Memchr::new(char, i);
        // let wat = memchr.map(split).collect_vec();
        let all = unsafe {
            big.get_unchecked(char as usize)
                .get_unchecked(..*lengths.get_unchecked(char as usize))
        };
        // assert_eq!(wat, all);
        let mut anti = |(x1, y1), (x2, y2): (u8, u8)| {
            let x3 = x2.wrapping_add(x2.wrapping_sub(x1));
            let y3 = y2.wrapping_add(y2.wrapping_sub(y1));
            if (x3 < SIZE as u8) & (y3 < SIZE as u8) {
                anti[y3 as usize] |= 1 << x3 as u64;
            }
        };
        match all.len() {
            0 => continue,
            4 => {
                for i in 0..4 {
                    for j in 0..i {
                        let i = all[i];
                        let j = all[j];
                        anti(i, j);
                        anti(j, i);
                    }
                }
            }
            3 => {
                for i in 0..3 {
                    for j in 0..i {
                        let i = all[i];
                        let j = all[j];
                        anti(i, j);
                        anti(j, i);
                    }
                }
            }
            _ => shucks!(),
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
