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
    const_cmp,
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
    core_intrinsics,
    gen_blocks
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
    hint::{assert_unchecked, unreachable_unchecked},
    mem::take,
    ops::{Coroutine, Deref, RangeInclusive},
    pin::Pin,
    simd::prelude::*,
    sync::atomic::{AtomicUsize, Ordering},
    time::{Duration, Instant},
};
use swizzle::array;
pub use util::prelude::*;
mod rah;
use atools::prelude::*;

use crate::util::UnionFind;
#[unsafe(no_mangle)]
#[implicit_fn::implicit_fn]
pub unsafe fn p1(x: &'static [u8]) -> impl Debug {
    let p = util::uints::<i64>(x)
        .array_chunks::<2>()
        .collect::<Vec<_>>();
    let mut i = fimg::Image::<_, 1>::alloc(
        p.iter().map(|[x, _]| *x).max().unwrap() as u32 / 12,
        p.iter().map(|[_, x]| *x).max().unwrap() as u32 / 12,
    );
    i.points(
        &p.iter()
            .map(|[a, b]| (*a as i32 / 12, *b as i32 / 12))
            .collect::<Vec<_>>(),
        [255],
    );
    i.points(
        &[
            (98278 / 12, 50283 / 12),
            (98278 / 12, 4250 / 12),
            (63518 / 12, 4250 / 12),
            (63518 / 12, 50283 / 12),
            (98278 / 12, 50283 / 12),
        ],
        [129],
    );
    i.save("hm.png");
    i.show();
    return p
        .into_iter()
        .array_combinations()
        .map_w(|z @ [[x1, y1], [x2, y2]]| {
            // dbg!(z);
            ((x2 - x1 - 1).abs() * (y2 - y1 - 1).abs())
        })
        .max_by_key(|x| x.1)
        .unwrap();
}

pub(crate) fn sq(p1: [i64; 3], p2: [i64; 3]) -> i64 {
    (p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2) + (p1[2] - p2[2]).pow(2)
}

#[unsafe(no_mangle)]
pub(crate) unsafe fn p2(x: &[u8]) -> i64 {
    let mut p = x.as_ptr();
    let end = p.add(x.len());

    static mut points: [[i32; 1000]; 3] = [[0; 1000]; 3];
    let mut i = 0;
    while p != end {
        let mut f = |til| {
            let mut rest = 0;
            while let x = p.λ()
                && x != til
            {
                rest *= 10;
                rest += (x - b'0') as i32;
            }
            rest
        };
        let x = f(b',');
        let y = f(b',');
        let z = f(b'\n');
        (points[0][i], points[1][i], points[2][i]) = (x, y, z);
        i += 1;
    }
    (281..955)
        .map(|i| {
            // let former = from_fn(|j| points[j][i] as i64);
            // let (_dist, _retval) = (0..1000)
            //     .filter(|j| i != *j)
            //     .filter_map(|j| {
            //         let with: [i64; 3] = from_fn(|i| points[i][j] as i64);
            //         let d = sq(former, from_fn(|i| points[i][j] as i64)) as i64;
            //         // println!("{former:?} <-> {with:?}: {d}");
            //         Some((d, points[0][i] as i64 * points[0][j] as i64))
            //     })
            //     .min_by_key(|&(d, _)| d)
            //     .unwrap_or((i64::MAX, 0));
            // dbg!(_dist);

            let [_x, _y, _z] = from_fn(|j| points[j][i]);
            let x = i32x8::splat(_x);
            let y = i32x8::splat(_y);
            let z = i32x8::splat(_z);
            // result of min distance
            let mut dist = i64x8::splat(i64::MAX);
            // result of the multiply
            let mut retval = Simd::splat(0);
            let [(x_, xrest), (y_, yrest), (z_, zrest)] = {
                [
                    points[0][25..905].as_chunks::<8>(),
                    points[1][25..905].as_chunks::<8>(),
                    points[2][25..905].as_chunks::<8>(),
                ]
            };
            // let mut rmin = [i64::MAX; 16];
            // let mut reduced = i64::MAX;
            for (x_, y_, z_) in izip!(x_, y_, z_) {
                let x_ = Simd::from_slice(x_);
                let y_ = Simd::from_slice(y_);
                let z_ = Simd::from_slice(z_);
                let one = (x - x_).cast::<i64>();
                let two = (y - y_).cast::<i64>();
                let thr = (z - z_).cast::<i64>();
                let d = one * one + two * two + thr * thr;
                // let min = izip!(x_.to_array(), y_.to_array(), z_.to_array())
                //     .map(|(x, y, z)| {
                //         sq(
                //             [x as i64, y as i64, z as i64],
                //             [_x as i64, _y as i64, _z as i64],
                //         )
                //     })
                //     .collect_array::<16>()
                //     .unwrap();
                // rmin = min
                //     .zip(rmin)
                //     .map(|(x, y)| if x.min(y) == 0 { y } else { x.min(y) });

                // dbg!(d);
                let m = d.simd_lt(dist) & d.simd_ne(Simd::splat(0));
                // dbg!(d);
                // reduced = reduced.min(m.select(d, Simd::splat(i64::MAX)).reduce_min());
                // min_by_key ( distance )
                dist = m.select(d, dist);
                // println!("{rmin:?} {dist:?}");
                // assert_eq!(rmin, dist.to_array());
                // dbg!(dist);

                // points[0][1] * points[1][0]
                retval = m.select(x.cast::<i64>() * x_.cast::<i64>(), retval);
            }
            // dbg!(rmin, reduced);

            // let (dist_, retval_) = izip!(xrest, yrest, zrest)
            //     .filter_map(|t| {
            //         let d = sq(
            //             [_x as i64, _y as i64, _z as i64],
            //             <[_; 3]>::from(t).map(|x| *x as i64),
            //         );
            //         Some((d, _x as i64 * *t.0 as i64))
            //     })
            //     // .chain(izip!(dist.to_array(), retval.to_array()))
            //     .min_by_key(|&(d, _)| d)
            //     .unwrap();
            // dbg!(dist, retval, dist_, _retval);

            (dist, retval)
            // todo x8?
            // let dist = dist
            //     .to_array()
            //     .into_iter()
            //     .zip(retval.to_array())
            //     .min_by_key(|x| x.0)
            //     .unwrap();
            // let rest = retval;
            // let (dist, retval) = izip!(xrest, yrest, zrest)
            //     .filter_map(|t| {
            //         let d = sq(
            //             [_x as i64, _y as i64, _z as i64],
            //             <[_; 3]>::from(t).map(|x| *x as i64),
            //         ) as i64;
            //         Some((d, _x as i64 * *t.0 as i64))
            //     })
            //     .chain(izip!(dist.to_array(), retval.to_array()))
            //     .filter(|(d, _)| *d != 0)
            //     .min_by_key(|&(d, _)| d)
            //     .unwrap();
            // println!("{i}");
            // assert_eq!(dist, _dist);
            // assert_eq!(retval, _retval);
            // println!("pass");
            // (dist, retval)
        })
        .fold((-1, 0), |(_scdist, _scretval), (dist, retval)| {
            // we do a lil maxxing
            // eprintln!("dist = {dist:?}\nretval = {retval:?}\n_dist = {_dist:?}\n_retval = {_retval:?}");
            let (scdist, scretval) = dist
                .to_array()
                .into_iter()
                .zip(retval.to_array())
                .min_by_key(|x| x.0)
                .unwrap();
            // eprintln!("min of new {scdist} {scretval}");
            // let m = Simd::splat(dbg!(dist.reduce_min().max(scdist))).simd_ge(_dist);
            // eprintln!("{} >= {_dist:?} {m:?}", dist.reduce_min());
            // eprintln!("dmatched = {:?}",m.select(dist, Simd::splat(0)));
            // eprintln!("rmatched = {:?}",m.select(retval, Simd::splat(0)));
            // dbg!(real);

            if scdist > _scdist {
                (scdist, scretval)
            } else {
                (_scdist, _scretval)
            }
            // m.select(dist, _dist),
            // m.select(retval, _retval),

            // if dist > mxd {}
        })
        .1

    // .reduce_min()
}

pub(crate) const ISIZE: usize = include_bytes!("inp.txt").len();

pub(crate) fn main() {
    // dbg!(sq([75262, 24842, 97390], [69492, 23068, 98918]));
    use atools::prelude::*;
    unsafe { println!("{:?}", p1(include_bytes!("../input_day_9"))) };

    // unsafe { println!("{:?}", p2(include_bytes!("inp.txt"))) };
    // unsafe { println!("{:?}", p2(include_bytes!("../1"))) }; // 3200955921
    // unsafe { println!("{:?}", p2(include_bytes!("../2"))) }; // 8141888143
    // unsafe { println!("{:?}", p2(include_bytes!("../3"))) }; // 8465902405
}

#[bench]
pub(crate) fn benc(b: &mut test::Bencher) {
    let i = boxd(include_bytes!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
