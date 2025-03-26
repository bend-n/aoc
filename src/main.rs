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
    iterator_try_reduce,
    cmp_minmax,
    custom_inner_attributes,
    extend_one,
    slice_as_array,
    stdarch_x86_avx512,
    impl_trait_in_bindings,
    iter_partition_in_place,
    iter_chain,
    slice_swap_unchecked,
    generic_const_exprs,
    iter_array_chunks,
    slice_from_ptr_range,
    if_let_guard,
    once_cell_get_mut,
    iter_collect_into,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    vec_into_raw_parts,
    try_blocks,
    portable_simd,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    import_trait_associated_functions,
    core_intrinsics
)]
extern crate test;
pub mod util;

use atools::prelude::*;
use collar::CollectArray;
use lower::apply;
use memchr::memmem;
use regex::bytes::Regex;
use std::{
    cmp::{Reverse, minmax},
    mem::take,
    simd::prelude::*,
};
pub use util::prelude::*;

#[allow(warnings)]
type u32x3 = Simd<u32, 3>;

#[unsafe(no_mangle)]
#[implicit_fn::implicit_fn]
pub unsafe fn p1(x: &'static str) -> impl Display {
    let mut bots = [const { vec![] }; 500];
    let mut outputs = [!0; 100];
    x.行().filter(|x| x.starts_with(b"value")).for_each(|x| {
        let [_, v, _, _, _, i] = x.μₙ(b' ').collect_array();
        let [v, i] = [v, i].map(_.λ::<usize>());
        bots[i].push(v);
    });
    for x in x.行().filter(_.starts_with(b"bot")).cycle() {
        let [b"bot", i, _, _, b"to", lo, lo_i, _, _, b"to", hi, hi_i] = x.μₙ(b' ').collect_array()
        else {
            unreachable!()
        };
        let [bot_i, lo_i, hi_i] = [i, lo_i, hi_i].map(_.λ::<usize>());
        if let &[a, b] = &*bots[bot_i] {
            let [lo_x, hi_x] = minmax(a, b);
            match lo {
                b"bot" => bots[lo_i].push(lo_x),
                b"output" => outputs[lo_i] = lo_x,
                _ => unreachable!(),
            }
            match hi {
                b"bot" => bots[hi_i].push(hi_x),
                b"output" => outputs[hi_i] = hi_x,
                _ => unreachable!(),
            }
            if lo_x == 17 && hi_x == 61 {
                println!("{bot_i}");
                // return bot_i;
            }
            bots[bot_i].clear();
        }
        if let Ok(x) = outputs[..=2]
            .iter()
            .filter(**_ != !0)
            .collect_array_checked::<3>()
        {
            return x.map(*_).product();
        }
    }

    0
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
