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

use atools::Split;
// pub use atools::prelude::*;
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
fn p2(v: &[u8]) -> usize {
    let rows = memchr::memchr(b'\n', v).ψ();
    let cols = rows + 1;

    let mut degree = vec![0u8; rows * cols];
    let mut queue = VecDeque::with_capacity(2000);

    for i in memchr::memchr_iter(b'@', v) {
        let count = util::nb((i / cols, i % cols))
            .map(|(x, y)| (v.get(x * cols + y) == Some(&b'@')) as u8)
            .into_iter()
            .sum::<u8>()
            + 1;

        degree[i] = count as _;

        if count < 4 + 1 {
            queue.push_back(i as u16);
        }
    }

    let mut sum = 0;
    while let Some(i) = queue.pop_front() {
        let i = i as usize;
        if degree[i] == 0 {
            continue;
        }

        degree[i] = 0;
        sum += 1;

        util::nb((i / cols, i % cols))
            .into_iter()
            .map(|(x, y)| x * cols + y)
            .for_each(|i| {
                if let Some(d) = degree.get_mut(i)
                    && *d != 0
                {
                    *d -= 1;
                    if *d < 4 + 1 {
                        queue.push_back(i as u16);
                    }
                }
            });
    }
    sum
}

macro_rules! nbors {
    ($width: literal, $tail:literal, $fname:ident) => {
        #[unsafe(no_mangle)]
        fn $fname(x: &[u8]) -> u32 {
            unsafe { core::hint::assert_unchecked(x.len() == $width * ($width + 1)) };
            let winc = $width + 1;
            let mut sum = 0;
            let mut l_tl;
            let mut r_tl;
            let mut t_tl;

            let mut l_tc;
            let mut r_tc;
            let mut t_tc;

            let mut l_tr;
            let mut r_tr;
            let mut t_tr;

            let t_mask = std::simd::Mask::<i8, _>::from([
                true, true, true, true, true, true, true, true, true, true, false, false, false,
                false, false, false,
            ]);
            {
                // first line
                // overlaps with right by 1
                let left = {
                    let f = |i| u8x64::from_slice(&x[i..i + 64]) & Simd::splat(2);
                    let cl =
                        u8x64::from_slice(&x[..64]).shift_elements_right::<1>(2) & Simd::splat(2);
                    l_tl = cl;
                    l_tc = f(0);
                    let cr = f(1);
                    l_tr = cr;
                    let bl = f(winc - 1);
                    let bc = f(winc);
                    let br = f(winc + 1);

                    let b_ = cl + cr;
                    let c_ = bl + bc + br;
                    (b_ + c_).simd_ge(Simd::splat(4)) & l_tc.simd_eq(u8x64::splat(0))
                };
                // overlaps with left by 1
                // overlaps with tail by 1
                // covers the span of 63..127
                let right = {
                    let f = |i| {
                        let i = i + 63;
                        u8x64::from_slice(&x[i..i + 64]) & Simd::splat(2)
                    };

                    let cl = u8x64::from_slice(&x[63 - 1..63 - 1 + 64]) & Simd::splat(2); // f(-1)
                    r_tl = cl;
                    // r_tl = u8x64::from_slice(&x[63 - 1..63 - 1 + 64]) &Simd::splat(2);
                    r_tc = u8x64::from_slice(&x[63..63 + 64]) & Simd::splat(2);
                    let cr = f(1);
                    r_tr = cr;
                    let bl = f(winc - 1);
                    let bc = f(winc);
                    let br = f(winc + 1);

                    let b_ = cl + cr;
                    let c_ = bl + bc + br;

                    (b_ + c_).simd_ge(Simd::splat(4)) & r_tc.simd_eq(u8x64::splat(0))
                };
                // covers the span of 126..136
                let tail = {
                    // last 10 (or 14)
                    let f = |i| {
                        let i = i + 126;
                        u8x16::from_slice(&x[i..i + 16]) & Simd::splat(2)
                    };

                    let cl = u8x16::from_slice(&x[126 - 1..126 - 1 + 16]) & Simd::splat(2);
                    t_tl = cl;
                    t_tc = u8x16::load_or_default(&x[126..126 + $tail]) & Simd::splat(2);
                    let cr = f(1);
                    t_tr = cr;
                    let bl = f(winc - 1);
                    let bc = f(winc);
                    let br = f(winc + 1);

                    let b_ = cl + cr;
                    let c_ = bl + bc + br;
                    (b_ + c_).simd_ge(Simd::splat(4)) & t_tc.simd_eq(Simd::splat(0))
                };

                // for &el in left.to_array().iter().take(63) {
                //     if el == true {
                //         print!("x");
                //     } else {
                //         print!(" ");
                //     }
                // }

                // for el in right.to_array().into_iter().take(63) {
                //     if el == true {
                //         print!("x");
                //     } else {
                //         print!(" ");
                //     }
                // }
                // for el in tail.to_array().into_iter().take(10) {
                //     if el == true {
                //         print!("x");
                //     } else {
                //         print!(" ");
                //     }
                // }
                // println!();
                let l = (left.to_bitmask() << 1).count_ones();
                let r = (right.to_bitmask() << 1).count_ones();
                let t = ((tail & t_mask).to_bitmask()).count_ones();
                sum += l + r + t;
            }
            for y in 1..$width - 1 {
                // overlaps with right by 1
                let left = {
                    let f = |i| u8x64::from_slice(&x[i..i + 64]) & Simd::splat(2);
                    let cl = f(y * winc - 1);
                    // let rel =
                    // u8x64::from_slice(&x[y * winc..y * winc + 64]).simd_eq(u8x64::splat(b'@'));
                    let cr = f(y * winc + 1);
                    let bl = f((y + 1) * winc - 1);
                    let bc = f((y + 1) * winc);
                    let br = f((y + 1) * winc + 1);

                    let a_ = l_tl + l_tc + l_tr;
                    l_tc = u8x64::from_slice(&x[y * winc..y * winc + 64]) & Simd::splat(2);
                    (l_tl, l_tr) = (cl, cr);
                    let b_ = cl + cr;
                    let c_ = bl + bc + br;

                    (a_ + b_ + c_).simd_ge(Simd::splat(10)) & l_tc.simd_eq(u8x64::splat(0))
                };
                // overlaps with left by 1
                // overlaps with tail by 1
                // covers the span of 63..127
                let right = {
                    let f = |i| {
                        let i = i + 63;
                        u8x64::from_slice(&x[i..i + 64]) & Simd::splat(2)
                    };
                    let cl = f(y * winc - 1);
                    let cr = f(y * winc + 1);
                    let bl = f((y + 1) * winc - 1);
                    let bc = f((y + 1) * winc);
                    let br = f((y + 1) * winc + 1);

                    let a_ = r_tl + r_tc + r_tr;
                    (r_tl, r_tr) = (cl, cr);
                    r_tc =
                        u8x64::from_slice(&x[y * winc + 63..y * winc + 63 + 64]) & Simd::splat(2);

                    let b_ = cl + cr;
                    let c_ = bl + bc + br;
                    // it double
                    (a_ + b_ + c_).simd_ge(Simd::splat(10)) & r_tc.simd_eq(u8x64::splat(0))
                };
                // covers the span of 126..136
                let tail = {
                    // last 8
                    let f = |i| {
                        let i = i + 126;
                        u8x16::load_or_default(&x[i..i + $tail]) & Simd::splat(2)
                    };

                    let cl = f(y * winc - 1);
                    let cr = f(y * winc + 1);
                    let bl = f((y + 1) * winc - 1);
                    let bc = f((y + 1) * winc);
                    let br = f((y + 1) * winc + 1);

                    let a_ = t_tl + t_tc + t_tr;
                    (t_tl, t_tr) = (cl, cr);
                    t_tc = u8x16::load_or_default(&x[y * winc + 126..y * winc + 126 + $tail])
                        & Simd::splat(2);
                    let b_ = cl + cr;
                    let c_ = bl + bc + br;
                    (a_ + b_ + c_).simd_ge(Simd::splat(10)) & t_tc.simd_eq(Simd::splat(0))
                };
                let l = (left.to_bitmask() << 1).count_ones();
                let r = (right.to_bitmask() << 1).count_ones();
                let t = ((tail & t_mask).to_bitmask()).count_ones();
                sum += l + r + t;
            }
            {
                // last line
                let y = $width - 1;
                // overlaps with right by 1
                let left = {
                    let f = |i| u8x64::from_slice(&x[i..i + 64]) & Simd::splat(2);
                    let cl = f(y * winc - 1);
                    let rel =
                        u8x64::from_slice(&x[y * winc..y * winc + 64]).simd_eq(u8x64::splat(b'@'));
                    let cr = f(y * winc + 1);

                    let a_ = l_tl + l_tc + l_tr;
                    let b_ = cl + cr;

                    (a_ + b_).simd_ge(Simd::splat(4)) & rel
                };
                // overlaps with left by 1
                // overlaps with tail by 1
                // covers the span of 63..127
                let right = {
                    let f = |i| {
                        let i = i + 63;
                        u8x64::from_slice(&x[i..i + 64]) & Simd::splat(2)
                    };
                    let cl = f(y * winc - 1);
                    let rel = u8x64::from_slice(&x[y * winc + 63..y * winc + 63 + 64])
                        .simd_eq(u8x64::splat(b'@'));
                    let cr = f(y * winc + 1);

                    let a_ = r_tl + r_tc + r_tr;
                    let b_ = cl + cr;

                    (a_ + b_).simd_ge(Simd::splat(4)) & rel
                };
                // covers the span of 126..136
                let tail = {
                    // last 8
                    let f = |i| {
                        let i = i + 126;
                        u8x16::load_or_default(&x[i..i + $tail]) & Simd::splat(2)
                    };
                    let cl = f(y * winc - 1);
                    let rel = u8x16::load_or_default(&x[y * winc + 126..y * winc + 126 + $tail])
                        .simd_eq(u8x16::splat(b'@'));
                    let cr = f(y * winc + 1);

                    let a_ = t_tl + t_tc + t_tr;
                    let b_ = cl + cr;
                    (a_ + b_).simd_ge(Simd::splat(4)) & rel
                };
                let l = (left.to_bitmask() << 1).count_ones();
                let r = (right.to_bitmask() << 1).count_ones();
                let t = ((tail & t_mask).to_bitmask()).count_ones();
                sum += l + r + t;
            }
            sum
        }
    };
}
nbors!(136, 10, nbors136);
nbors!(140, 14, nbors140);

#[unsafe(no_mangle)]
#[implicit_fn::implicit_fn]
pub unsafe fn p1(x: &'static [u8]) -> impl Debug {
    let grid = x.行().collect::<Vec<_>>();
    let mut tot = 0;
    // want nbors of each cell
    // [0,  @ , 0]
    // [0, [@], @]
    // [@, 0  , 0]
    // hmm

    // get nbor left and right
    // above + above >> 1 + above << 2
    let mut to = vec![vec![b'.'; 136]; 136];
    for roll in grid.clone().as_slice().find_iter(b'@') {
        let nb = util::nb(roll)
            .map(|(x, y)| {
                grid.get(y)
                    .and_then(|y| y.get(x))
                    .is_some_and(|x| *x == b'@') as u8
            })
            .into_iter()
            .sum::<u8>();
        if nb < 4 {
            tot += 1;
            // to[roll.1][roll.0] = b'x';
        } else {
            // to[roll.1][roll.0] = b'@';
        }
    }
    // for l in to {
    //     println!("{}", l.str());
    // }
    tot
}

const ISIZE: usize = include_bytes!("inp.txt").len();
fn main() {
    use atools::prelude::*;

    unsafe {
        println!(
            "{:?}",
            p1(include_bytes!(
                "/home/os/aoc_inputs_2025_4_53616c7465645f5f5021a2c6f2f1477957cf648aa98f5c84aded26838a2ede90785e4813a18ec8ac8a3d220956ea04b6b445865b32aee7b2e54bb761dc1fa967"
            ))
        )
    };

    // unsafe { println!("{:?}", p1(include_bytes!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_bytes!("inp.txt"));
    b.iter(|| unsafe { nbors136(i) });
}
