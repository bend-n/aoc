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
struct Setz {
    bits: [u128; 100],
}
impl Setz {
    fn new() -> Self {
        Self { bits: [0; 100] }
    }
    fn insert(&mut self, a: u8, b: u8) {
        unsafe { *self.bits.get_unchecked_mut(a as usize) |= 1 << b };
    }
    fn test(&self, a: u8, b: u8) -> bool {
        unsafe { (*self.bits.get_unchecked(a as usize) & 1 << b) != 0 }
    }
}

#[no_mangle]
pub fn run(i: &str) -> impl Display {
    let i = i.as_bytes();
    let c = unsafe { C! { &i[..1176 * 6]}.as_chunks_unchecked::<6>() };
    let mut bitsets = Setz::new();
    for i in 0..1176 {
        let [a, b, _, c, d, _] = C! { c[i] };
        bitsets.insert((a - b'0') * 10 + (b - b'0'), (c - b'0') * 10 + (d - b'0'));
    }
    let mut sum = 0;
    let mut v = Vec::with_capacity(25);
    'out: for mut pages in C!(&i[1176 * 6 + 1..]).行() {
        v.clear();
        let [a, b, rest @ ..] = pages else {
            unsafe { std::hint::unreachable_unchecked() }
        };
        pages = rest;
        v.push((a - b'0') * 10 + (b - b'0'));
        let mut i = 0;
        loop {
            mat!(pages {
                [b',', a, b, rest @ ..] => {
                    v.push((a - b'0') * 10 + (b - b'0'));
                    if let &[a, b] = &v[i..] {
                        // valid ones always have a rule
                        if !bitsets.test(a, b) {
                            continue 'out;
                        }
                        i += 1;
                    }
                    pages = rest;
                },
                [] => break,
            })
        }
        sum += C! { v[(v.len() - 1) / 2] } as u32;
    }
    leek!(v);
    sum
}

#[no_mangle]
pub fn p2(i: &str) -> impl Display {
    let i = i.as_bytes();
    let c = unsafe { C! { &i[..1176 * 6]}.as_chunks_unchecked::<6>() };
    let mut bitsets = Setz::new();
    for i in 0..1176 {
        let [a, b, _, c, d, _] = C! { c[i]};
        bitsets.insert((a - b'0') * 10 + (b - b'0'), (c - b'0') * 10 + (d - b'0'));
    }
    let mut sum = 0;
    let mut v = Vec::with_capacity(25);
    for mut pages in C!(&i[1176 * 6 + 1..]).行() {
        v.clear();
        let [a, b, rest @ ..] = pages else {
            unsafe { std::hint::unreachable_unchecked() }
        };
        pages = rest;
        v.push((a - b'0') * 10 + (b - b'0'));
        let mut i = 0;
        let mut faults = 0;
        loop {
            mat!(pages {
                [b',', a, b, rest @ ..] => {
                    v.push((a - b'0') * 10 + (b - b'0'));
                    if let &[a, b] = &v[i..] {
                        if !bitsets.test(a, b) {
                            faults += 1;
                        }
                        i += 1;
                    }
                    pages = rest;
                },
                [] => break,
            })
        }
        if faults == 0 {
            continue;
        }
        let mid = (v.len() - 1) / 2;
        let (_, &mut mid, _) = v.select_nth_unstable_by(mid, |&a, &b| {
            if bitsets.test(a, b) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });
        sum += mid as u32;
    }
    leek!(v);
    sum
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
