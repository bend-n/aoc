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

#[derive(Copy, Clone)]
struct Piece {
    a: [u64; 3],
    b: [u64; 3],
}

impl std::fmt::Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [x1, y1, z1] = self.a;
        let [x2, y2, z2] = self.b;
        write!(f, "[Vector3({x1}, {z1}, {y1}), Vector3({x2}, {z2}, {y2})]")
    }
}

pub fn run(i: &str) -> impl Display {
    let mut pieces = vec![];
    i.行()
        .map(|x| {
            let (a, b) = x
                .μ('~')
                .mb(|x| x.split(|&x| x == b',').map(読む::完了).Ν::<3>());
            pieces.push(Piece { a, b });
        })
        .Θ();
    println!("{pieces:?}");
    std::process::exit(0);
    0
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
