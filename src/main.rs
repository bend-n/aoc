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
    let mut x = 0i32;
    let mut y = 0i32;
    // boundary points, shoelace
    let (b, a) = i.行().fold((0, 0), |(b, a), i| {
        let d = unsafe { std::mem::transmute::<u8, Dir>(i[0]) };
        let c = i.μ(' ').1.μ(' ').0.λ::<u8>();
        let (ox, oy) = (x, y);
        for _ in 0..c {
            (x, y) = match d {
                // y down
                Dir::N => (x, y + 1),
                Dir::E => (x + 1, y),
                Dir::S => (x, y - 1),
                Dir::W => (x - 1, y),
            };
        }
        (b + c as u16, a + ((x + ox) * (y - oy)))
    });
    // use shoelace formula to get the area, then use picks formula to count the number of inner points
    ((a.abs() / 2) as u16) + (1 + b / 2)
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
