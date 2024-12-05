#![allow(
    confusable_idents,
    uncommon_codepoints,
    non_upper_case_globals,
    internal_features,
    mixed_script_confusables,
    static_mut_refs,
    incomplete_features
)]
#![feature(
    slice_swap_unchecked,
    generic_const_exprs,
    iter_array_chunks,
    get_many_mut,
    maybe_uninit_uninit_array,
    iter_collect_into,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    try_blocks,
    slice_take,
    portable_simd,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    core_intrinsics
)]
extern crate test;
pub mod util;
use atools::prelude::*;
pub use util::prelude::*;
const SIZE: usize = 140;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
struct bitset140([u64; 3]);
impl bitset140 {
    #[inline]
    fn count_ones(self) -> u32 {
        self.0.iter().copied().map(u64::count_ones).sum()
    }
}

impl std::ops::Shr<u32> for bitset140 {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: u32) -> Self::Output {
        let bitset140([a, b, c]) = self;
        Self([
            a >> rhs,
            b >> rhs | a << (u64::BITS - rhs),
            c >> rhs | b << (u64::BITS - rhs),
        ])
    }
}
impl std::ops::Shl<u32> for bitset140 {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: u32) -> Self::Output {
        let bitset140([a, b, c]) = self;
        Self([
            a << rhs | b >> (u64::BITS - rhs),
            b << rhs | c >> (u64::BITS - rhs),
            c << rhs,
        ])
    }
}

impl std::ops::BitAnd<bitset140> for bitset140 {
    type Output = Self;

    #[inline]
    fn bitand(self, bitset140(rhs): bitset140) -> Self::Output {
        bitset140(self.0.zip(rhs).map(|(a, b)| a & b))
    }
}

use std::simd::prelude::*;
#[no_mangle]
pub fn run(i: &str) -> impl Display {
    fn load(i: &[u8; SIZE + 1]) -> [bitset140; 4] {
        let loaded = [
            u8x64::load_or_default(&i[128..]),
            u8x64::from_slice(&i[64..128]),
            u8x64::from_slice(&i[..64]),
        ];

        [
            bitset140(loaded.map(|x| x.simd_eq(Simd::splat(b'X')).to_bitmask())),
            bitset140(loaded.map(|x| x.simd_eq(Simd::splat(b'M')).to_bitmask())),
            bitset140(loaded.map(|x| x.simd_eq(Simd::splat(b'A')).to_bitmask())),
            bitset140(loaded.map(|x| x.simd_eq(Simd::splat(b'S')).to_bitmask())),
        ]
    }
    unsafe {
        let grid = i.as_bytes().as_chunks_unchecked::<{ SIZE + 1 }>();
        // let masked: [masks; SIZE] = std::array::from_fn(|i| load(grid.get_unchecked(i)));
        let mut sum = 0;
        let mut prev_x = [bitset140::default(); 4];
        let mut prev_m = [bitset140::default(); 4];
        let mut prev_a = [bitset140::default(); 4];
        let mut prev_s = [bitset140::default(); 4];

        for y in 0..=2 {
            let [x, m, a, s] = load(grid.get_unchecked(y));
            sum += (x & m << 1 & a << 2 & s << 3).count_ones();
            sum += (x & m >> 1 & a >> 2 & s >> 3).count_ones();
            prev_x[y + 1] = x;
            prev_m[y + 1] = m;
            prev_a[y + 1] = a;
            prev_s[y + 1] = s;
            // prev[y + 1] = masks;
        }
        for y in 3..SIZE {
            let [x, m, a, s] = load(grid.get_unchecked(y));
            let [_, p1, p2, p3] = prev_x;
            prev_x = [p1, p2, p3, x];
            let [_, p1, p2, p3] = prev_m;
            prev_m = [p1, p2, p3, m];
            let [_, p1, p2, p3] = prev_a;
            prev_a = [p1, p2, p3, a];
            let [_, p1, p2, p3] = prev_s;
            prev_s = [p1, p2, p3, s];

            // println!("{a:?}");
            // // ↘→←↓↙→←↑↗↖↗↖↗→↖↗↑↖18
            // // overlap right
            sum += (x & m << 1 & a << 2 & s << 3).count_ones();
            // // overlap left
            sum += (x & m >> 1 & a >> 2 & s >> 3).count_ones();

            // // idea is to overlap and then count
            // // idea comes from gioschii
            // if y >= 3
            //
            {
                let x = x;
                let m = prev_m[3 - 1];
                let a = prev_a[3 - 2];
                let s = prev_s[3 - 3];

                // down diagonal
                sum += (x & m << 1 & a << 2 & s << 3).count_ones();
                sum += (x & m >> 1 & a >> 2 & s >> 3).count_ones();

                // vertical
                sum += (x & m & a & s).count_ones();
            }
            //
            {
                let s = s;
                let a = prev_a[3 - 1];
                let m = prev_m[3 - 2];
                let x = prev_x[3 - 3];

                // up diagonals
                sum += (x & m << 1 & a << 2 & s << 3).count_ones();
                sum += (x & m >> 1 & a >> 2 & s >> 3).count_ones();

                sum += (x & m & a & s).count_ones();
            }
            // println!(
            //     "{y}: {:064b}{:064b}{:064b}",
            //     // sum - sum_,
            //     x.0[0],
            //     x.0[1],
            //     x.0[2]
            // );
        }
        sum
    }
}
#[no_mangle]
pub fn two(i: &str) -> impl Display {
    // let grid = i.as_bytes();
    // let get = |x: usize, y: usize| -> u8 { unsafe { *grid.get_unchecked(y * (SIZE + 1) + x) } };
    // let mut sum = 0;
    // for y in 1..SIZE - 1 {
    //     for x in 1..SIZE - 1 {
    //         let a = get(x - 1, y - 1);
    //         let b = get(x + 1, y + 1);
    //         let c = get(x + 1, y - 1);
    //         let d = get(x - 1, y + 1);
    //         sum +=
    //             (((a ^ b) + (c ^ d) == (b'S' ^ b'M') + (b'S' ^ b'M')) & (get(x, y) == b'A')) as u32;
    //     }
    // }
    // sum

    let g = i.as_bytes();
    assert_eq!(g.len(), 141 * 140);
    let mut sum = 0;
    // chunks of 64
    for i in 0..304 {
        // tl
        let a = u8x64::from_slice(&g[64 * i..]);
        // trd
        let b = u8x64::from_slice(&g[64 * i + 2..]);
        // mid
        let current = u8x64::from_slice(&g[141 + 64 * i + 1..]);
        // bl
        let d = u8x64::from_slice(&g[141 * 2 + 64 * i..]);
        // br
        let c = u8x64::from_slice(&g[141 * 2 + 64 * i + 2..]);
        let wz = ((a ^ c) + (b ^ d)).simd_eq(u8x64::splat(60));
        let a = current.simd_eq(u8x64::splat(b'A'));
        sum += (wz & a).to_bitmask().count_ones();
    }
    sum
}

fn main() {
    // let mut s = String::new();
    // for i in 0..1280 {
    let i = include_str!("inp.txt");
    //     s.push_str(i);
    // }
    // std::fs::write("src/inp.txt", s);
    // prinan!("{}", p1(i));
    println!("{}", run(i));
    println!("{}", two(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| run(i));
}
