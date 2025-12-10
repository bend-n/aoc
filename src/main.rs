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
    int_lowest_highest_one,
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
use z3::ast::Int;
mod rah;
use atools::prelude::*;

use crate::util::UnionFind;
#[unsafe(no_mangle)]
#[implicit_fn::implicit_fn]
pub unsafe fn p1(x: &'static [u8; ISIZE]) -> impl Debug {
    let x = x
        .行()
        .map(|x| {
            let (expected, buttons) = x.μ(' ');
            let mut c = vec![];
            let mut r = vec![];
            let (mut buttons, power) = buttons.rsplit_once(|x| *x == b' ').unwrap();

            while let Some(x) = buttons.get(0) {
                match x {
                    b'(' => {}
                    d @ b'0'..=b'9' => c.push(d - b'0'),
                    b',' => {}
                    b')' => {
                        r.push(c.clone());
                        c.clear();
                    }
                    b' ' => {}
                    x => panic!("{}", *x as char),
                }
                buttons = &buttons[1..];
            }
            let mut power = &power[1..];
            let mut pvec = vec![];
            let mut c = 0;
            while let Some(x) = power.get(0) {
                match x {
                    d @ b'0'..=b'9' => {
                        c *= 10;
                        c += (d - b'0') as i16
                    }
                    b',' => {
                        pvec.push(c);
                        c = 0;
                    }
                    b'}' => {
                        pvec.push(c);
                        break;
                    }
                    b' ' => {}
                    x => panic!("{}", *x as char),
                }
                power = &power[1..];
            }

            (
                expected[1..expected.len() - 1]
                    .iter()
                    .map(|x| *x == b'#')
                    .collect::<Vec<_>>(),
                r,
                pvec,
            )
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for (xpected, buttons, power) in x {
        let o = z3::Optimize::new();
        let p = (0..buttons.len())
            .map(|_| Int::fresh_const("p"))
            .inspect(|x| o.assert(&x.ge(0)))
            .collect::<Vec<_>>();
        for j in 0..power.len() {
            let sat = &buttons
                .iter()
                .enumerate()
                .filter(|(_, b)| b.contains(&(j as u8)))
                .map(|(x, _)| p[x].clone())
                .reduce(|a, b| a + b)
                .unwrap()
                .eq(power[j]);
            o.assert(sat);
        }
        let s = p.iter().cloned().reduce(|a, b| a + b).unwrap();
        o.minimize(&s);
        assert_eq!(o.check(&[]), z3::SatResult::Sat);
        let m = o.get_model().unwrap();
        sum += m.eval(&s, true).unwrap().as_i64().unwrap();

        // sum += util::dijkstra(
        //     (vec![false; xpected.len()], 0),
        //     |(x, p)| {
        //         buttons.iter().map(move |button| {
        //             let mut new_state = x.clone();
        //             for lem in button {
        //                 let p = &mut new_state[*lem as usize];
        //                 *p = !*p;
        //             }
        //             ((new_state, p + 1), p)
        //         })
        //     },
        //     |(x, _)| x == &xpected,
        // )
        // .unwrap()
        // .1
        // .1;
    }
    sum
}
pub(crate) const ISIZE: usize = include_bytes!("inp.txt").len();

pub(crate) fn main() {
    // dbg!(sq([75262, 24842, 97390], [69492, 23068, 98918]));
    use atools::prelude::*;
    unsafe { println!("{:?}", p1(include_bytes!("inp.txt"))) }; // 1550760868

    // unsafe { println!("{:?}", p2(include_bytes!("inp.txt"))) };
    // unsafe { println!("{:?}", rah::run(include_bytes!("../1")) == 1644094530) }; // 1644094530
    // unsafe { println!("{:?}", rah::run(include_bytes!("../2")) == 1501292304) }; // 1501292304
    // unsafe { println!("{:?}", rah::run(include_bytes!("../3")) == 1429075575) }; // 1429075575
}

#[bench]
pub(crate) fn benc(b: &mut test::Bencher) {
    let i = boxd(include_bytes!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
