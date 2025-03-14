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
use lower::apply;
use memchr::memmem;
use regex::bytes::Regex;
use std::{cmp::Reverse, simd::prelude::*};
pub use util::prelude::*;

#[allow(warnings)]
type u32x3 = Simd<u32, 3>;

#[unsafe(no_mangle)]
pub unsafe fn p1(x: &'static str) -> impl Display {
    let re = Regex::new(r"\[([^\]]+)\]").ψ();
    x.行()
        .map(|x| {
            [
                re.captures_iter(x)
                    .map(|x| x.get(1).ψ().as_bytes().to_vec())
                    .collect::<Vec<_>>(),
                re.replace_all(x, *b"_")
                    .μₙ(b'_')
                    .map(<[u8]>::to_vec)
                    .collect::<Vec<_>>(),
            ]
        })
        .filter(|[bracket, not]| {
            let has_abba = |x: &[Vec<u8>]| {
                x.iter().any(|x| {
                    x.array_windows::<4>()
                        .any(|&[a, b, c, d]| a != b && c == b && d == a)
                })
            };

            // !has_abba(&bracket) && has_abba(&not)
            not.iter().any(|not| {
                not.array_windows::<3>().any(|&[a, b, c]| {
                    ((a != b) & (a == c))
                        && bracket
                            .iter()
                            .flat_map(|x| x.array_windows::<3>())
                            .contains(&[b, a, b])
                })
            })
        })
        .count()
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
