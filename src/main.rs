#![allow(
    confusable_idents,
    uncommon_codepoints,
    internal_features,
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
    core_intrinsics,
    byte_slice_trim_ascii
)]
extern crate test;
pub mod util;
use std::mem::MaybeUninit;

pub use util::prelude::*;

pub unsafe fn intersect(
    p0x: f32,
    p0y: f32,
    v0x: f32,
    v0y: f32,
    p1x: f32,
    p1y: f32,
    v1x: f32,
    v1y: f32,
) -> Option<(f32, f32)> {
    let x = ((p1y - p0y) + p0x * (v0y / v0x) - p1x * (v1y / v1x)) / ((v0y / v0x) - (v1y / v1x));
    let t0 = (x - p0x) / v0x;
    let t1 = (x - p1x) / v1x;
    (t0 > 0. && t1 > 0.).then_some((x, p0y + t0 * v0y))
}

pub fn run(i: &str) -> impl Display {
    let mut v: [MaybeUninit::<_>; 300] =
    // SAFETY: mu likes this
    unsafe { MaybeUninit::uninit().assume_init() };
    let mut x = i.as_bytes();
    for i in 0..300 {
        let α = 読む::迄::<i64>(&mut x, b',') as f32;
        x.skip(1);
        let β = 読む::迄::<i64>(&mut x, b',') as f32;
        x.skip(2);
        // memchr bad here
        while x.by().ψ() != b' ' {}
        x.skip(2);
        let δ = 読む::負迄(&mut x, b',') as f32;
        x.skip(1);
        let ε = 読む::負迄(&mut x, b',') as f32;
        x.skip(1);
        if let Some(n) = memchr::memchr(b'\n', x) {
            x.skip(n + 1);
        }
        v[i].write(([α, β], [δ, ε]));
    }
    let v = v.map(|elem| unsafe { elem.assume_init() });
    let mut sum = 0;
    for (i, &([x0, y0], [v0x, v0y])) in v.iter().enumerate() {
        for &([x1, y1], [v1x, v1y]) in &v[i..] {
            if let Some((x, y)) = unsafe { intersect(x0, y0, v0x, v0y, x1, y1, v1x, v1y) } {
                let min = 200000000000000.;
                let max = 400000000000000.;
                if x > min && x < max && y > min && y < max {
                    sum += 1;
                }
            }
        }
    }
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
