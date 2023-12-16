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

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum D {
    N,
    E,
    S,
    W,
}

macro_rules! csub {
    ($a:ident -= $b:expr) => {
        // faster than a overflowing sub :/
        $a = match $a.checked_sub($b) {
            Some(x) => x,
            None => break,
        }
    };
}

const SZ: u8 = 110;

fn test(mat: &[[u8; SZ as usize + 1]; SZ as usize], p: (u8, u8), d: D) -> u16 {
    use D::*;
    let mut e = vec![0u128; SZ.nat()];
    // *three dimensions* omg
    let mut been = vec![[0; 110]; 110];
    fn beam(
        mat: &[[u8; SZ as usize + 1]; SZ as usize],
        (mut x, mut y): (u8, u8),
        mut d: D,
        e: &mut [u128],
        been: &mut [[u8; 110]],
    ) {
        loop {
            if y >= SZ || x >= SZ {
                break;
            }
            bits!(e[y.nat()] + x);
            let w = (mat[y.nat()][x.nat()], d);
            mat! { w {
                (b'|', E | W) => {
                    if !bits!(been[x.nat()][y.nat()][d as u8]) {
                        bits!(been[x.nat()][y.nat()] + d as u8);
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
                    if !bits!(been[x.nat()][y.nat()][d as u8]) {
                        bits!(been[x.nat()][y.nat()] + d as u8);
                        if let Some(v) = x.checked_sub(1) {
                            beam(mat, (v, y), W, e, been);
                        }
                        d = E;
                        x += 1;
                    } else {
                        return;
                    }
                },
                (b'|' | b'.', N) => csub!(y -= 1),
                (b'-' | b'.', E) => x += 1,
                (b'|' | b'.', S) => y += 1,
                (b'-' | b'.', W) => csub!(x -= 1),
                (b'/', N) | (b'\\', S) => {
                    d = E;
                    x += 1;
                },
                (b'/', E) | (b'\\', W) => {
                    d = N;
                    csub!(y -= 1);
                },
                (b'/', S) | (b'\\', N) => {
                    d = W;
                    csub!(x -= 1);
                },
                (b'/', W) | (b'\\', E) => {
                    d = S;
                    y += 1;
                },
            }}
        }
    }
    beam(mat, p, d, &mut e, &mut been);
    e.iter().map(|x| x.count_ones() as u16).sum::<u16>()
}

use rayon::prelude::*;
pub fn p2(mat: &[[u8; 111]; 110]) -> impl Display {
    use D::*;
    let a = (0..SZ).into_par_iter().map(|i| test(mat, (i, 0), S)).max();
    let b = (0..SZ).into_par_iter().map(|i| test(mat, (i, SZ), N)).max();
    let c = (0..SZ).into_par_iter().map(|i| test(mat, (0, i), E)).max();
    let d = (0..SZ).into_par_iter().map(|i| test(mat, (SZ, i), W)).max();
    a.α().max(b.α()).max(c.α()).max(d.α())
}

pub fn p1(mat: &[[u8; 111]; 110]) -> impl Display {
    test(mat, (0, 0), D::E)
}

pub fn run(i: &str) -> impl Display {
    let mat = unsafe { &*(i.as_bytes().as_ptr() as *const [[u8; 111]; 110]) };
    p1(mat)
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
