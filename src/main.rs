#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(
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

pub fn p1(i: &str) -> usize {
    let mut mat = Vec::with_capacity(100);
    let mut i = i.as_bytes();
    for _ in 0..100 {
        mat.push(unsafe { <&[u8; 100]>::try_from(i.get_unchecked(..100)).unwrap() });
        i = unsafe { i.get_unchecked(100..) };
        if i.len() != 0 {
            i = unsafe { i.get_unchecked(1..) };
        }
    }
    let mut rows = [0u8; 100];
    for j in 0..100 {
        let mut count = 0;
        for i in 0..100 {
            if *unsafe { mat.get_unchecked(i).get_unchecked(j) } == b'O' {
                *unsafe { rows.get_unchecked_mut(count) } += 1;
                count += 1;
            } else if mat[i][j] == b'#' {
                count = i + 1;
            }
        }
    }
    rows.iter()
        .ι::<usize>()
        .map(|(&x, i)| (x.nat()) * (100 - i))
        .sum::<usize>()
}

fn pushhhh(mat: &mut Vec<[u8; 100]>) {
    for j in 0..100 {
        let mut count = 0;
        for i in 0..100 {
            let curr = C! { mat[i][j] };
            mat!(curr {
                b'O' => {
                    C! { mat[i][j] = mat[count][j] };
                    C! { mat[count][j] = curr };
                    count += 1;
                },
                b'#' => count = i + 1,
                b'.' => {},
            });
        }
    }
}

#[no_mangle]
pub fn run(i: &str) -> impl Display {
    p2(i)
}

fn weigh(mat: &[[u8; 100]]) -> u32 {
    mat.iter()
        .ι::<usize>()
        .map(|(row, i)| (util::count::<100>(&row, b'O') * (100 - i)) as u32)
        .sum()
}

pub fn h(data: &[u8; 10000]) -> u16 {
    let mut buffer = 509;
    let mut data = &data[..];
    while data.len() > 16 {
        let (value, rest) = data.split_at(16);
        let [a, b]: [u64; 2] = unsafe { rint(<[u8; 16]>::try_from(value).unwrap()) };
        buffer ^= a.wrapping_mul(b) ^ a;
        data = rest;
    }
    (buffer ^ buffer.swap_bytes()) as u16
}

fn hash(x: &[[u8; 100]]) -> u16 {
    h(&unsafe { *x.as_ptr().cast() })
}

pub fn p2(i: &str) -> impl Display {
    let mut mat = Vec::with_capacity(100);
    let mut i = i.as_bytes();
    for _ in 0..100 {
        mat.push(unsafe { <[u8; 100]>::try_from(i.get_unchecked(..100)).unwrap() });
        i = unsafe { i.get_unchecked(100..) };
        if i.len() != 0 {
            i = unsafe { i.get_unchecked(1..) };
        }
    }
    let mut map = vec![(hash(&mat), weigh(&mat))];
    loop {
        for _ in 0..4 {
            pushhhh(&mut mat);
            let mut new = vec![[0; 100]; 100];
            for row in 0..100 {
                for col in 0..100 {
                    C! { new[col][99 - row] = mat[row][col] };
                }
            }
            mat = new;
        }
        let c = hash(&mat);
        if let Some(y) = map.iter().position(|&(x, _)| x == c) {
            let c = map.len() - y;
            return C! { map[y + (1e9 as usize - y) % c] }.1;
        }
        map.push((c, weigh(&mat)));
    }
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
