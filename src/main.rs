#![allow(
    unexpected_cfgs,
    confusable_idents,
    uncommon_codepoints,
    non_upper_case_globals,
    internal_features,
    mixed_script_confusables,
    static_mut_refs,
    incomplete_features,
    unused_imports,
    unsafe_op_in_unsafe_fn,
    redundant_semicolons
)]
#![feature(
    type_alias_impl_trait,
    iter_from_coroutine,
    iterator_try_reduce,
    step_trait,
    cmp_minmax,
    custom_inner_attributes,
    pattern_types,
    pattern_type_macro,
    extend_one,
    impl_trait_in_bindings,
    coroutines,
    stmt_expr_attributes,
    pattern_type_range_trait,
    const_trait_impl,
    coroutine_trait,
    iter_partition_in_place,
    slice_swap_unchecked,
    generic_const_exprs,
    iter_array_chunks,
    slice_from_ptr_range,
    if_let_guard,
    once_cell_get_mut,
    iter_collect_into,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    try_blocks,
    portable_simd,
    test,
    slice_split_once,
    import_trait_associated_functions,
    core_intrinsics
)]
extern crate test;
pub mod util;

pub use atools::prelude::*;
use itertools::chain;
use lower::apply;
use memchr::memmem;
use regex::bytes::Regex;
use rustc_hash::FxBuildHasher;
use std::{
    cmp::{Reverse, minmax},
    hash::Hash,
    mem::take,
    ops::{Coroutine, Deref},
    pin::Pin,
    simd::prelude::*,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};
use swizzle::array;
pub use util::prelude::*;

use crate::util::{MapF, UnionFind};

#[unsafe(no_mangle)]
// #[implicit_fn::implicit_fn]
pub unsafe fn p1(x: &'static [u8; ISIZE]) -> impl Debug {
    let mut registers = [0i64; 26];
    let instrs = x.行().collect::<Vec<_>>();
    let mut p = 0;
    let mut snds = vec![];
    use crossbeam::channel::unbounded;
    loop {
        let Some((a, x)) = instrs.get(p).map(|x| x.μ(' ')) else {
            break;
        };
        let reg = (x[0].wrapping_sub(b'a')) as usize;
        let rval = if x[0] == b'1' { 1 } else { registers[reg] };

        let b = (x.len() > 1).then(|| {
            x[1..]
                .str()
                .trim_ascii()
                .parse::<i64>()
                .unwrap_or_else(|_| registers[(x[2] - b'a') as usize])
        });
        match a {
            b"snd" => snds.push(rval),
            b"jgz" if rval > 0 => {
                p = (p as i64 + b.unwrap()) as usize;
                continue;
            }
            b"set" => registers[reg] = b.unwrap(),

            b"mod" => registers[reg] %= b.unwrap(),

            b"mul" => registers[reg] *= b.unwrap(),
            b"add" => registers[reg] += b.unwrap(),

            b"rcv" if rval != 0 => panic!("{snds:?}"),

            _ => {}
        }
        p += 1;
    }
    registers[1]
}
use crossbeam::channel::{self, Receiver, Sender, unbounded};

pub fn p2(x: &'static [u8]) -> impl Debug {
    // thought about coroutines, but eh.
    let (tx, rx) = unbounded();
    let (tx2, rx2) = unbounded();
    let count = &*Box::leak(Box::new(AtomicUsize::new(0)));
    std::thread::spawn(move || prog(0, x, (tx2, rx), &AtomicUsize::new(0)));
    std::thread::spawn(move || prog(1, x, (tx, rx2), count));
    std::thread::sleep(Duration::from_millis(150));
    count.load(Ordering::Relaxed)
}

pub fn prog(
    id: i64,
    x: &'static [u8],
    (tx, rx): (Sender<i64>, Receiver<i64>),
    count: &AtomicUsize,
) {
    let mut registers = [0i64; 26];
    registers[(b'p' - b'a') as usize] = id;
    let instrs = x.行().collect::<Vec<_>>();
    let mut p = 0;
    loop {
        let (a, x) = instrs.get(p).map(|x| x.μ(' ')).unwrap();
        let reg = (x[0].wrapping_sub(b'a')) as usize;
        let rval = if x[0] == b'1' { 1 } else { registers[reg] };

        let b = (x.len() > 1).then(|| {
            x[1..]
                .str()
                .trim_ascii()
                .parse::<i64>()
                .unwrap_or_else(|_| registers[(x[2] - b'a') as usize])
        });
        match a {
            b"snd" => {
                count.fetch_add(1, Ordering::Relaxed);
                tx.send(rval).unwrap();
            }
            b"jgz" if rval > 0 => {
                p = (p as i64 + b.unwrap()) as usize;
                continue;
            }
            b"set" => registers[reg] = b.unwrap(),
            b"mod" => registers[reg] %= b.unwrap(),
            b"mul" => registers[reg] *= b.unwrap(),
            b"add" => registers[reg] += b.unwrap(),
            b"rcv" => registers[reg] = rx.recv().unwrap(),
            _ => {}
        }
        p += 1;
    }
}
const ISIZE: usize = include_bytes!("inp.txt").len();
fn main() {
    unsafe { println!("{:?}", p2(include_bytes!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_bytes!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
