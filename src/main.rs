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
use std::sync::OnceLock;

pub use util::prelude::*;

#[no_mangle]
pub fn run(i: &str) -> impl Display {
    let i = i.as_bytes();
    let mut input = i.κ::<u64>().collect_vec();
    input.reserve_exact(30000);
    for _ in 0..25 {
        let i = input.clone();
        unsafe { input.set_len(0) };
        for stone in i {
            match stone {
                0 => input.push(1),
                n if let ͱ = n.ͱ() as usize
                    && ͱ % 2 == 0 =>
                {
                    let pow = util::powers[ͱ / 2];
                    input.push(n % pow);
                    input.push(n / pow);
                }
                n => input.push(n * 2024),
            }
        }
    }
    input.len()
}

#[no_mangle]
pub unsafe fn p2(i: &str) -> impl Display {
    let i = i.as_bytes().trim_ascii_end();
    let mut o = [0u64; 10];
    let n = reading::κ(i, &mut o);
    static mut map: OnceLock<HashMap<(u64, u8), u64>> = OnceLock::new();
    unsafe {
        match map.get_mut() {
            Some(x) => x.clear(),
            None => drop(map.get_or_init(|| {
                HashMap::with_capacity_and_hasher(150000, rustc_hash::FxBuildHasher::default())
            })),
        }
    };
    fn rocc(one: u64, iters: u8) -> u64 {
        if let Some(&x) = unsafe { map.get_mut().ψ().get(&(one, iters)) } {
            return x;
        }
        let answer = {
            match iters.checked_sub(1) {
                Some(iters) if one == 0 => rocc(1, iters),
                Some(iters)
                    if let ͱ = one.ͱ() as usize
                        && ͱ % 2 == 0 =>
                {
                    let pow = util::powers[ͱ / 2];
                    rocc(one / pow, iters) + rocc(one % pow, iters)
                }
                Some(iters) if iters > 1 && (one * 2024).ͱ() % 2 == 1 => {
                    // skip
                    let one = one * 2024 * 2024;
                    let pow = util::powers[one.ͱ() as usize / 2];
                    rocc(one / pow, iters - 2) + rocc(one % pow, iters - 2)
                }
                Some(iters) => rocc(one * 2024, iters),
                None => 1,
            }
        };
        unsafe { map.get_mut().ψ() }.insert((one, iters), answer);
        answer
    }
    o.into_iter()
        .take(n)
        .map(|stone| rocc(stone, 75))
        .sum::<u64>()
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
    println!("{}", unsafe { p2(i) });
    // println!("{}", p1(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p2(i) });
}
