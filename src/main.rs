#![allow(
    overflowing_literals,
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
    int_roundings,
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
    arch::x86_64::*,
    cmp::{Reverse, minmax},
    hash::Hash,
    hint::assert_unchecked,
    mem::take,
    ops::{Coroutine, Deref},
    pin::Pin,
    simd::prelude::*,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};
use swizzle::array;
pub use util::prelude::*;
#[unsafe(no_mangle)]
pub unsafe fn max(data: &[u8; 99]) -> u8 {
    let v0 = u8x64::from_slice(&data[0..64]);
    let v1 = u8x32::from_slice(&data[64..96]).resize::<64>(0);
    let mut mx = v0.simd_max(v1).reduce_max();
    for &b in &data[96..] {
        if b > mx {
            mx = b;
        }
    }
    mx
}

#[unsafe(no_mangle)]
pub unsafe fn max89(data: &[u8; 89]) -> u8 {
    let v0 = u8x64::from_slice(&data[0..64]);
    let v1 = u8x32::load_or_default(&data[64..]).resize::<64>(0);
    v0.simd_max(v1).reduce_max()
}

pub unsafe fn max2(data: &[u8]) -> u8 {
    if data.len() == 1 {
        return data[0];
    }
    let start = u8x64::load_or_default(data);
    if data.len() > 64 {
        let extra = u8x64::load_or_default(&data[64..]);
        start.simd_max(extra).reduce_max()
    } else {
        start.reduce_max()
    }
}
#[unsafe(no_mangle)]
#[inline(always)]
// https://stackoverflow.com/a/77709227
unsafe fn maxi32(v: u8x32) -> u8 {
    let v: u16x16 = transmute(v);
    //indexes
    const even: u16x16 = u16x16::from_array([
        0xff00, 0xff02, 0xff04, 0xff06, 0xff08, 0xff0a, 0xff0c, 0xff0e, 0xff10, 0xff12, 0xff14,
        0xff16, 0xff18, 0xff1a, 0xff1c, 0xff1e,
    ]);
    let odd = even + u16x16::splat(1);
    let vu16 = (v & u16x16::splat(0xff00u16) ^ odd).simd_min(v << 8 ^ even);
    _mm_cvtsi128_si32(_mm_minpos_epu16(transmute(
        vu16.extract::<0, 8>().simd_min(vu16.extract::<8, 8>()),
    ))) as u8
}
#[unsafe(no_mangle)]
unsafe fn maxi(data: &[u8]) -> (u8, u8) {
    if data.len() > 96 {
        let f1 = maxi32(u8x32::from_slice(&data[..32]));
        let f2 = 32 + maxi32(u8x32::from_slice(&data[32..64]));
        let f3 = 64 + maxi32(u8x32::from_slice(&data[64..96]));

        let Left(a, b) = Left(data.get(98).copied().unwrap_or(0), 98)
            .max(Left(data.get(97).copied().unwrap_or(0), 97))
            .max(Left(data.get(96).copied().unwrap_or(0), 96))
            .max(Left(*data.get_unchecked(f3 as usize), f3))
            .max(Left(*data.get_unchecked(f2 as usize), f2))
            .max(Left(*data.get_unchecked(f1 as usize), f1));
        (a, b)
    } else if data.len() > 64 {
        let start = u8x32::from_slice(&data[..32]);
        let extra = u8x32::from_slice(&data[32..64]);
        let f1 = maxi32(start);
        let f2 = 32 + maxi32(extra);
        let f3 = 64 + maxi32(u8x32::load_or_default(&data[64..]));
        let Left(a, b) = Left(*data.get_unchecked(f3 as usize), f3)
            .max(Left(*data.get_unchecked(f2 as usize), f2))
            .max(Left(*data.get_unchecked(f1 as usize), f1));
        (a, b)
    } else if data.len() > 32 {
        let start = u8x32::from_slice(&data[..32]);
        let extra = u8x32::load_or_default(&data[32..]);
        let f1 = maxi32(start);
        let f2 = 32 + maxi32(extra);
        let Left(a, b) = std::cmp::max(
            Left(*data.get_unchecked(f2 as usize), f2),
            Left(*data.get_unchecked(f1 as usize), f1),
        );
        (a, b)
    } else {
        let start = u8x32::load_or_default(&data);
        let x = maxi32(start);
        (*data.get_unchecked(x as usize), x)
    }
}
#[unsafe(no_mangle)]
#[implicit_fn::implicit_fn]
pub unsafe fn p1(x: &'static [u8; ISIZE]) -> impl Debug {
    core::hint::assert_unchecked(x.len() == 200 * 101);
    x.as_chunks_unchecked::<101>()
        .into_iter()
        .map(|l| {
            let l = l.get_unchecked(..100);
            let l_ = l.get_unchecked(..l.len() - 1);
            let el = max(unsafe { &*(l_.as_ptr() as *const [u8; 99]) });
            let i = memchr::memchr(el, l_).unwrap_unchecked();

            let n = (el - b'0') as u64;
            let l_ = l.get_unchecked(i as usize + 1..);
            let el = max2(l_);
            (n * 10) + (el - b'0') as u64
        })
        .sum::<u64>()
}

#[unsafe(no_mangle)]
pub unsafe fn p2(x: &'static [u8; ISIZE]) -> impl Debug {
    core::hint::assert_unchecked(x.len() == 200 * 101);
    x.as_chunks_unchecked::<101>()
        .into_iter()
        .map(|l| {
            let mut l = l.get_unchecked(..100);

            let l_ = l.get_unchecked(..l.len() - 11);
            let (el, i) = maxi(l_);
            let mut n = (el - b'0') as u64;
            l = l.get_unchecked(i as usize + 1..);

            for i in 1..11 {
                let l_ = l.get_unchecked(..l.len() + i - 11);
                let (el, i) = maxi(l_);
                n *= 10;
                n += (el - b'0') as u64;
                l = l.get_unchecked(i as usize + 1..);
            }

            let el = max2(l);
            n * 10 + (el - b'0') as u64
        })
        .sum::<u64>()
}
const ISIZE: usize = include_bytes!("inp.txt").len();
fn main() {
    unsafe { println!("{:?}", p2(include_bytes!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_bytes!("inp.txt"));
    b.iter(|| unsafe { p2(i) });
}
