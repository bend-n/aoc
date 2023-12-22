#![allow(
    confusable_idents,
    uncommon_codepoints,
    mixed_script_confusables,
    incomplete_features
)]
#![feature(
    generic_const_exprs,
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

#[derive(Copy, Clone)]
struct Piece {
    a: [u16; 3],
    b: [u16; 3],
}
pub fn run(i: &str) -> impl Display {
    let mut pieces = vec![];
    let mut i = i.as_bytes();
    while i.len() > 0 {
        pieces.push(Piece {
            a: 読む::迄::<u16>(&mut i, b',')
                .and(読む::迄(&mut i, b','))
                .and(読む::迄(&mut i, b'~')),
            b: 読む::迄::<u16>(&mut i, b',')
                .and(読む::迄(&mut i, b','))
                .and(読む::迄または完了(&mut i, b'\n')),
        });
    }
    pieces.sort_unstable_by(|a, b| a.a[2].cmp(&b.a[2]));
    let mut m: HashMap<(u16, u16), u16> = HashMap::default();
    for p in pieces.iter_mut() {
        let a = (p.a[0]..=p.b[0]).flat_map(|x| (p.a[1]..=p.b[1]).map(move |y| (x, y)));
        let k = a.clone().map(|x| *m.get(&x).unwrap_or(&0)).max().unwrap() + 1;
        for e in a {
            m.insert(e, k + p.b[2] - p.a[2]);
        }
        *p = Piece {
            a: p.a.trunc().and(k),
            b: p.b.trunc().and(k + p.b[2] - p.a[2]),
        };
    }
    // (0..pieces.len())
    //     .filter_map(|i| {
    //         let mut m = HashMap::new();
    //         for (p, j) in pieces.iter().ι::<usize>() {
    //             if j == i {
    //                 continue;
    //             }
    //             let a = (p.a[0]..=p.b[0]).flat_map(|x| (p.a[1]..=p.b[1]).map(move |y| (x, y)));
    //             let k = a.clone().map(|x| *m.get(&x).unwrap_or(&0)).max().unwrap() + 1;
    //             for e in a {
    //                 m.insert(e, k + p.b[2] - p.a[2]);
    //             }
    //             if k < p.a[2] {
    //                 return None;
    //             }
    //         }
    //         return Some(1);
    //     })
    //     .sum::<u64>()
    (0..pieces.len())
        .map(|i| {
            let mut m = HashMap::new();
            let mut n = 0;
            for (p, j) in pieces.iter().ι::<usize>() {
                if j == i {
                    continue;
                }
                let a = (p.a[0]..=p.b[0]).flat_map(|x| (p.a[1]..=p.b[1]).map(move |y| (x, y)));
                let k = a.clone().map(|x| *m.get(&x).unwrap_or(&0)).max().unwrap() + 1;
                for e in a {
                    m.insert(e, k + p.b[2] - p.a[2]);
                }
                n += (k < p.a[2]) as u16;
            }
            n as u64
        })
        .sum::<u64>()
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
