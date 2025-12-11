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
    const_index,
    maybe_uninit_slice,
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
    collections::HashMap,
    hash::Hash,
    hint::{assert_unchecked, unreachable_unchecked},
    mem::{MaybeUninit, take, zeroed},
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
    let mut g = HashMap::<_, Vec<_>>::new();
    x.行()
        .map(|x| {
            let (start, dest) = x.μ(':');
            for lem in dest.str().split_ascii_whitespace() {
                g.entry(start).or_default().push(lem.as_bytes());
            }
        })
        .θ();
    let mut n = 0;
    util::countg(
        b"you".as_slice(),
        &mut |x| g[x].clone().into_iter(),
        &mut |x| x == b"out",
        &mut n,
        &mut HashSet::default(),
    );
    n
}
pub(crate) const ISIZE: usize = include_bytes!("inp.txt").len();
pub(crate) fn main() {
    // unsafe { MASKS = (rahh()) };
    // unsafe { println!("{MASKS:?}") };
    // dbg!(sq([75262, 24842, 97390], [69492, 23068, 98918]));
    use atools::prelude::*;
    unsafe { println!("{:?}", p1(include_bytes!("inp.txt"))) }; // 1550760868
    //
    // unsafe { println!("{:?}", p1(include_bytes!("../1"))) };
    // unsafe { println!("{:?}", p1(include_bytes!("../2"))) };
    // unsafe { println!("{:?}", p1(include_bytes!("../3"))) };

    // unsafe { println!("{:?}", rah::run(include_bytes!("../1")) == 1644094530) }; // 1644094530
    // unsafe { println!("{:?}", rah::run(include_bytes!("../2")) == 1501292304) }; // 1501292304
    // unsafe { println!("{:?}", rah::run(include_bytes!("../3")) == 1429075575) }; // 1429075575
}

#[bench]
pub(crate) fn benc(b: &mut test::Bencher) {
    let i = boxd(include_bytes!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
