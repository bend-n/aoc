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
    iter_repeat_n,
    slice_swap_unchecked,
    generic_const_exprs,
    iter_array_chunks,
    if_let_guard,
    const_mut_refs,
    get_many_mut,
    maybe_uninit_uninit_array,
    once_cell_get_mut,
    iter_collect_into,
    hint_assert_unchecked,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    vec_into_raw_parts,
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
pub use util::prelude::*;

const WIDTH: usize = 71;
pub unsafe fn run(x: &str) -> impl Display {
    let mut grid = [[0; WIDTH + 1]; WIDTH + 1];
    let mut add = x.行().map(|x| x.μ(',').mb(|x| x.λ::<u8>()));
    loop {
        let x = add.next().unwrap();
        grid[x.1.nat()][x.0.nat()] = 1;
        if util::dijkstra(
            |x: (usize, usize)| {
                [Dir::N + x, Dir::E + x, Dir::S + x, Dir::W + x]
                    .into_iter()
                    .filter(|&(x, y)| grid.get(y).and_then(|y| y.get(x)) == Some(&0))
                    .map(|x| (x, 1))
            },
            (0, 0),
            |x| x == (WIDTH, WIDTH),
        )
        .is_none()
        {
            return format!("{x:?}");
        }
    }

    // 0
}

pub struct UnionFind {
    pub p: [u16; 5184],
    pub s: [u16; 5184],
}

impl UnionFind {
    pub const fn new() -> Self {
        Self {
            s: [1; 5184],
            p: car::map!(atools::range(), |x| x as u16),
        }
    }

    pub const fn find(&mut self, key: u16) -> u16 {
        if unsafe { *self.p.as_ptr().add(key as usize) } == key {
            return key;
        }
        let parent = self.find(unsafe { *self.p.as_ptr().add(key as usize) });
        unsafe { *self.p.as_mut_ptr().add(key as usize) = parent };
        parent
    }

    pub const fn union(&mut self, a: u16, b: u16) -> bool {
        let α = self.find(a);
        let β = self.find(b);
        if α == β {
            return false;
        }
        let a = unsafe { *self.s.as_ptr().add(α as usize) };
        let b = unsafe { *self.s.as_ptr().add(β as usize) };
        if a >= b {
            unsafe { *self.s.as_mut_ptr().add(α as usize) += b };
            unsafe { *self.p.as_mut_ptr().add(β as usize) = α };
        } else {
            unsafe { *self.s.as_mut_ptr().add(β as usize) += a };
            unsafe { *self.p.as_mut_ptr().add(α as usize) = β };
        }
        true
    }
}

pub fn part2(x: &str) -> impl Display {
    let add = x.行().map(|x| x.μ(',').mb(|x| x.λ::<u8>()));
    macro_rules! dex {
        ($x:expr, $y:expr) => {
            unsafe {
                (($y as u16)
                    .unchecked_mul((WIDTH as u16 + 1))
                    .unchecked_add($x as u16))
            }
        };
    }
    const W: u8 = WIDTH as u8;
    const UF: UnionFind = {
        let mut f = UnionFind::new();
        let mut i = 0;
        while i < W - 1 {
            f.union(dex!(i + 1, 0), dex!(i + 2, 0));
            f.union(dex!(i, W), dex!(i + 1, W));
            f.union(dex!(0, i + 1), dex!(0, i + 2));
            f.union(dex!(W, i), dex!(W, i + 1));
            i += 1;
        }
        f
    };
    let mut uf = UF;
    for (x, y) in add {
        let i = dex!(x, y);
        uf.union(i, dex!(x + 1, y));
        uf.union(i, dex!(x, y + 1));
        uf.union(i, dex!(x + 1, y + 1));

        if uf.find(dex!(0, 1)) == uf.find(dex!(1, 0)) {
            return format!("{},{}", x, y);
        }
    }
    dang!();
}

fn main() {
    let s = include_str!("inp.txt");
    println!("{}", unsafe { part2(s) });
    // dbg!(exec(&program, regi));
}
#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| part2(i));
}
