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
    slice_take,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    core_intrinsics
)]
extern crate test;
pub mod util;
use atools::prelude::*;
pub use util::prelude::*;

fn manual_n<const N: usize>(n: [&u8; N]) -> u32 {
    n.iter()
        .map(|&&x| (x - b'0') as u32)
        .fold(0, |acc, x| acc * 10 + x)
}

pub fn p1(i: &str) -> impl Display {
    let mut i = i.as_bytes();
    let mut sum = 0;
    while let Some(idx) = memchr::memchr(b'm', i) {
        i = C! { &i[idx + 1..] };
        match i {
            [b'u', b'l', b'(', rest @ ..] => {
                macro_rules! cases {
                    ($([$($i:ident)+,$($t:ident)+])+) => {
                        match rest {
                            $(
                                [$($i @ b'0'..=b'9'),+, b',', $($t @ b'0'..=b'9'),+, b')', rest @ ..] => {
                                    (manual_n([$($i),+]) * manual_n([$($t),+]), rest)
                                }
                            )+
                            _ => (0, i),
                        }

                    };
                }
                let (res, rest) = cases!(
                    [a b c , d e f]
                    [a b c , d e]
                    [a b c , d]
                    [a b , d e f]
                    [a b , d e]
                    [a b , d]
                    [a , d e f]
                    [a , d e]
                    [a , d]
                );
                sum += res;
                i = rest;
            }
            _ => {}
        }
    }

    sum
}

pub fn run(i: &str) -> impl Display {
    let mut i = i.as_bytes();
    let mut sum = 0;
    let mut on = 1;
    while let Some(idx) = memchr::memchr2(b'm', b'd', i) {
        i = C! { &i[idx + 1..] };
        match i {
            [b'u', b'l', b'(', rest @ ..] => {
                macro_rules! cases {
                ($([$($i:ident)+,$($t:ident)+])+) => {
                    match rest {
                        $(
                            [$($i @ b'0'..=b'9'),+, b',', $($t @ b'0'..=b'9'),+, b')', rest @ ..] => {
                                (manual_n([$($i),+]) * manual_n([$($t),+]) * on, rest)
                            }
                        )+
                        _ => (0, i),
                    }

                };
            }
                let (res, rest) = cases!(
                    [a b c , d e f]
                    [a b c , d e]
                    [a b c , d]
                    [a b , d e f]
                    [a b , d e]
                    [a b , d]
                    [a , d e f]
                    [a , d e]
                    [a , d]
                );
                sum += res;
                i = rest;
            }
            _ => mat! { on {
                0 => match i {
                    [b'o', b'(', rest @ ..] => {
                        on = 1;
                        i = rest;
                    }
                    _ => {}
                },
                1 => match i {
                    [b'o', b'n', rest @ ..] => {
                        on = 0;
                        i =rest;
                    },
                    _ => {},
                },
            }},
        }
    }

    sum
}

fn main() {
    // let mut s = String::new();
    // for i in 0..1280 {
    let i = include_str!("inp.txt");
    //     s.push_str(i);
    // }
    // std::fs::write("src/inp.txt", s);
    println!("{}", p1(i));
    println!("{}", run(i));
    // println!("{}", p2(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| run(i));
}

#[bench]
fn bench2(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| run(i));
}
