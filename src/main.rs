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

const SIZE: usize = 10;

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
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

#[no_mangle]
pub fn run(i: &str) -> impl Display {
    i.行()
        .map(|x| {
            let (a, b) = x.μ(':');
            let should = a.λ::<u64>();
            let nums = b.κ::<u64>().collect_vec();
            let mut nums = &*nums;
            let &first = nums.take_first().ψ();
            should
                * search(
                    nums,
                    should,
                    first,
                    &[
                        (std::ops::Add::<u64>::add),
                        (std::ops::Mul::<u64>::mul as fn(u64, u64) -> u64),
                    ],
                ) as u64
        })
        .sum::<u64>()
}

#[no_mangle]
pub fn p2(i: &str) -> impl Display {
    i.行()
        .map(|x| {
            let (a, b) = x.μ(':');
            let should = a.λ::<u64>();
            let nums = b.κ::<u64>().collect_vec();
            let mut nums = &*nums;
            let &first = nums.take_first().ψ();
            should
                * search(
                    nums,
                    should,
                    first,
                    &[
                        (std::ops::Add::<u64>::add),
                        (std::ops::Mul::<u64>::mul as fn(u64, u64) -> u64),
                        concat,
                    ],
                ) as u64
        })
        .sum::<u64>()
}

fn main() {
    // let mut s = String::new();
    // for i in 0..1280 {
    let i = include_str!("inp.txt");
    //     s.push_str(i);
    // }
    // std::fs::write("src/inp.txt", s);
    println!("{}", run(i));
    println!("{}", p2(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| run(i));
}
