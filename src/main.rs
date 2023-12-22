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
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((65u8, 65u8, 0u8));
    let mut even_corners = 0;
    let mut odd_corners = 0;
    let mut odd_all = 0;
    let mut even_all = 0;
    while let Some((x, y, g)) = q.pop_front() {
        match visited.insert((x, y)) {
            false => continue,
            true => {
                if g % 2 == 0 {
                    if g > 65 {
                        even_corners += 1;
                    }
                    even_all += 1;
                } else {
                    if g > 65 {
                        odd_corners += 1;
                    }
                    odd_all += 1;
                }
            }
        };

        for (x, y, d) in [
            Dir::N + (x, y),
            Dir::E + (x, y),
            Dir::S + (x, y),
            Dir::W + (x, y),
        ]
        .into_iter()
        .flatten()
        .fl(lt(131))
        .fr(lt(131))
        .filter(|&(x, y)| C! { i[y as usize * 132 + x as usize] } != b'#')
        .map(move |(x, y)| (x as u8, y as u8, g + 1))
        {
            q.push_back((x, y, d));
        }
    }

    let n = 202300;

    let even = n * n;
    let odd = (n + 1) * (n + 1);

    odd * odd_all + even * even_all - ((n + 1) * odd_corners) + (n * even_corners)
}

pub fn p1(i: &str) -> usize {
    let i = i.as_bytes();
    let mut state = vec![false; i.len() * 2];
    let mut q = VecDeque::with_capacity(4096);
    q.push_back((65u8, 65u8));
    for step in 1..65 {
        let is_odd = (step - 1) % 2;
        for _ in 0..q.len() {
            let (x, y) = q.pop_front().unwrap();
            for (x, y) in [
                Dir::N + (x, y),
                Dir::E + (x, y),
                Dir::S + (x, y),
                Dir::W + (x, y),
            ]
            .into_iter()
            .flatten()
            .fl(lt(131))
            .fr(lt(131))
            .filter(|&(x, y)| i[x.nat() * 132 + y.nat()] != b'#')
            {
                let cache_key = is_odd * i.len() + x.nat() * 132 + y.nat();
                if !state[cache_key] {
                    state[cache_key] = true;
                    q.push_back((x, y));
                }
            }
        }
    }
    state[i.len()..].iter().copied().filter(|&x| x).count()
}

pub fn run(i: &str) -> impl Display {
    p1(i)
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
