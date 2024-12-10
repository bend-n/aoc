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
    get_many_mut,
    maybe_uninit_uninit_array,
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

const SIZE: usize = 50;

#[no_mangle]
pub fn p2(i: &str) -> impl Display {
    let i = i.as_bytes().trim_ascii_end();
    let mut files = Vec::with_capacity(10000);
    let mut free = Vec::with_capacity(10000);
    let mut j = 0;
    i.iter().ι::<usize>().for_each(|(x, i)| {
        let n = *x - b'0';
        if i % 2 == 1 {
            free.push((n, j));
        } else {
            files.push((n, j));
        }
        j += n as u32;
    });

    for (size, fat) in files.iter_mut().rev() {
        let Some((si, &(space, at))) = free
            .iter()
            .enumerate()
            .take_while(|(_, &(_, j))| j <= *fat)
            .find(|(_, &(s, _))| s >= *size)
        else {
            continue;
        };
        free[si as usize] = (space - *size, at + *size as u32);
        *fat = at;
    }
    files
        .into_iter()
        .ι::<u64>()
        .map(|((size, at), n)| ((size as u64, at as u64), n as u64))
        .map(|((size, at), n)| n * (at * size + (size * (size - 1)) / 2))
        .sum::<u64>()
}

#[no_mangle]
pub fn run(i: &str) -> impl Display {
    let i = i.as_bytes().trim_ascii_end();
    const SPACE: u16 = u16::MAX;
    let map = i
        .iter()
        .ι::<u16>()
        .flat_map(|(x, i)| {
            let times = (*x - b'0') as usize;
            std::iter::repeat_n(if i % 2 == 1 { SPACE } else { i / 2 }, times)
        })
        .collect_vec();
    let (map, len, _) = map.into_raw_parts();
    let eight_bit = unsafe { std::slice::from_raw_parts(map as *const u8, len * 2) };
    let mut emptys = memchr::memmem::find_iter(eight_bit, &[0xff; 2]).map(|x| x / 2);
    for i in (0..len).rev() {
        if unsafe { *map.add(i) == SPACE } {
            continue;
        }
        let empty = emptys.Δ();
        if empty > i {
            break;
        }
        unsafe { map.add(empty).swap(map.add(i)) };
    }
    unsafe { std::slice::from_raw_parts(map, memchr::memmem::find(eight_bit, &[0xff; 2]).ψ() / 2) }
        .iter()
        .copied()
        .ι::<usize>()
        .map(|(id, i)| id as usize * i)
        .sum::<usize>()
    // 0
}

fn main() {
    // (1..u32::MAX as u64).for_each(|a| assert_eq!(a.ilog10() + 1, digs(a)));
    // let mut s = String::new();
    // for i in 0..1280 {
    let i = include_str!("inp.txt");
    //     s.push_str(i);
    // }
    // std::fs::write("src/inp.txt", s);
    println!("{}", p2(i));
    println!("{}", run(i));
    // println!("{}", p1(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| run(i));
}
