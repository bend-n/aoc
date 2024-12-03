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
use logos::{Lexer as RealLexer, Logos, SpannedIter};
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[\n\s]+")]
#[allow(dead_code)]
pub enum P2 {
    #[token("do()")]
    Do,
    #[token("don't()")]
    Dont,
    #[regex(r"mul\([0-9]+,[0-9]+\)", |lex| lex.slice().μ(',').array().map(|x| x.as_bytes().iter().filter(|x| x.is_ascii_digit()).fold(0, |acc, x| acc * 10 + (x -b'0') as u32)))]
    Mul([u32; 2]),
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[\n\s]+")]
#[allow(dead_code)]
pub enum P1 {
    #[regex(r"mul\([0-9]+,[0-9]+\)", |lex| lex.slice().μ(',').array().map(|x| x.as_bytes().iter().filter(|x| x.is_ascii_digit()).fold(0, |acc, x| acc * 10 + (x -b'0') as u32)))]
    Mul([u32; 2]),
}

pub use util::prelude::*;

pub fn p1(i: &str) -> impl Display {
    P1::lexer(i)
        .filter_map(Result::ok)
        .map(|P1::Mul(a)| a.product())
        .sum::<u32>()
    // let re = regex::Regex::new("mul\\(([0-9]+),([0-9]+)\\)").unwrap();
    // re.captures_iter(i)
    //     .map(|find| {
    //         reading::all::<u32>(find.get(1).ψ().as_str().as_bytes())
    //             * reading::all::<u32>(find.get(2).ψ().as_str().as_bytes())
    //     })
    //     .sum::<u32>()

    // i.行()
    //     .filter(|x| {
    //         // let x = x.κ::<u64>().collect_vec();
    //     })
    //     .count()
}

pub fn run(i: &str) -> impl Display {
    let mut state = true;
    P2::lexer(i)
        .filter_map(Result::ok)
        .map(|x| {
            match x {
                P2::Mul([a, b]) => return a * b * state as u32,
                P2::Do => state = true,
                P2::Dont => state = false,
            }
            0
        })
        .sum::<u32>()
    // let re = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)|don't\(\)|do\(\)").unwrap();
    // let mut on = true;
    // re.captures_iter(i)
    //     .map(|find| unsafe {
    //         match find.get(0).unwrap_unchecked().as_str() {
    //             "don't()" => on = false,
    //             "do()" => on = true,
    //             _ if on => {
    //                 return reading::all::<u32>(find.get(1).ψ().as_str().as_bytes())
    //                     * reading::all::<u32>(find.get(2).ψ().as_str().as_bytes())
    //             }
    //             _ => (),
    //         };
    //         0
    //     })
    //     .sum::<u32>()
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
