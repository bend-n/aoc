#![allow(
    confusable_idents,
    uncommon_codepoints,
    non_upper_case_globals,
    internal_features,
    mixed_script_confusables,
    static_mut_refs,
    incomplete_features,
    redundant_semicolons
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
use lower::apply;
use memchr::memmem;
use regex::bytes::Regex;
use std::simd::prelude::*;
pub use util::prelude::*;

#[allow(warnings)]
type u32x3 = Simd<u32, 3>;

#[no_mangle]
pub fn p1(x: &'static str) -> impl Display {
    let mut registers = [1, 0];
    let instrs = x.行().collect::<Vec<_>>();
    let mut p = 0;
    loop {
        let Some((a, x)) = instrs.get(p).map(|x| x.μ(' ')) else {
            break;
        };
        match a {
            b"hlf" => registers[(x == b"b") as usize] /= 2,
            b"tpl" => registers[(x == b"b") as usize] *= 3,
            b"inc" => registers[(x == b"b") as usize] += 1,
            b"jmp" => {
                p = (p as i32 + x.λ::<i32>()) as usize;
                continue;
            }
            b"jie" => {
                let (reg, to) = x.μ(',');
                if registers[(reg == b"b") as usize] % 2 == 0 {
                    p = (p as i32 + to.trim_ascii().λ::<i32>()) as usize;
                    continue;
                }
            }
            b"jio" => {
                let (reg, to) = x.μ(',');
                if registers[(reg == b"b") as usize] == 1 {
                    p = (p as i32 + to.trim_ascii().λ::<i32>()) as usize;
                    continue;
                }
            }
            x => {
                unreachable!("{}", x.p());
            }
        }
        p += 1;
    }
    registers[1]
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
