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
const SIZE: usize = 140;

fn explore(
    (x, y): (usize, usize),
    handled: &mut [[bool; 140]; 140],
    char: u8,
    get: &mut impl FnMut(usize, usize) -> Option<u8>,
    tot: &mut u32,
    count: &mut u32,
) {
    if get(x, y) == Some(char) && handled[y][x] == false {
        handled[y][x] = true;
        // αbβ
        // a.c
        // γdδ
        let α = get(x.wrapping_sub(1), y.wrapping_sub(1)) != Some(char);
        let β = get(x.wrapping_add(1), y.wrapping_sub(1)) != Some(char);
        let γ = get(x.wrapping_sub(1), y.wrapping_add(1)) != Some(char);
        let δ = get(x.wrapping_add(1), y.wrapping_add(1)) != Some(char);

        let a = get(x.wrapping_sub(1), y) != Some(char);
        let b = get(x, y.wrapping_sub(1)) != Some(char);
        let c = get(x.wrapping_add(1), y) != Some(char);
        let d = get(x, y.wrapping_add(1)) != Some(char);
        fn u(a: bool) -> u32 {
            a as u32
        }
        // *tot += u(a) + u(b) + u(c) + u(d);

        *tot += u(a & b) + u(b & c) + u(c & d) + u(a & d);
        *tot += u(!a & !b & α) + u(!b & !c & β) + u(!c & !d & δ) + u(!a & !d & γ);
        *count += 1;

        explore((x.wrapping_sub(1), y), handled, char, get, tot, count);
        explore((x + 1, y), handled, char, get, tot, count);
        explore((x, y + 1), handled, char, get, tot, count);
        explore((x, y.wrapping_sub(1)), handled, char, get, tot, count);
    }
}

#[no_mangle]
pub fn run(i: &str) -> impl Display {
    let grid = unsafe { i.as_bytes().as_chunks_unchecked::<{ SIZE + 1 }>() };
    let handled = &mut [[false; SIZE]; SIZE];
    let mut get = |x: usize, y: usize| {
        unsafe { core::hint::assert_unchecked(grid.len() == SIZE) };
        (x < SIZE && y < SIZE).then(|| grid[y][x])
    };
    (0..SIZE)
        .flat_map(move |y| (0..SIZE).map(move |x| (x, y)))
        .filter_map(|(x, y)| {
            let mut sides = 0;
            let mut area = 0;
            (!handled[y][x]).then(|| {
                let char = C! { grid[y][x]};
                explore((x, y), handled, char, &mut get, &mut sides, &mut area);
                area * sides
            })
        })
        .sum::<u32>()
}

fn main() {
    // (1..u32::MAX as u64).for_each(|a| assert_eq!(a.ilog10() + 1, digs(a)));
    // let mut s = String::new();
    // for i in 0..1280 {
    let i = include_str!("inp.txt");
    //     s.push_str(i);i
    // }
    // std::fs::write("src/inp.txt", s);
    #[allow(unused_unsafe)]
    println!("{}", unsafe { run(i) });
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { run(i) });
}
