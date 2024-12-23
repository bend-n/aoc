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
pub use util::prelude::*;
#[no_mangle]
fn changes(mut x: u32) -> [(u8, i8); 2001] {
    let mut secret = x;
    x %= 10;
    std::array::from_fn(|_| {
        secret ^= (secret * 64) % 16777216;
        secret ^= (secret / 32) % 16777216;
        secret ^= (secret * 2048) % 16777216;
        let n = secret % 10;
        let v = (x as u8, (n as i64 - x as i64) as i8);
        x = n;
        v
    })
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
    let i = x.as_bytes();
    let mut map = HashMap::<[i8; 4], u16>::with_capacity_and_hasher(
        50000,
        rustc_hash::FxBuildHasher::default(),
    );
    let mut seen =
        HashSet::<[i8; 4]>::with_capacity_and_hasher(2000, rustc_hash::FxBuildHasher::default());
    i.行().map(reading::all::<u32>).map(changes).for_each(|x| {
        for &[elem @ .., (p, _)] in x.array_windows::<5>() {
            let elem = elem.map(|x| x.1);
            if seen.insert(elem) {
                *map.entry(elem).or_default() += p as u16;
            }
        }
        seen.clear();
    });
    map.into_iter().r().max().ψ()
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
    println!("{}", unsafe { p1(s) });
    // dbg!(exec(&program, regi));
}
#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
