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
pub use util::prelude::*;
const SIZE: usize = 5;
const H: usize = 7;

#[no_mangle]
pub unsafe fn p1(x: &str) -> impl Display {
    let mut i = x.as_bytes().as_ptr();
    let mut keys = [0u64; 250];
    let mut ki = 0;
    let mut locks = [0u64; 250];
    let mut li = 0;
    for _ in 0..500 {
        let mut acc = 0;
        for y in 0..H {
            for x in 0..SIZE {
                acc <<= 1;
                acc |= (i.add(y * 6 + x).read() == b'#') as u64
            }
        }
        i = i.add(6 * 7 + 1);
        if acc & 1 == 0 {
            C! { keys[ki] = acc };
            ki += 1;
        } else {
            C! { locks[li] = acc };
            li += 1;
        }
    }
    let mut sum = 0;
    for &k in &keys {
        for &lock in &locks {
            if k & lock == 0 {
                sum += 1;
            }
        }
    }
    sum
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
