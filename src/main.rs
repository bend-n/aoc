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
use atools::ArrayTools;
pub use util::prelude::*;
#[no_mangle]
pub fn run(x: &str) -> impl Display {
    let g = Graph::load(x);
    let mut x = g.mxq().into_iter().map(|x| NAMES[x]);
    let a: [_; 13] = std::array::from_fn(|_| x.Δ());
    let c = b',';
    let mut out = [
        0, 0, c, 0, 0, c, 0, 0, c, 0, 0, c, 0, 0, c, 0, 0, c, 0, 0, c, 0, 0, c, 0, 0, c, 0, 0, c,
        0, 0, c, 0, 0, c, 0, 0,
    ];
    for (elem, i) in a.into_iter().ι::<usize>() {
        out[i + i * 2..i + i * 2 + 2].copy_from_slice(&elem);
    }
    unsafe { String::from_utf8_unchecked(out.to_vec()) }
}

fn main() {
    let s = include_str!("inp.txt");
    println!("{}", unsafe { run(s) });
    // dbg!(exec(&program, regi));
}
#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { run(i) });
}

struct Graph {
    // vert: [[u8; 2]; SIZE],
    adj: Box<[[u64; WORDS]; SIZE]>,
}
const SIZE: usize = 676;
const WORDS: usize = (SIZE + 63) / 64;
fn h([a, b]: [u8; 2]) -> usize {
    a as usize + 26 * b as usize
}
const NAMES: [[u8; 2]; 676] = include!("../lut2");

impl Graph {
    fn load(content: &str) -> Self {
        const INDEX: [u16; 3295] = {
            let mut l = [0; 3295];
            include!("../lut");
            l
        };
        let mut i = content.as_ptr();
        let mut adj = Box::new([[0u64; WORDS]; SIZE]);
        for _ in 0..3380 {
            unsafe {
                let a = *(i as *const [u8; 2]);
                let b = *(i.add(3) as *const [u8; 2]);
                let ha = h(a);
                let hb = h(b);
                i = i.add(6);
                let i = INDEX[ha] as usize;
                let j = INDEX[hb] as usize;
                *adj.get_unchecked_mut(i).get_unchecked_mut(j / 64) |= 1u64 << (j % 64);
                *adj.get_unchecked_mut(j).get_unchecked_mut(i / 64) |= 1u64 << (i % 64);
            }
        }
        Graph { adj }
    }

    fn print_mat(&self, x: [u64; WORDS], l: [u8; 2]) {
        let n = Self::adj_on(x);
        print!("{}: ", l.p());
        for neighbor in n {
            print!("{} ", NAMES[neighbor].p());
        }
        println!();
    }

    fn adj_on(x: [u64; WORDS]) -> Vec<usize> {
        let mut n = Vec::with_capacity(13);
        for j in 0..WORDS {
            let mut x = x[j];
            while x != 0 {
                let bit = x.trailing_zeros();
                n.push(64 * j + bit as usize);
                x &= !(1 << bit);
            }
        }
        n
    }

    fn mxq(&self) -> Vec<usize> {
        'out: for computer in 0..SIZE {
            let mut neighbors = self.adj[computer];
            if neighbors == [0; 11] {
                continue;
            }
            neighbors[computer / 64] |= 1 << (computer % 64);
            let mut missing = 0;
            for j in 0..WORDS {
                let mut x = neighbors[j];
                while x != 0 {
                    let bit = x.trailing_zeros();
                    // node ∩ neighbors
                    let inter = (0..WORDS)
                        .map(|i| (self.adj[64 * j + bit as usize][i] & neighbors[i]).count_ones())
                        .sum::<u32>();
                    if inter < 12 {
                        if missing > 1 {
                            continue 'out;
                        }
                        missing += 1;
                        neighbors[j] &= !(1 << bit);
                    }
                    x &= !(1 << bit);
                }
            }
            return Self::adj_on(neighbors);
        }
        panic!()
    }
}
