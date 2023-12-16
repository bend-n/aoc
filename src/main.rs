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
use arrayvec::ArrayVec;
pub use util::prelude::*;

pub fn hash16(s: &[u8]) -> u16 {
    s.as_ref()
        .iter()
        .fold(0u16, |acc, &x| acc.wrapping_add(x.widen()).wrapping_mul(17))
}

pub fn hash(s: impl AsRef<[u8]>) -> u8 {
    s.as_ref()
        .iter()
        .fold(0u8, |acc, &x| acc.wrapping_add(x).wrapping_mul(17))
}

pub fn p2(i: &str) -> u32 {
    // can be 5
    let mut 品 = [const { ArrayVec::<_, 6>::new_const() }; 256];
    for i in i
        .as_bytes()
        .split(|&b| b == b',')
        .take(4000)
        .inspect(|x| shucks!(if x.len() > 8))
    {
        match i
            .split_once(|&b| b == b'=')
            .map(|x| x.mr(|x| C! { x[0] } - b'0'))
        {
            None => {
                let ι = &i[..i.len() - 1];
                let h = hash16(ι);
                let lh = h + C! { ι[0] }.widen();
                let β = &mut 品[(h as u8).nat()];
                β.retain(|(α, _)| *α != lh);
            }
            Some((ι, κ)) => {
                let h = hash16(ι);
                let lh = h + C! { ι[0] }.widen();
                let bx = &mut 品[(h as u8).nat()];
                if let Some((_, σ)) = bx.iter_mut().find(|(α, _)| *α == lh) {
                    *σ = κ;
                } else {
                    unsafe { bx.push_unchecked((lh, κ)) };
                }
            }
        }
    }
    品.into_iter()
        .ι1::<u32>()
        .map(|(bx, i)| {
            bx.iter()
                .map(|(_, x)| *x)
                .ι1::<u32>()
                .map(|(x, j)| x as u32 * j)
                .sum::<u32>()
                * i
        })
        .sum::<u32>()
}

#[no_mangle]
pub fn p1(i: &str) -> impl Display {
    i.as_bytes()
        .split(|&x| x == b',')
        .take(4000)
        .inspect(|x| shucks!(if x.len() > 8))
        .map(|x| hash(x) as u32)
        .sum::<u32>()
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
