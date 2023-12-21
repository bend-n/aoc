#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(
    maybe_uninit_uninit_array,
    inline_const,
    slice_flatten,
    iter_collect_into,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    unchecked_math,
    array_windows,
    slice_take,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    byte_slice_trim_ascii
)]
extern crate test;
pub mod util;

pub use util::prelude::*;

pub fn p2(i: &str) -> usize {
    let i = i.as_bytes();
    let mut visited = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back((65i64, 65i64, 0u64));
    while let Some((x, y, g)) = q.pop_front() {
        match visited.entry((x, y)) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(x) => x.insert(g),
        };

        for (x, y, d) in [
            Dir::N + (x, y),
            Dir::E + (x, y),
            Dir::S + (x, y),
            Dir::W + (x, y),
        ]
        .into_iter()
        .filter(|&(x, _)| x < 131 && x >= 0)
        .filter(|&(_, y)| y < 131 && y >= 0)
        .filter(|&(x, y)| C! { i[y as usize * 132 + x as usize] } != b'#')
        .map(move |(x, y)| (x, y, g + 1))
        {
            q.push_back((x, y, d));
        }
    }
    let mut even_corners = 0;
    let mut odd_corners = 0;
    let mut odd_all = 0;
    let mut even_all = 0;
    for &v in visited.values() {
        if v % 2 == 0 {
            if v > 65 {
                even_corners += 1;
            }
            even_all += 1;
        } else {
            if v > 65 {
                odd_corners += 1;
            }
            odd_all += 1;
        }
    }

    let n = 202300;

    let even = n * n;
    let odd = (n + 1) * (n + 1);

    odd * odd_all + even * even_all - ((n + 1) * odd_corners) + (n * even_corners)
}

pub fn p1(i: &str) -> usize {
    let i = i.as_bytes();
    let mut n = 0;
    let (x, y) = (65u8, 65u8);
    pub fn countg<I: Iterator<Item = (u8, u8, u8)>>(
        start: (u8, u8, u8),
        graph: &mut impl Fn((u8, u8, u8)) -> I,
        sum: &mut usize,
        has: &mut HashSet<(u8, u8, u8)>,
    ) {
        if start.2 == 64 {
            *sum += 1;
        } else {
            graph(start)
                .map(|n| {
                    if has.insert(n) {
                        countg(n, graph, sum, has);
                    }
                })
                .Θ();
        }
    }
    countg(
        (x, y, 0),
        &mut |(x, y, n)| {
            [
                Dir::N + (x, y),
                Dir::E + (x, y),
                Dir::S + (x, y),
                Dir::W + (x, y),
            ]
            .into_iter()
            .flatten()
            .filter(|&(x, _)| x < i.len() as u8)
            .filter(|&(_, y)| y < i.len() as u8)
            .filter(|&(x, y)| C! { i[y.nat() * 132 + x.nat()] } != b'#')
            .map(move |(x, y)| (x, y, n + 1))
        },
        &mut n,
        &mut HashSet::new(),
    );
    n
}

pub fn run(i: &str) -> impl Display {
    p2(i)
}

fn main() {
    let i = include_str!("inp.txt").trim();
    println!("{}", run(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| run(i));
}
