#![feature(generic_const_exprs, portable_simd)]
use std::{ops::Range, simd::prelude::*};
#[unsafe(no_mangle)]
pub unsafe fn run(x: &'static [u8]) -> u64 {
    let width = 3712 + memchr::memchr(b'\n', x.get_unchecked(3712..)).unwrap_unchecked();
    let numbers = [
        x,
        x.get_unchecked(width + 1..),
        x.get_unchecked((width + 1) * 2..),
        x.get_unchecked((width + 1) * 3..),
    ];
    let mut tot = 0;
    let ll = x.get_unchecked((width + 1) * 4..(width + 1) * 5 - 1);
    let mut at = 0;
    while let Some((index, mut mask)) = {
        let sl = ll.get(at..).unwrap_unchecked();
        let x = u8x64::load_or(sl, Simd::splat(b' '));
        let mask = x.simd_ne(Simd::splat(b' ')).to_bitmask() as u64;
        (mask != 0).then(|| {
            (
                at + mask.trailing_zeros() as usize,
                mask ^ (mask & mask.wrapping_neg()),
            )
        })
    } {
        let mut x = Simd::splat(0u16);
        for i in 0..4 {
            let c = u8x64::from_slice(numbers.get_unchecked(i).get_unchecked(index..index + 64));
            let m = c.simd_ne(Simd::splat(b' '));
            x = m
                .cast::<i16>()
                .select(x * Simd::splat(10) + (c - Simd::splat(b'0')).cast(), x);
        }
        let f = |ix: Range<usize>| {
            // let iter1 = ix.clone().map(|i| {
            //     numbers
            //         .iter()
            //         .filter(|x| *x.get_unchecked(index + i) != b' ')
            //         .fold(0, |acc, x| {
            //             acc * 10 + (x.get_unchecked(index + i) - b'0') as u64
            //         })
            // });
            // let mut tot_ = 0;
            // match *ll.get_unchecked(index + ix.start) {
            //     b'+' => tot_ += iter1.clone().sum::<u64>(),
            //     b'*' => tot_ += iter1.clone().product::<u64>(),
            //     _ => panic!(),
            // }

            // print!("{:?} = ", iter1.clone().collect::<Vec<_>>());
            // println!("{:?} {}", &x[ix.clone()], ll[index + ix.start] as char);
            const IDX: u32x64 = u32x64::from_array([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
                44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
            ]);

            let i = x
                .as_array()
                .get_unchecked(ix.clone())
                .iter()
                .map(|&x| x as u64);
            let x = match *ll.get_unchecked(index + ix.start) {
                b'+' => {
                    let mask = IDX.simd_lt(Simd::splat(ix.end as _))
                        & IDX.simd_ge(Simd::splat(ix.start as _));
                    (mask.to_int().cast::<u32>() & x.cast::<u32>()).reduce_sum() as u64
                }
                _ => i.product::<u64>(),
            };
            // assert_eq!(x, tot_);
            x
        };
        if mask.leading_zeros() as usize == 64 {
            return tot + f(0..ll.len() - index);
        }

        let mut last = 0;
        let mut hold: Option<Range<usize>> = None;

        while mask != 0 {
            let next = mask.trailing_zeros() as usize;
            mask ^= mask & mask.wrapping_neg();
            if *ll.get_unchecked(index + last) == b'+' {
                if let Some(r) = &mut hold {
                    r.end = next;
                } else {
                    hold = Some(last..next);
                }
            } else {
                // *
                if let Some(r) = hold.take() {
                    tot += f(r);
                }
                tot += f(last..next - 1);
            }

            last = next;
        }

        if let Some(r) = hold {
            tot += f(r);
        }

        at += last;
    }
    tot
}
