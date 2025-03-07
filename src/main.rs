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
    iter_partition_in_place,
    iter_chain,
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
#[lower::apply(wrapping)]
pub fn p1(x: &str) -> impl Display {
    let mut relations = HashMap::<&[u8], HashMap<_, _>>::default();
    x.行()
        .map(|x| {
            let [who, _, gain, quant, _, _, _, _, _, _, to] = x.μₙ(b' ').carr();
            let quant = if gain == b"gain" {
                quant.λ::<i64>()
            } else {
                -quant.λ::<i64>()
            };
            relations
                .entry(who)
                .or_default()
                .insert(&to[..to.len() - 1], quant);
        })
        .Θ();

    for person in relations.keys().copied().collect::<Vec<_>>() {
        relations.entry(b"me").or_default().insert(person, 0);
        relations.entry(person).or_default().insert(b"me", 0);
    }

    let people = Vec::from_iter(relations.keys().copied());
    people
        .iter()
        .copied()
        .permutations(people.len())
        .map(|arrangement| {
            arrangement
                .iter()
                .ι::<usize>()
                .flat_map(|(me, i)| {
                    let left = arrangement[(i - 1) % arrangement.len()];
                    let right = arrangement[(i + 1) % arrangement.len()];
                    [relations[me][left], relations[me][right]]
                })
                .sum::<i64>()
        })
        .max()
        .unwrap()
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
