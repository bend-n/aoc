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
use atools::CollectArray;
use iter::once;
use util::memoize;
pub use util::prelude::*;

#[rustfmt::skip]
fn numpad(x: u8) -> (usize, usize) {
    match x {
        b'7' => (0, 0), b'8' => (1, 0), b'9' => (2, 0),
        b'4' => (0, 1), b'5' => (1, 1), b'6' => (2, 1),
        b'1' => (0, 2), b'2' => (1, 2), b'3' => (2, 2),
        b'0'                 => (1, 3), b'A' => (2, 3),
        _ => unreachable!(),
    }
}
#[rustfmt::skip]
fn dpad(x: u8) -> (usize, usize) {
    match x {
                        b'^' => (1, 0), b'A' => (2, 0),
        b'<' => (0, 1), b'v' => (1, 1), b'>' => (2, 1),
        _ => unreachable!(),
    }
}
fn pathfind<const N: usize, const M: usize>(
    input: impl IntoIterator<Item = (u8, (usize, usize))>,
    grid: [[u8; N]; M],
) -> Vec<Vec<u8>> {
    let mut ways = HashSet::<Vec<u8>>::default();
    for ((code, start), first) in input
        .into_iter()
        .zip(std::iter::once(true).chain(std::iter::repeat(false)))
    {
        let current = pathfinding::directed::astar::astar_bag(
            &(start, code, b'.'),
            |&(p, at, _)| {
                [
                    (Dir::N + p, b'^'),
                    (Dir::E + p, b'>'),
                    (Dir::S + p, b'v'),
                    (Dir::W + p, b'<'),
                ]
                .into_iter()
                .filter(|&((x, y), _)| matches!(grid.get(y).map(|y| y.get(x)), Some(Some(_))))
                .filter(|&((x, y), _)| grid[y][x] != 0)
                .map(move |(p, dir)| ((p, at, dir), 1))
            },
            |_| 0,
            |&((x, y), at, _)| grid.get(y).map(|y| y.get(x)) == Some(Some(&at)),
        )
        .ψ()
        .0
        .into_iter()
        .map(|x| {
            x.into_iter()
                .skip(1)
                .map(|x| x.2)
                .chain(once(b'A'))
                .collect_vec()
        })
        .collect_vec();

        if first {
            ways.extend(current);
        } else {
            ways = ways
                .into_iter()
                .flat_map(|x| {
                    current.iter().map(move |way| {
                        let mut x = x.clone();
                        x.extend(way);
                        x
                    })
                })
                .collect();
        }
    }

    ways.into_iter().collect_vec()
}

fn num(code: [u8; 4]) -> Vec<Vec<u8>> {
    #[rustfmt::skip]
    let grid = [
        [b'7',b'8',b'9'],
        [b'4',b'5',b'6'],
        [b'1',b'2',b'3'],
        [0 ,  b'0',b'A'],
    ];
    let starts = once(numpad(b'A')).chain(code.iter().copied().map(numpad));
    pathfind(code.iter().copied().zip(starts), grid)
}
fn pad(code: Vec<u8>) -> Vec<Vec<u8>> {
    #[rustfmt::skip]
    let grid = [
        [0,   b'^',b'A'],
        [b'<',b'v',b'>'],
    ];
    let starts = once(dpad(b'A')).chain(code.iter().copied().map(dpad));
    pathfind(code.iter().copied().zip(starts), grid)
}

const MAXDEPTH: u8 = 2;
fn solve(thing: Vec<u8>, deep: u8) -> usize {
    if deep == MAXDEPTH {
        return thing.len();
    }
    memoize!(|(thing, deep) in (Vec<u8>, u8)| -> usize {
        thing[..thing.len() - 1]
            .split(|x| *x == b'A')
            .into_iter()
            .map(|x| {
                let mut x = x.to_vec();
                x.push(b'A');
                pad(x.to_vec())
                    .into_iter()
                    .map(|x| solve(x, deep + 1))
                    .min()
                    .unwrap()
            })
            .sum()
    }; (thing, deep))
}

static P2: [u64; 3750202] = {
    let mut l = [0; 3750202];
    include!("../lut");
    l
};
static P1: [u64; 3750202] = {
    let mut l = [0; 3750202];
    include!("../lut2");
    l
};
#[no_mangle]
pub fn run(x: &str) -> impl Display {
    let i = x.as_bytes();
    let codes: &[[u8; 5]; 5] = unsafe { i.as_chunks_unchecked::<5>().try_into().ψ() };
    /*
    for code in 1..1000 {
        let code_ = format!("{code:03}A");
        let code__ = code_.as_bytes();
        let length = num(code_.as_bytes()[..4].try_into().unwrap())
            .into_iter()
            .map(|x| solve(x, 0))
            .min()
            .unwrap() as u64;
        print!(
            "l[{}]={};",
            u32::from_le_bytes(code__.try_into().unwrap()) & 16777215,
            length * code as u64
        );
    }
    return 0;
    */

    codes
        .into_iter()
        .map(|x| C! { P1[u32::from_le_bytes(x[..4].try_into().ψ()) as usize & 0xffffff] })
        .sum::<u64>()
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
