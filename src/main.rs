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
use lower::apply;
use memchr::memmem;
use regex::bytes::Regex;
use std::simd::prelude::*;
pub use util::prelude::*;

#[allow(warnings)]
type u32x3 = Simd<u32, 3>;

#[no_mangle]
pub fn p1(x: &'static str) -> impl Display {
    let boss = [104, 8, 1];
    const HP: usize = 0;
    const DAMAGE: usize = 1;
    const ARMOR: usize = 2;

    let weapons =
        [[8, 4, 0], [10, 5, 0], [25, 6, 0], [40, 7, 0], [74, 8, 0]].map(u32x3::from_array);
    let armors =
        [[13, 0, 1], [31, 0, 2], [53, 0, 3], [75, 0, 4], [102, 0, 5]].map(u32x3::from_array);
    let rings = [
        [25, 1, 0],
        [50, 2, 0],
        [100, 3, 0],
        [20, 0, 1],
        [40, 0, 2],
        [80, 0, 3],
    ]
    .map(u32x3::from_array);

    #[apply(saturating)]
    fn wins(mut boss: [u32; 3], mut player: [u32; 3]) -> bool {
        loop {
            boss[HP] = boss[HP] - (player[DAMAGE] - boss[ARMOR]);
            if boss[HP] == 0 {
                return true;
            }
            player[HP] = player[HP] - (boss[DAMAGE] - player[ARMOR]);
            if player[HP] == 0 {
                return false;
            }
        }
    }
    iproduct!(
        weapons,
        armors.into_iter().chain(once(Simd::splat(0))),
        chain(rings, twice(Simd::splat(0)))
            .into_iter()
            .combinations(2)
            .map(|rings| rings.into_iter().sum())
    )
    .map(|x| x.array().sum())
    .map(u32x3::to_array)
    .filter(|&[_, d, a]| wins(boss, [100, d, a]))
    .map(|[x, ..]| x)
    .min()
    .ψ()
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
