#![allow(
    confusable_idents,
    uncommon_codepoints,
    internal_features,
    mixed_script_confusables,
    incomplete_features
)]
#![feature(
    slice_swap_unchecked,
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

use umath::{generic_float::Constructors, FF64};
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

#[no_mangle]
pub fn p2(i: &str) -> impl Display {
    let mut v: [MaybeUninit::<_>; 3] =
    // SAFETY: mu likes this
    unsafe { MaybeUninit::uninit().assume_init() };
    let mut x = i.as_bytes();
    for i in 0..3 {
        let α = unsafe { FF64::new(読む::迄::<i64>(&mut x, b',') as f64) };
        x.skip(1);
        let β = unsafe { FF64::new(読む::迄::<i64>(&mut x, b',') as f64) };
        x.skip(1);
        let γ = unsafe { FF64::new(読む::迄::<i64>(&mut x, b' ') as f64) };
        x.skip(2);
        let δ = unsafe { FF64::new(読む::負迄(&mut x, b',') as f64) };
        x.skip(1);
        let ε = unsafe { FF64::new(読む::負迄(&mut x, b',') as f64) };
        x.skip(1);
        let ζ = unsafe { FF64::new(読む::負迄(&mut x, b'\n') as f64) };
        v[i].write(([α, β, γ], [δ, ε, ζ]));
    }
    let [([x0, y0, z0], [v0x, v0y, v0z]), ([x1, y1, z1], [v1x, v1y, v1z]), ([x2, y2, z2], [v3x, v3y, v3z])] =
        v.map(|elem| unsafe { elem.assume_init() });

    // credit: giooschi
    let z = unsafe { FF64::zero() };
    #[rustfmt::skip]
    let mut coeffs = [
        [z, -v0z + v1z, v0y - v1y, z, z0 - z1, -y0 + y1,  v0y * z0 - v1y * z1 - v0z * y0 + v1z * y1],
        [v0z - v1z, z, -v0x + v1x, -z0 + z1, z, x0 - x1, -v0x * z0 + v1x * z1 + v0z * x0 - v1z * x1],
        [-v0y + v1y, v0x - v1x, z, y0 - y1, -x0 + x1, z,  v0x * y0 - v1x * y1 - v0y * x0 + v1y * x1],
        [z, -v1z + v3z, v1y - v3y, z, z1 - z2, -y1 + y2,  v1y * z1 - v3y * z2 - v1z * y1 + v3z * y2],
        [v1z - v3z, z, -v1x + v3x, -z1 + z2, z, x1 - x2, -v1x * z1 + v3x * z2 + v1z * x1 - v3z * x2],
        [-v1y + v3y, v1x - v3x, z, y1 - y2, -x1 + x2, z,  v1x * y1 - v3x * y2 - v1y * x1 + v3y * x2],
    ];

    for i in 0..6 {
        let j = (i..6)
            .max_by(|&j, &k| unsafe {
                FF64::cmp(
                    &FF64::new(coeffs[j][i].abs()),
                    &FF64::new(coeffs[k][i].abs()),
                )
            })
            .ψ();
        unsafe { coeffs.swap_unchecked(i, j) };
        (i..7).rev().for_each(|j| coeffs[i][j] /= coeffs[i][i]);
        for j in i + 1..6 {
            for k in (i..7).rev() {
                coeffs[j][k] -= coeffs[i][k] * coeffs[j][i];
            }
        }
    }

    for i in (1..6).rev() {
        for j in 0..i {
            coeffs[j][6] -= coeffs[j][i] * coeffs[i][6];
            coeffs[j][i] = z;
        }
    }

    *(coeffs[0][6] + coeffs[1][6] + coeffs[2][6]) as u64
}

pub fn p1(i: &str) -> impl Display {
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
