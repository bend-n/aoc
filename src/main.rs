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
pub fn p1(x: &str) -> impl Display {
    std::iter::successors(Some(b"cqjxxyzz".map(|x| x - b'a')), |&(mut x)| {
        for e in x.iter_mut().rev() {
            *e += 1;
            if *e != 26 {
                break;
            }
            *e %= 26
        }
        Some(x)
    })
    .skip(1)
    .filter(|x| x.array_windows().any(|&[a, b, c]| a + 1 == b && b + 1 == c))
    .filter(|x| !(*b"iol").into_iter().any(|y| x.contains(&(y - b'a'))))
    .filter(|x| {
        let mut w = x.array_windows::<2>();
        let mut good = false;
        while let Some(&[a, b]) = w.next() {
            if a == b {
                if good {
                    return true;
                }
                good = true;
                w.next();
            }
        }
        false
    })
    .next()
    .unwrap()
    .add(b'a')
    .str()
    .to_owned()
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
