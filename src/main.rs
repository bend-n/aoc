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

// https://stackoverflow.com/a/9721570
fn digs(x: u64) -> u64 {
    static powers: [u64; 20] = car::from_fn!(|x| 10u64.pow(x as u32));
    static mdigs: [u32; 65] = car::from_fn!(|x| 2u128.pow(x as u32).ilog10() + 1);
    let bit = std::mem::size_of::<u64>() * 8 - x.leading_zeros() as usize;
    let mut digs = mdigs[bit];
    if x < powers[digs as usize - 1] {
        digs -= 1;
    }
    powers[digs as usize]
}

fn search(mut nums: &[u64], should: u64, current: u64, ops: &[fn(u64, u64) -> u64]) -> bool {
    let Some(&a) = nums.take_first() else {
        return current == should;
    };
    for op in ops {
        let result = op(current, a);
        if result > should {
            continue;
        }
        if search(nums, should, result, ops) {
            return true;
        }
    }
    false
}

#[inline]
fn do_(ops: &[fn(u64, u64) -> u64], i: &str) -> u64 {
    let mut v = Vec::with_capacity(10);
    i.行()
        .map(|x| {
            v.clear();
            let (a, b) = x.μ(':');
            let should = reading::all(a);
            reading::κ(C! { &b[1..] }, &mut v);
            let mut nums = &*v;
            let &first = nums.take_first().ψ();
            should * search(nums, should, first, ops) as u64
        })
        .sum::<u64>()
}

#[no_mangle]
pub fn run(i: &str) -> impl Display {
    do_(
        &[
            (std::ops::Add::<u64>::add),
            (std::ops::Mul::<u64>::mul as fn(u64, u64) -> u64),
        ],
        i,
    )
}

#[no_mangle]
pub fn p2(i: &str) -> impl Display {
    do_(
        &[
            (std::ops::Add::<u64>::add),
            (std::ops::Mul::<u64>::mul as fn(u64, u64) -> u64),
            |a, b| a * digs(b) + b,
            // |a, b| a * 10u64.pow(b.checked_ilog10().ψ() + 1) + b,
        ],
        i,
    )
}

fn main() {
    // (1..u32::MAX as u64).for_each(|a| assert_eq!(a.ilog10() + 1, digs(a)));
    // let mut s = String::new();
    // for i in 0..1280 {
    let i = include_str!("inp.txt");
    //     s.push_str(i);
    // }
    // std::fs::write("src/inp.txt", s);
    assert_eq!(run(i).to_string(), 663613490587u64.to_string());
    assert_eq!(p2(i).to_string(), 110365987435001u64.to_string());
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| run(i));
}
