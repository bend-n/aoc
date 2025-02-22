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
    let mut pos = (1, 1);
    let grid = b"123456789".map(|x| x - b'0').chunked::<3>();
    let mut n = 0;
    for &x in i.as_bytes() {
        if x == b'\n' {
            n = n * 10 + grid[pos.1][pos.0] as u32;
            continue;
        }
        pos = Dir::urdl(x).lim_add(pos, [0, 2], [0, 2]);
    }
    n
}

#[no_mangle]
pub unsafe fn p2(i: &str) -> impl Display {
    let mut pos = (1, 3);
    let mut chars = Vec::<u8>::new();
    #[rustfmt::skip]
    let grid = [
        [b' '; 7],
      *b"   1   ",
      *b"  234  ",
      *b" 56789 ",
      *b"  ABC  ",
      *b"   D   ",
        [b' '; 7],
    ];
    for &x in i.as_bytes() {
        if x == b'\n' {
            chars.push(grid[pos.1][pos.0]);
            continue;
        }
        let npos = Dir::urdl(x) + pos;
        if grid[npos.1][npos.0] != b' ' {
            pos = npos;
        }
    }
    chars.leak().str()
}

fn main() {
    unsafe { println!("{}", p2(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}

#[bench]
fn benc_sort(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p2(i) });
}
