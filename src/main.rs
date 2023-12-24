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
use core::intrinsics::{fadd_fast as af, fdiv_fast as df, fmul_fast as mf, fsub_fast as sf};
pub mod util;
use std::mem::MaybeUninit;

pub use util::prelude::*;

pub unsafe fn intersect(
    p0x: f64,
    p0y: f64,
    v0x: f64,
    v0y: f64,
    p1x: f64,
    p1y: f64,
    v1x: f64,
    v1y: f64,
) -> Option<(f64, f64)> {
    let x = df(
        sf(
            af(sf(p1y, p0y), mf(p0x, df(v0y, v0x))),
            mf(p1x, df(v1y, v1x)),
        ),
        sf(df(v0y, v0x), df(v1y, v1x)),
    );
    let t0 = df(sf(x, p0x), v0x);
    let t1 = df(sf(x, p1x), v1x);
    (t0 > 0. && t1 > 0.).then_some((x, af(p0y, mf(t0, v0y))))
}

pub fn run(i: &str) -> impl Display {
    let mut v: [MaybeUninit::<_>; 300] =
    // SAFETY: mu likes this
    unsafe { MaybeUninit::uninit().assume_init() };
    let mut x = i.as_bytes();
    for i in 0..300 {
        let α = 読む::迄::<i64>(&mut x, b',') as f64;
        x.skip(1);
        let β = 読む::迄::<i64>(&mut x, b',') as f64;
        x.skip(2);
        // memchr bad here
        while x.by().ψ() != b' ' {}
        x.skip(2);
        let δ = 読む::負迄(&mut x, b',') as f64;
        x.skip(1);
        let ε = 読む::負迄(&mut x, b',') as f64;
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
