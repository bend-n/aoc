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

const WIDTH: usize = 71;
pub unsafe fn run(x: &str) -> impl Display {
    let mut i = x.as_bytes();
    let towels = i
        .take_line()
        .ψ()
        .to_vec()
        .leak()
        .split(|x| *x == b',')
        .map(|x| x.trim_ascii())
        .collect_vec()
        .leak();
    i.take_line();

    i.行()
        .map(|x| {
            fn ck(x: &'static [u8], towels: &'static [&[u8]]) -> usize {
                util::memoize! {
                    |(x, towels) in (&[u8], &[&[u8]])| -> usize {
                        if let [] = x {
                            return 1;
                        }
                        towels
                            .iter()
                            .filter(|&&y| x.starts_with(y))
                            .map(|y| ck(&x[y.len()..], towels))
                            .sum::<usize>()
                    };
                    (x, towels)
                }
            }

            ck(x.to_vec().leak(), towels)
        })
        .sum::<usize>()
}

fn main() {
    let s = include_str!("inp.txt");
    println!("{}", unsafe { run(s) });
    // dbg!(exec(&program, regi));
}
#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { run(i) });
}
