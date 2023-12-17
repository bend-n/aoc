#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(
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

pub fn run(i: &str) -> impl Display {
    let v = i.行().collect_vec();
    let g = util::LMap::new(|(x, y, δx, δy, gone): (i16, i16, i16, i16, i16)| {
        Some(
            [
                Dir::W + (x, y),
                Dir::E + (x, y),
                Dir::N + (x, y),
                Dir::S + (x, y),
            ]
            .iter()
            .filter_map(|&(nx, ny)| {
                let ngon;
                let nδx = nx - x;
                let nδy = ny - y;
                if nδx == δx && nδy == δy {
                    if gone == 3 {
                        return None;
                    }
                    ngon = gone + 1;
                } else if (nδx == -δx && nδx != 0) || (nδy == -δy && nδy != 0) {
                    return None;
                } else {
                    ngon = 1;
                }
                (nx >= 0 && nx < v[0].len() as i16 && ny >= 0 && ny < v.len() as i16).then(|| {
                    (
                        (nx, ny, nδx, nδy, ngon),
                        (v[ny as usize][nx as usize] - b'0') as u32,
                    )
                })
            })
            .collect_vec(),
        )
    });
    util::dijkstra(g, (0i16, 0i16, 0i16, 0i16, 0i16), |(x, y, _, _, _)| {
        x == v[0].len() as i16 - 1 && y == v.len() as i16 - 1
    })
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
