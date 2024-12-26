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
    iter_array_chunks,
    if_let_guard,
    const_mut_refs,
    get_many_mut,
    maybe_uninit_uninit_array,
    once_cell_get_mut,
    iter_collect_into,
    hint_assert_unchecked,
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
    core_intrinsics
)]
extern crate test;
pub mod util;
use std::simd::prelude::*;

pub use util::prelude::*;

#[no_mangle]
pub unsafe fn p1(x: &str) -> impl Display {
    let i = x.as_bytes().as_ptr();
    static mut keys: [u32; 256] = [u32::MAX; 256];
    let mut ki = 0;
    static mut locks: [u32; 250] = [u32::MAX; 250];
    let mut li = 0;
    for j in 0..500 {
        let acc = i
            .add(j * (7 * 6 + 1) + 3)
            .cast::<u8x32>()
            .read_unaligned()
            .simd_eq(Simd::splat(b'#'))
            .to_bitmask() as u32;
        if acc & 1 == 0 {
            C! { keys[ki] = acc };
            ki += 1;
        } else {
            C! { locks[li] = acc };
            li += 1;
        }
    }
    let mut sum = i32x8::splat(0);
    for &lock in &locks {
        for &k in keys.as_chunks_unchecked::<8>() {
            sum += (u32x8::splat(lock) & u32x8::from_array(k))
                .simd_eq(Simd::splat(0))
                .to_int();
        }
    }
    -sum.reduce_sum() as u32
}

fn main() {
    let s = include_str!("inp.txt");
    println!("{}", unsafe { p1(s) });

    // dbg!(exec(&program, regi));
}
#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
