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
#[no_mangle]
fn changes(mut x: u32) -> [(u8, i8); 2001] {
    let mut secret = x;
    x %= 10;
    std::array::from_fn(|_| {
        secret ^= (secret * 64) % 16777216;
        secret ^= (secret / 32) % 16777216;
        secret ^= (secret * 2048) % 16777216;
        let n = secret % 10;
        let v = (x as u8, (n as i64 - x as i64) as i8);
        x = n;
        v
    })
}

#[no_mangle]
pub fn run(x: &str) -> impl Display {
    let i = x.as_bytes();
    let mut map = HashMap::<[i8; 4], u16>::with_capacity_and_hasher(
        50000,
        rustc_hash::FxBuildHasher::default(),
    );
    let mut seen =
        HashSet::<[i8; 4]>::with_capacity_and_hasher(2000, rustc_hash::FxBuildHasher::default());
    i.行().map(reading::all::<u32>).map(changes).for_each(|x| {
        for &[elem @ .., (p, _)] in x.array_windows::<5>() {
            let elem = elem.map(|x| x.1);
            if seen.insert(elem) {
                *map.entry(elem).or_default() += p as u16;
            }
        }
        seen.clear();
    });
    map.into_iter().r().max().ψ()
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
