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
            Dir::N => y = y + (c as i32),
            Dir::E => a = a + (c as i32 * y),
            Dir::S => y = y - (c as i32),
            Dir::W => a = a - (c as i32 * y),
        }
    }

    // use shoelace formula to get the area, then use picks formula to count the number of inner points
    a.abs() as u16 + (1 + b / 2)
}

pub fn p2(i: &str) -> u64 {
    let mut x = 0i32;
    let mut y = 0i32;
    let (b, a) = i.行().fold((0, 0), |(b, a), i| {
        let dat = unsafe {
            &*(if C! { i[3] } == b' ' {
                C! { &i[6..] }
            } else {
                C! { &i[7..] }
            }
            .as_ptr() as *const [u8; 6])
        };
        let c = 読む::hex5(dat[0..5].try_into().unwrap());
        let (ox, oy) = (x, y);
        for _ in 0..c {
            let d = 読む::hex_dig(dat[5]);
            (x, y) = mat!(d {
                0 => (x + 1, y),
                1 => (x, y - 1),
                2 => (x - 1, y),
                3 => (x, y + 1),
            });
        }
        (
            b + c as u64,
            a + ((x as i64 + ox as i64) * (y as i64 - oy as i64)),
        )
    });

    ((a.abs() / 2) as u64) + (1 + b / 2)
}

pub fn run(i: &str) -> impl Display {
    p1(i)
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
