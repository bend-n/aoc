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

#[inline]
fn do_(i: &str, search: fn(&[u64], u64) -> bool) -> u64 {
    let mut v = [0u64; 12];
    let mut i = i.as_bytes();
    let mut sum = 0;
    while !i.is_empty() {
        let should = reading::until(&mut i, b':');
        i.skip(1);
        let i = i.take_line().ψ();
        let read = reading::κ(i, &mut v);
        sum += should * search(C! { &v[..read] }, should) as u64;
    }
    sum
}

#[no_mangle]
pub fn run(i: &str) -> impl Display {
    // go backwards for extreme pruning ability
    fn search(nums: &[u64], tv: u64) -> bool {
        match nums {
            &[tail] => tv == tail,
            [head @ .., tail] => {
                let tail = *tail;
                unsafe { core::hint::assert_unchecked(tail != 0) };
                (tv % tail == 0 && search(head, tv / tail))
                    || (tv > tail && search(head, tv - tail))
            }
            [] => shucks!(),
        }
    }
    do_(i, search)
}

#[no_mangle]
pub fn p2(i: &str) -> impl Display {
    fn search(nums: &[u64], tv: u64) -> bool {
        match nums {
            &[tail] => tv == tail,
            [head @ .., tail] => {
                let &d = unsafe {
                    util::powers
                        .get_unchecked((((tail + 0xbf6) & (tail + 0x79c)) >> 10) as usize + 1)
                };
                let tail = *tail;
                (tv % tail == 0 && search(head, tv / tail))
                    || ((tv - tail) % d == 0 && search(head, tv / d))
                    || (tv > tail && search(head, tv - tail))
            }
            [] => shucks!(),
        }
    }
    do_(i, |n, should| search(n, should))
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
    assert_eq!(run(i).to_string(), 663613490587u64.to_string());
    assert_eq!(p2(i).to_string(), 110365987435001u64.to_string());
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| run(i));
}
