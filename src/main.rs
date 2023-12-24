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
use z3::ast::{Ast, Int};

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

pub fn p2(i: &str) -> impl Display {
    let mut v: [MaybeUninit::<_>; 300] =
    // SAFETY: mu likes this
    unsafe { MaybeUninit::uninit().assume_init() };
    let mut x = i.as_bytes();
    for i in 0..300 {
        let α = 読む::迄::<i64>(&mut x, b',');
        x.skip(1);
        let β = 読む::迄::<i64>(&mut x, b',');
        x.skip(1);
        let γ = 読む::迄::<i64>(&mut x, b' ');
        x.skip(2);
        let δ = 読む::負迄(&mut x, b',');
        x.skip(1);
        let ε = 読む::負迄(&mut x, b',');
        x.skip(1);
        let ζ = {
            let (sign, mut n) = match x.by().ψ() {
                b'-' => (-1, 0),
                b => (1, i64::from(b - b'0')),
            };
            while let Ok(b) = x.by()
                && b != b'\n'
            {
                n = n * 10 + i64::from(b - b'0');
            }
            n * sign as i64
        };
        v[i].write(([α, β, γ], [δ, ε, ζ]));
    }
    let v = v.map(|elem| unsafe { elem.assume_init() });
    let c = z3::Context::new(&z3::Config::new());
    let s = z3::Solver::new(&c);
    macro_rules! dec {
        (from $($x:ident)+) => {
            $(let $x = z3::ast::Int::from_i64(&c, $x);)+
        };
        ($($x:ident)+) => {
            $(let $x = z3::ast::Int::new_const(&c, stringify!($x));)+
        };
    }
    dec!(x y z vx vy vz);
    for (([xi, yi, zi], [vxi, vyi, vzi]), i) in v.into_iter().ι::<u16>() {
        dec!(from xi yi zi vxi vyi vzi);
        let ti = Int::new_const(&c, format!("t{i}"));
        s.assert(&(xi + vxi * &ti)._eq(&(&x + &vx * &ti)));
        s.assert(&(yi + vyi * &ti)._eq(&(&y + &vy * &ti)));
        s.assert(&(zi + vzi * &ti)._eq(&(&z + &vz * &ti)));
    }
    s.check();
    let r = s
        .get_model()
        .unwrap()
        .eval(&(x + y + z), true)
        .unwrap()
        .as_i64()
        .unwrap();
    r
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
