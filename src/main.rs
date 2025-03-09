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
use memchr::memmem;
use regex::bytes::Regex;
use std::simd::prelude::*;
pub use util::prelude::*;

#[no_mangle]
pub fn p1(x: &'static str) -> impl Display {
    let mut i: impl Iterator<Item = &'static [u8]> = x.as_bytes().行();

    let mapping = i
        .by_ref()
        .take_while(|x| !x.is_empty())
        .map(|x| x.μ(' ').mr(|x| x.μ1(' ')))
        .collect::<Vec<_>>();

    let x = i.Δ();
    let n = mapping
        .iter()
        .flat_map(move |replacement| {
            memmem::find_iter(x, replacement.0).map(|n| {
                let mut x = x.to_vec();
                x.splice(n..n + replacement.0.len(), replacement.1.iter().copied());
                x
            })
        })
        .collect::<HashSet<_>>()
        .len();
    dbg!(n);
    let mut x = x.to_vec();
    let mut steps = 0;
    while x != b"e" {
        for replacement in &mapping {
            if let Some(n) = memmem::find(&x, replacement.1) {
                steps += 1;
                x.splice(n..n + replacement.1.len(), replacement.0.iter().copied());
            }
        }
    }
    steps
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
