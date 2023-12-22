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
    i.行()
        .map(|x| {
            let (a, b) = x
                .μ('~')
                .mb(|x| x.split(|&x| x == b',').map(|x| 読む::完了(x, 10)).Ν::<3>());
            pieces.push(Piece { a, b });
        })
        .Θ();

    pieces.sort_unstable_by(|a, b| a.a[2].cmp(&b.a[2]));
    let mut m: HashMap<(u16, u16), u16> = HashMap::new();
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
    let mut x = vec![0; pieces.len()];
    for (elem, i) in x.iter_mut().ι::<usize>() {
        let mut m: HashMap<(u16, u16), u16> = HashMap::new();
        let mut n = 0;
        for (p, j) in pieces.clone().iter_mut().ι::<usize>() {
            if j == i {
                continue;
            }
            let a = (p.a[0]..=p.b[0]).flat_map(|x| (p.a[1]..=p.b[1]).map(move |y| (x, y)));
            let k = a.clone().map(|x| *m.get(&x).unwrap_or(&0)).max().unwrap() + 1;
            for e in a {
                m.insert(e, k + p.b[2] - p.a[2]);
            }
            let z = p.a[2];
            *p = Piece {
                a: p.a.trunc().and(k),
                b: p.b.trunc().and(k + p.b[2] - p.a[2]),
            };

            n += (k < z) as u16;
        }
        *elem = n
    }
    x.into_iter().map(|x| x as u64).sum::<u64>()
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
