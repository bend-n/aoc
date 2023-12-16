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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum D {
    N,
    E,
    S,
    W,
}

macro_rules! sret {
    ($a:ident -= $b:expr) => {
        $a = match $a.checked_sub($b) {
            Some(x) => x,
            None => break,
        }
    };
}

fn test(mat: &[&[u8]], p: (u8, u8), d: D) -> u16 {
    use D::*;
    let mut e = vec![0u128; mat.len()];
    let mut been = HashSet::new();
    fn beam(
        mat: &[&[u8]],
        (mut x, mut y): (u8, u8),
        mut d: D,
        e: &mut [u128],
        been: &mut HashSet<(u8, u8, D)>,
    ) {
        loop {
            if y >= mat.len() as u8 || x >= mat.len() as u8 {
                break;
            }
            bits!(e[y.nat()] + x);
            let w = (mat[y.nat()][x.nat()], d);
            mat! { w {
                (b'|', E | W) => {
                    if been.insert((x, y, N)) {
                        if let Some(v) = y.checked_sub(1) {
                            beam(mat, (x, v), N, e, been);
                        }
                        d = S;
                        y += 1;
                    } else {
                        return;
                    }
                },
                (b'-', N | S) => {
                    if been.insert((x, y, N)) {
                        if let Some(v) = x.checked_sub(1) {
                            beam(mat, (v, y), W, e, been);
                        }
                        d = E;
                        x += 1;
                    } else {
                        return;
                    }
                },
                (b'|' | b'.', N) => sret!(y -= 1),
                (b'-' | b'.', E) => x += 1,
                (b'|' | b'.', S) => y += 1,
                (b'-' | b'.', W) => sret!(x -= 1),
                (b'/', N) | (b'\\', S) => {
                    d = E;
                    x += 1;
                },
                (b'/', E) | (b'\\', W) => {
                    d = N;
                    sret!(y -= 1);
                },
                (b'/', S) | (b'\\', N) => {
                    d = W;
                    sret!(x -= 1);
                },
                (b'/', W) | (b'\\', E) => {
                    d = S;
                    y += 1;
                },
            }}
        }
    }
    beam(&mat, p, d, &mut e, &mut been);
    e.iter().map(|x| x.count_ones() as u16).sum::<u16>()
}

pub fn run(i: &str) -> impl Display {
    use D::*;
    let mat = i.行().collect::<Box<_>>();
    let mut max = 0;
    for i in 0..mat.len() as u8 {
        max = max.max(test(&mat, (i, 0), S));
        max = max.max(test(&mat, (i, mat.len() as u8 - 1), N));
    }
    for j in 0..mat.len() as u8 {
        max = max.max(test(&mat, (0, j), E));
        max = max.max(test(&mat, (mat.len() as u8 - 1, j), W));
    }
    max
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
