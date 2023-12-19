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

pub fn p1(i: &str) -> u16 {
    let mut y = 0i32;
    let mut a = 0i32;
    let mut b = 0u16;
    let mut i = i.as_bytes();
    loop {
        let d = unsafe {
            rint::<_, Dir>(match i.by() {
                Ok(x) => x,
                Err(_) => break,
            })
        };
        let c = match unsafe { i.rd::<3>().unwrap_unchecked() } {
            [_, b, b' '] => {
                i = C! { &i[10..] };
                b - b'0'
            }
            [_, a, b] => {
                _ = i.read(&mut [0; 11]);
                // i = C! { &i[11..] };
                (a - b'0') * 10 + (b - b'0')
            }
        };
        b += c as u16;
        match d {
            Dir::N => y += c as i32,
            Dir::E => a += c as i32 * y,
            Dir::S => y -= c as i32,
            Dir::W => a -= c as i32 * y,
        }
    }

    // use shoelace formula to get the area, then use picks formula to count the number of inner points
    a.abs() as u16 + (1 + b / 2)
}

pub fn p2(i: &str) -> u64 {
    let mut y = 0;
    let mut a = 0;
    let mut b = 0;
    let mut i = i.as_bytes();
    loop {
        let Ok(_) = i.by() else { break };
        i.skip(2);
        if unsafe { i.by().unwrap_unchecked() == b' ' } {
            i.skip(2);
        } else {
            i.skip(3);
        }
        let dat = unsafe { i.rd::<6>().unwrap_unchecked() };
        _ = i.read(&mut [0; 2]);
        let c = 読む::hex5(dat[0..5].try_into().unwrap());
        const A: [i64; 4] = [1, 0, -1, 0];
        const Y: [i64; 4] = [0, -1, 0, 1];
        let d = dat[5] - b'0';
        a += C! { A[d.nat()] } * c as i64 * y;
        y += C! { Y[d.nat()] } * c as i64;
        b += c as u64;
    }
    a.abs() as u64 + (1 + b / 2)
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
