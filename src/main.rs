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

pub fn run(i: &str) -> impl Display {
    use D::*;
    let mat = i.行().collect::<Box<_>>();
    let mut e = vec![vec![false; mat.len()]; mat.len()];
    let mut been = HashSet::new();
    fn beam(
        mat: &[&[u8]],
        (mut x, mut y): (usize, usize),
        mut d: D,
        e: &mut [Vec<bool>],
        been: &mut HashSet<(usize, usize, D)>,
    ) {
        println!("new beam!");
        loop {
            if y >= mat.len() || x >= mat.len() {
                break;
            }
            e[y][x] = true;
            println!("hello {} (going {d:?})", mat[y][x] as char);
            match (mat[y][x], d) {
                (b'|', E | W) => {
                    if been.insert((x, y, N)) {
                        println!("splitting |");
                        if let Some(v) = y.checked_sub(1)
                            && been.insert((x, v, N))
                        {
                            beam(mat, (x, v), N, e, been);
                        }
                        d = S;
                        y += 1;
                    } else {
                        println!("beam death: repetition");
                        return;
                    }
                }
                (b'-', N | S) => {
                    if been.insert((x, y, N)) {
                        if let Some(v) = x.checked_sub(1) {
                            println!("splitting -");
                            beam(mat, (v, y), W, e, been);
                        }
                        d = E;
                        x += 1;
                    } else {
                        println!("beam death: repetition");
                        return;
                    }
                }
                (b'|' | b'.', N) => sret!(y -= 1),
                (b'-' | b'.', E) => x += 1,
                (b'|' | b'.', S) => y += 1,
                (b'-' | b'.', W) => sret!(x -= 1),
                (b'/', N) | (b'\\', S) => {
                    d = E;
                    x += 1;
                }
                (b'/', E) | (b'\\', W) => {
                    d = N;
                    sret!(y -= 1);
                }
                (b'/', S) | (b'\\', N) => {
                    d = W;
                    sret!(x -= 1);
                }
                (b'/', W) | (b'\\', E) => {
                    d = S;
                    y += 1;
                }
                _ => unreachable!(),
            }
        }
        println!("beam death");
    }
    beam(&mat, (0, 0), E, &mut e, &mut been);
    e.iter()
        .map(|x| x.iter().filter(|x| **x).count())
        .sum::<usize>()
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
