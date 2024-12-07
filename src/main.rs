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
struct bitset17030([u64; 267]);
impl bitset17030 {
    const fn new() -> Self {
        Self([0; 267])
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
// struct bitset17030(bits);
// impl bitset17030 {
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

fn follow(i: &[u8]) -> (i64, bitset17030) {
    let mut marked = bitset17030::new();
    let mut position = memchr::memchr(b'^', i).ψ() as i64;
    let guard = position;
    let mut pointing = 0u8;

    'outer: loop {
        match pointing {
            0 => loop {
                marked.set(position as usize);
                let check = position - SIZE as i64 - 1;
                if check < 0 {
                    break 'outer;
                }
                match C! { i[check as usize] } {
                    b'\n' => break 'outer,
                    b'#' => break,
                    _ => position = check,
                };
            },
            1 => loop {
                marked.set(position as usize);
                let check = position + 1;
                match C! { i[check as usize] } {
                    b'\n' => break 'outer,
                    b'#' => break,
                    _ => position = check,
                };
            },
            2 => loop {
                marked.set(position as usize);
                let check = position + SIZE as i64 + 1;
                match C! { i[check as usize] } {
                    b'\n' => break 'outer,
                    b'#' => break,
                    _ => position = check,
                };
            },
            3 => loop {
                marked.set(position as usize);
                let check = position - 1;
                if check < 0 {
                    break 'outer;
                }
                match C! { i[check as usize] } {
                    b'\n' => break 'outer,
                    b'#' => break,
                    _ => position = check,
                };
            },
            _ => shucks!(),
        }
        pointing += 1;
        pointing %= 4;
    }
    (guard, marked)
}

#[no_mangle]
pub fn run(i: &str) -> impl Display {
    follow(i.as_bytes()).1.popcnt()
}

#[no_mangle]
pub fn p2(i: &str) -> impl Display {
    let i = i.as_bytes();
    fn run(i: &[u8], guard: i64, obstacle: i64) -> bool {
        let mut marked = [bitset17030::new(); 4];
        let mut position = guard;
        let mut pointing = 0u8;

        macro_rules! check {
            () => {
                if marked[pointing as usize].get(position as usize) {
                    return true;
                } else {
                    marked[pointing as usize].set(position as usize)
                }
            };
        }
        macro_rules! entry {
            (sub $check:expr) => {
                loop {
                    check!();
                    let check = $check;
                    if check < 0 {
                        return false;
                    }
                    if check == obstacle {
                        break;
                    }
                    match C! { i[check as usize] } {
                        b'\n' => return false,
                        b'#' => break,
                        _ => position = check,
                    };
                }
            };
            (add $check:expr) => {
                loop {
                    check!();
                    let check = $check;
                    if check == obstacle {
                        break;
                    }
                    match i.get(check as usize) {
                        None | Some(b'\n') => return false,
                        Some(b'#') => break,
                        _ => position = check,
                    };
                }
            };
        }

        loop {
            match pointing {
                0 => entry!(sub position - SIZE as i64 - 1),
                1 => entry!(add position + 1),
                2 => entry!(add position + SIZE as i64 + 1),
                3 => entry!(sub position - 1),
                _ => shucks!(),
            }
            pointing += 1;
            pointing %= 4;
        }
    }
    let (guard, marked) = follow(&i);
    use rayon::prelude::*;
    (0..SIZE * (SIZE + 1))
        .filter(|&i| marked.get(i))
        .filter(|&j| C! { i[j] } == b'.')
        .par_bridge()
        .filter(|&j| run(&i, guard, j as i64))
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
