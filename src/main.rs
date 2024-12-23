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
    iter_repeat_n,
    slice_swap_unchecked,
    generic_const_exprs,
    iter_array_chunks,
    if_let_guard,
    const_mut_refs,
    get_many_mut,
    maybe_uninit_uninit_array,
    once_cell_get_mut,
    iter_collect_into,
    hint_assert_unchecked,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    vec_into_raw_parts,
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
use atools::{ArrayTools, CollectArray as _, Join, Pop};
pub use util::prelude::*;
#[no_mangle]
fn changes(mut x: u32) -> [(u8, i8); 2001] {
    let mut secret = x;
    x = mod10(x);
    std::array::from_fn(|_| {
        secret = next(secret);
        let n = mod10(secret);
        let v = (x as u8, (n as i64 - x as i64) as i8);
        x = n;
        v
    })
}

pub fn mod10(a: u32) -> u32 {
    const D: u32 = 10;
    const M: u64 = (u64::MAX / D as u64) + 1;
    (M.wrapping_mul(a as u64) as u128 * D as u128 >> 64) as u32
}

fn next(mut x: u32) -> u32 {
    x ^= (x * 64) & 16777215;
    x ^= x / 32;
    x ^= (x * 2048) & 16777215;
    x
}

#[rustfmt::skip]
// 8051
fn next2000(n: u32) -> u32 {
    let n = n as u64;
    let m = n | n << 24;
    let r = (m & 0x61a765) ^ (m >> 1 & 0xc2f82d) ^ (m >> 2 & 0x286d53) ^ (m >> 3 & 0x44f679)
    ^ (m >> 4 & 0x4d6be8) ^ (m >> 5 & 0x118005) ^ (m >> 6 & 0x5f19f2) ^ (m >> 7 & 0xf03667)
    ^ (m >> 8 & 0xcea653) ^ (m >> 9 & 0xafa201) ^ (m >> 10 & 0xfd0d29) ^ (m >> 11 & 0x949200)
    ^ (m >> 12 & 0x49a994) ^ (m >> 13 & 0x21673) ^ (m >> 14 & 0xb4c5bf) ^ (m >> 15 & 0x1e0aaf)
    ^ (m >> 16 & 0x7cab00) ^ (m >> 17 & 0x95ba48) ^ (m >> 18 & 0x49f04c) ^ (m >> 19 & 0x9a8320)
    ^ (m >> 20 & 0xb69d39) ^ (m >> 21 & 0x6a2085) ^ (m >> 22 & 0xd13c84) ^ (m >> 23 & 0x1c9e15);
    r as u32

}

#[no_mangle]
pub fn run(x: &str) -> impl Display {
    let mut i = x.as_bytes();
    let mut seen = vec![0; 130321];
    let mut map = vec![0; 130321];
    reading::Integer::<u32>::new(&mut i)
        .ι1::<usize>()
        .for_each(|(mut x, j)| {
            let f @ [_, _, _, 三] = std::iter::successors(Some(x), |&f| Some(next(f))).carr();
            let [mut ⅰ, mut ⅱ, mut ⅲ, mut ⅳ]: [u32; 4] = 0u32.join(
                f.windowed::<2>()
                    .map(|&[a, b]| (9 + mod10(b) - mod10(a)) as u32),
            );

            x = 三;
            let mut l = mod10(三);
            for _ in 3..2000 {
                x = next(x);
                let p = x % 10;

                (ⅰ, ⅱ, ⅲ, ⅳ) = (ⅱ, ⅲ, ⅳ, (9 + p - l) as u32);
                let i = (ⅰ * 19 * 19 * 19 + ⅱ * 19 * 19 + ⅲ * 19 + ⅳ) as usize;
                if seen[i] != j {
                    map[i] += p as u16;
                    seen[i] = j;
                }
                l = p;
            }
        });
    map.into_iter().max().ψ()
}

use std::simd::prelude::*;
#[no_mangle]
pub fn p1(x: &str) -> impl Display {
    let mut x = x.as_bytes();
    let mut i = reading::Integer::<u32>::new(&mut x).array_chunks::<8>();
    i.by_ref()
        .map(|x| u32x8::from_array(x.map(next2000)))
        .fold(u32x8::splat(0), |acc, x| acc + x)
        .cast::<u64>()
        .reduce_sum()
        + i.into_remainder()
            .map_or(0, |x| x.map(next2000).map(|x| x as u64).sum())
}

fn main() {
    let s = include_str!("inp.txt");
    println!("{}", unsafe { run(s) });
    // dbg!(exec(&program, regi));
}
#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { run(i) });
}
