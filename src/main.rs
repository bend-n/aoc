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
    iter_from_coroutine,
    iterator_try_reduce,
    step_trait,
    cmp_minmax,
    custom_inner_attributes,
    extend_one,
    slice_as_array,
    impl_trait_in_bindings,
    coroutines,
    stmt_expr_attributes,
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
    vec_into_raw_parts,
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
use atools::{CollectArray, prelude::*};
use itertools::chain;
use lower::apply;
use md5::{Digest, Md5};
use memchr::memmem;
use regex::bytes::Regex;
use rustc_hash::FxBuildHasher;
use std::{
    cmp::{Reverse, minmax},
    mem::take,
    ops::Coroutine,
    pin::Pin,
    simd::prelude::*,
};
use swizzle::array;
pub use util::prelude::*;

#[allow(warnings)]
type u32x3 = Simd<u32, 3>;

fn run(code: &[Vec<&[u8]>], a: i32) -> impl Coroutine<Yield = i32, Return = ()> {
    let mut ptr = 0i32;
    #[coroutine]
    move || {
        let mut regis =
            HashMap::<&[u8], i32>::from_iter([(&b"a"[..], a), (b"b", 0), (b"c", 0), (b"d", 0)]);
        while let Some(i) = code.get(ptr as usize).cloned() {
            let p = |j: usize| i[j].str().parse::<i32>().unwrap_or_else(|_| regis[i[j]]);

            match i[0] {
                b"out" => drop(yield regis[i[1]]),
                b"cpy" => *regis.get_mut(i[2]).unwrap() = p(1),
                b"inc" => *regis.get_mut(i[1]).unwrap() += 1,
                b"dec" => *regis.get_mut(i[1]).unwrap() -= 1,
                b"jnz" if p(1) != 0 => {
                    ptr += p(2);

                    continue;
                }

                _ => {}
            }

            ptr += 1;
        }
    }
}

#[unsafe(no_mangle)]
#[implicit_fn::implicit_fn]
pub unsafe fn p1(x: &[u8; ISIZE]) -> impl Display {
    let x = x
        .行()
        .map(|x| x.μₙ(b' ').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for n in 0.. {
        dbg!(n);
        std::iter::from_coroutine(run(&x, n)).eq([0, 1].iter().copied().cycle());
    }

    0
}
const ISIZE: usize = include_bytes!("inp.txt").len();
fn main() {
    unsafe { println!("{}", p1(include_bytes!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_bytes!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
