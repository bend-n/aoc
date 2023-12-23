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

pub fn iterg<I: Iterator<Item = (u8, u8, u16)>>(
    start: (u8, u8, u16, HashSet<(u8, u8)>),
    graph: &mut impl Fn((u8, u8, u16)) -> I,
    end: &mut impl Fn((u8, u8)) -> bool,
    finally: &mut impl FnMut(u16),
    i: &[&[u8]],
) {
    if end((start.0, start.1)) {
        finally(start.2);
    } else {
        graph((start.0, start.1, start.2))
            .map(|(a, b, n)| {
                let mut m = start.3.clone();
                if m.insert((a, b)) {
                    iterg((a, b, n, m), graph, end, finally, i)
                } else {
                    // if n > 60 {
                    // for (line, y) in i.iter().ι::<u8>() {
                    // for (&elem, x) in line.iter().ι::<u8>() {
                    //     if m.contains(&(x, y)) {
                    //         print!("O");
                    //     } else {
                    //         print!("{}", elem as char);
                    //     }
                    // }
                    // println!();
                    // }
                    // }
                }
            })
            .Θ();
    };
}

pub fn run(i: &str) -> impl Display {
    let x = i.行().collect_vec();
    let i = x.as_slice();
    let end = (x.len() as u8 - 2, x.len() as u8 - 1);
    let mut sum = 0;
    iterg(
        (1u8, 0u8, 0u16, HashSet::from_iter([(1, 0)])),
        &mut |(x, y, n)| {
            let v = match i[y.nat()][x.nat()] {
                b'>' => vec![(x + 1, y, n + 1)],
                b'<' => vec![(x - 1, y, n + 1)],
                b'^' => vec![(x, y - 1, n + 1)],
                b'v' => vec![(x, y + 1, n + 1)],
                _ => [
                    Dir::N + (x, y),
                    Dir::E + (x, y),
                    Dir::S + (x, y),
                    Dir::W + (x, y),
                ]
                .into_iter()
                .flatten()
                .fl(lt(i.len() as u8))
                .fr(lt(i.len() as u8))
                .filter(|(x, y)| i[y.nat()][x.nat()] != b'#')
                .map(|(x, y)| (x, y, n + 1))
                .collect_vec(),
            };

            v.into_iter()
        },
        &mut |(x, y)| (x, y) == end,
        &mut |x| sum = sum.max(x),
        i,
    );
    sum
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
