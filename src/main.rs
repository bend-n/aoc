#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(
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
mod util;
pub use util::prelude::*;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Element {
    #[allow(dead_code)]
    Space = b'.',
    Galaxy = b'#',
    #[allow(dead_code)]
    New = 10,
}

impl Element {
    #[inline]
    fn galaxy(self) -> bool {
        self == Self::Galaxy
    }
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self as u8 == 10 {
            return write!(f, "@");
        }
        write!(f, "{}", *self as u8 as char)
    }
}

const S: u8 = 140;
const W: u16 = S as u16 + 1;

struct Map<'a> {
    map: &'a [Element],
}

impl<'a> std::ops::Index<u16> for Map<'a> {
    type Output = Element;

    fn index(&self, index: u16) -> &'a Self::Output {
        unsafe { self.map.get_unchecked(index.nat()) }
    }
}

pub unsafe fn galaxies(line: &[u8]) -> usize {
    use std::arch::x86_64::*;
    let galaxy = _mm256_set1_epi8(b'#' as i8);
    let mut counts = _mm256_setzero_si256();
    for i in 0..4 {
        counts = _mm256_sub_epi8(
            counts,
            _mm256_cmpeq_epi8(
                _mm256_loadu_si256(line.as_ptr().add(i * 32) as *const _),
                galaxy,
            ),
        );
    }
    const MASK: [u8; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ];
    counts = _mm256_sub_epi8(
        counts,
        _mm256_and_si256(
            _mm256_cmpeq_epi8(
                _mm256_loadu_si256(line.as_ptr().add(108) as *const _),
                galaxy,
            ),
            _mm256_loadu_si256(MASK.as_ptr().add(12) as *const _),
        ),
    );

    let sums = _mm256_sad_epu8(counts, _mm256_setzero_si256());
    (_mm256_extract_epi64(sums, 0)
        + _mm256_extract_epi64(sums, 1)
        + _mm256_extract_epi64(sums, 2)
        + _mm256_extract_epi64(sums, 3)) as usize
}

impl Map<'_> {
    fn at(&self, x: u8, y: u8) -> Element {
        self[y.widen() * W + x.widen()]
    }

    fn solve(&self) -> usize {
        let mut sum = 0;
        let mut above = 0;
        let mut upward = 0;
        // const FACTOR: usize = 2;
        const FACTOR: usize = 1000000;
        for has in self.map.chunks(W.nat()).map(|x| {
            #[cfg(target_feature = "avx")]
            return unsafe { galaxies(std::mem::transmute(x)) };
            #[cfg(not(target_feature = "avx"))]
            return x.iter().filter(|x| x.galaxy()).count();
        }) {
            if has == 0 {
                upward += above * FACTOR;
            } else {
                sum += upward * has;
                above += has;
                upward += above;
            }
        }
        let mut beside = 0;
        let mut left = 0;
        #[cfg(target_feature = "avx")]
        let mut x = [Element::Space; 140];
        for has in (0..S)
            .map(move |x| (0..S).map(move |y| self.at(x, y)))
            .map(|mut y| {
                #[cfg(not(target_feature = "avx"))]
                return y.filter(|x| x.galaxy()).count();
                #[cfg(target_feature = "avx")]
                return {
                    y.ν(&mut x);
                    unsafe { galaxies(&std::mem::transmute::<_, [u8; 140]>(x)) }
                };
            })
        {
            if has == 0 {
                left += beside * FACTOR;
            } else {
                sum += left * has;
                beside += has;
                left += beside;
            }
        }
        sum
    }
}

impl From<&[u8]> for Map<'_> {
    fn from(i: &[u8]) -> Self {
        Self {
            map: &unsafe { core::mem::transmute::<_, &[Element]>(i) }[..(W.nat() * S.nat()) - 1],
        }
    }
}

pub fn run(i: &str) -> impl Display {
    let map = Map::from(i.as_bytes());
    map.solve()
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
