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
    iter_repeat_n,
    stdarch_x86_avx512,
    iter_partition_in_place,
    slice_swap_unchecked,
    generic_const_exprs,
    ptr_sub_ptr,
    iter_array_chunks,
    slice_from_ptr_range,
    if_let_guard,
    maybe_uninit_uninit_array,
    once_cell_get_mut,
    iter_collect_into,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    vec_into_raw_parts,
    try_blocks,
    slice_take,
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
pub use util::prelude::*;
#[no_mangle]
pub unsafe fn p1(i: &str) -> impl Display {
    let mut m = [[0u32; 1000]; 1000];
    for l in i.行() {
        let (d, a, b) = if l.starts_with(b"toggle") {
            let [_, a, _, b] = l.str().split(' ').carr::<4>();
            (2, a, b)
        } else {
            let [_, x, a, _, b] = l.str().split(' ').carr::<5>();
            ((x == "on") as u32, a, b)
        };
        let [a, b] = [a, b].map(|x| x.μ(',').mb(|x| x.λ::<u32>()));
        for x in a.0..=b.0 {
            for y in a.1..=b.1 {
                let e = &mut m[x as usize][y as usize];
                match d {
                    0 => *e = e.saturating_sub(1),
                    1 => *e += 1,
                    _ => *e += 2,
                }
            }
        }
    }
    m.as_flattened().into_iter().sum::<u32>()
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
