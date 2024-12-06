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
pub use util::prelude::*;

const SIZE: usize = 130;

#[derive(Clone, Copy)]
struct bitset16960([u64; 265]);
impl bitset16960 {
    fn new() -> Self {
        Self([0; 265])
    }
    #[inline(always)]
    fn set(&mut self, index: usize) {
        unsafe { *self.0.get_unchecked_mut(index >> 6) |= 1u64.wrapping_shl(index as u32) };
    }
    #[inline(always)]
    fn get(&self, index: usize) -> bool {
        unsafe { *self.0.get_unchecked(index >> 6) & 1u64.wrapping_shl(index as u32) != 0 }
    }
    #[inline(always)]
    fn popcnt(self) -> u32 {
        self.0.into_iter().map(u64::count_ones).sum::<u32>()
    }
}

// type bits = bitvec::BitArr!(for 16960, in u64);
// #[derive(Clone, Copy)]
// struct bitset16960(bits);
// impl bitset16960 {
//     fn new() -> Self {
//         Self(bits::default())
//     }
//     #[inline(always)]
//     #[no_mangle]
//     fn set(&mut self, index: usize) {
//         unsafe { self.0.set_unchecked(index, true) };
//     }
//     #[inline(always)]
//     #[no_mangle]
//     fn get(&self, index: usize) -> bool {
//         unsafe { *self.0.get_unchecked(index) }
//     }
//     #[inline(always)]
//     fn popcnt(self) -> u32 {
//         self.0.count_ones() as _
//     }
// }

fn follow(i: &[u8]) -> bitset16960 {
    let mut marked = bitset16960::new();
    let grid = unsafe { i.as_chunks_unchecked::<{ SIZE + 1 }>() };
    let guard = memchr::memchr(b'^', i).ψ();
    let mut y = (guard / SIZE) as i64;
    let mut x = (guard % (SIZE + 1)) as i64;
    let mut pointing = 0u8;
    loop {
        marked.set(y as usize * SIZE + x as usize);
        let (check_x, check_y) = mat!(pointing {
            0 => (x, y - 1),
            1 => (x + 1, y),
            2 => (x, y + 1),
            3 => (x - 1, y),
        });
        if (check_y >= SIZE as i64) || (check_y < 0) || (check_x >= SIZE as i64) || (check_x < 0) {
            return marked;
        }
        if C! { grid[check_y as usize] [check_x as usize] } != b'#' {
            (x, y) = (check_x, check_y);
        } else {
            pointing += 1;
            pointing %= 4;
        }
    }
}

#[no_mangle]
pub fn run(i: &str) -> impl Display {
    follow(i.as_bytes()).popcnt()
}

#[no_mangle]
pub fn p2(i: &str) -> impl Display {
    let i = i.as_bytes();
    fn run(i: &[u8], obstacle: (i64, i64)) -> bool {
        let mut marked = [bitset16960::new(); 4];
        let grid = unsafe { i.as_chunks_unchecked::<{ SIZE + 1 }>() };
        let guard = memchr::memchr(b'^', i).ψ();
        let mut y = (guard / SIZE) as i64;
        let mut x = (guard % (SIZE + 1)) as i64;
        let mut pointing = 0u8;
        loop {
            let i = y as usize * SIZE + x as usize;
            if C! { &marked[pointing as usize] }.get(i) {
                return true;
            } else {
                C! { &mut marked[pointing as usize] }.set(i);
            }
            let (check_x, check_y) = mat!(pointing {
                0 => (x, y - 1),
                1 => (x + 1, y),
                2 => (x, y + 1),
                3 => (x - 1, y),
            });
            if (check_y >= SIZE as i64)
                || (check_y < 0)
                || (check_x >= SIZE as i64)
                || (check_x < 0)
            {
                return false;
            }
            if C! { grid[check_y as usize] [check_x as usize] } != b'#'
                && (check_x, check_y) != obstacle
            {
                (x, y) = (check_x, check_y);
            } else {
                pointing += 1;
                pointing %= 4;
            }
        }
    }
    let marked = follow(&i);
    use rayon::prelude::*;
    (0..SIZE)
        .flat_map(|x| (0..SIZE).map(move |y| (x, y)))
        .filter(|&(x, y)| marked.get(y * SIZE + x))
        .filter(|&(x, y)| i[y * (SIZE + 1) + x] == b'.')
        .par_bridge()
        .filter(|&(x, y)| run(&i, (x as i64, y as i64)))
        .count()
}

fn main() {
    // let mut s = String::new();
    // for i in 0..1280 {
    let i = include_str!("inp.txt");
    //     s.push_str(i);
    // }
    // std::fs::write("src/inp.txt", s);
    println!("{}", run(i));
    println!("{}", p2(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| run(i));
}
