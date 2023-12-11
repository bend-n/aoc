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

    #[inline]
    fn space(self) -> bool {
        self == Self::Space
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

impl Map<'_> {
    fn at(&self, x: u8, y: u8) -> Element {
        self[y.widen() * W + x.widen()]
    }

    fn solve(&self) -> usize {
        let mut n = 0u8;
        let emptyr = {
            self.map
                .chunks(W.nat())
                .map(|x| {
                    if x.iter().take(S.nat()).copied().all(Element::space) {
                        n += 1;
                    }
                    n
                })
                .Ν::<{ S as usize }>()
        };
        let mut n = 0u8;
        let emptyc = (0..S)
            .map(move |x| (0..S).map(move |y| self.at(x, y)))
            .map(|mut x| {
                if x.all(Element::space) {
                    n += 1;
                }
                n
            })
            .Ν::<{ S as usize }>();

        (0..S)
            .flat_map(|y| (0..S).map(move |x| (x, y)))
            .filter(|&(x, y)| self.at(x, y).galaxy())
            .combine()
            .map(|((x1, y1), (x2, y2))| {
                let expand = 2;
                // let expand = 1000000;
                ((y1 as i16 - y2 as i16).abs() + (x1 as i16 - x2 as i16).abs()) as usize
                    + (expand - 1)
                        * (emptyr[y1.max(y2).nat()].nat() - emptyr[y1.min(y2).nat()].nat())
                    + (expand - 1)
                        * (emptyc[x1.max(x2).nat()].nat() - emptyc[x1.min(x2).nat()].nat())
            })
            .sum()
    }
}

impl From<&[u8]> for Map<'_> {
    fn from(i: &[u8]) -> Self {
        Self {
            map: unsafe { core::mem::transmute(i) },
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
