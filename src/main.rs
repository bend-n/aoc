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
use atools::CollectArray;
use std::cmp::Reverse;
pub use util::prelude::*;

const SIZE: usize = 141;

fn exec(program: &[u8], [mut a, mut b, mut c]: [u128; 3]) -> Vec<u8> {
    let mut pointer = 0;
    let mut out = vec![];
    while let Some(opcode) = program.get(pointer) {
        let operand = program[pointer + 1];
        let combo = || match operand {
            0..=3 => operand as u128,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!(),
        };
        match opcode {
            0 => {
                a = a / (2u128.pow(combo() as u32));
                pointer += 2;
            }
            1 => {
                b ^= operand as u128;
                pointer += 2;
            }
            2 => {
                b = combo() & 7;
                pointer += 2;
            }
            3 => {
                if a == 0 {
                    pointer += 2;
                } else {
                    pointer = operand.into();
                }
            }
            4 => {
                b ^= c;
                pointer += 2;
            }
            5 => {
                out.push((combo() & 7) as u8);
                pointer += 2;
            }
            6 => {
                b = a / (1 << combo() as u32);
                pointer += 2;
            }
            7 => {
                c = a / (1 << combo() as u32);
                pointer += 2;
            }
            _ => unreachable!(),
        }
        // println!("{a} {b} {c}");
    }
    out
}

#[no_mangle]
pub unsafe fn run(i: &str) -> impl Display {
    let i = i.as_bytes();
    let mut i = i.行();
    let regi = i
        .by_ref()
        .take_while(|x| !x.is_empty())
        .map(|x| x.μ1(':').trim_ascii_start().λ::<u128>())
        .carr();
    let program = i
        .nth(1)
        .ψ()
        .μ1(':')
        .trim_ascii_start()
        .split(|x| *x == b',')
        .map(|x| x.λ::<u8>())
        .collect_vec();
    exec(&program, regi).into_iter().join(",")
}

fn main() {
    let i = include_str!("inp.txt");
    let mut i = i.行();
    let regi: [u128; 3] = i
        .by_ref()
        .take_while(|x| !x.is_empty())
        .map(|x| x.μ1(':').trim_ascii_start().λ::<u128>())
        .carr();
    let program = i
        .nth(1)
        .ψ()
        .μ1(':')
        .trim_ascii_start()
        .split(|x| *x == b',')
        .map(|x| x.λ::<u8>())
        .collect_vec();
    println!(
        r#"
from z3 import *
solver = Optimize()
s = BitVec("s", 64)
a = s
b, c = (0, 0)"#
    );
    println!("for x in {program:?}:");
    for &[opcode, operand] in program.array_chunks::<2>() {
        let combo = || match operand {
            0..=3 => format!("{operand}"),
            4 => "a".to_string(),
            5 => "b".to_string(),
            6 => "c".to_string(),
            _ => panic!(),
        };
        match opcode {
            0 => println!("  a >>= {}", combo()),
            1 => println!("  b ^= {operand}"),
            2 => println!("  b = {} & 7", combo()),
            3 => {} // println!("  a == 0 ? {operand}"),
            4 => println!("  b ^= c"),
            5 => println!("  solver.add({} % 8 == x)", combo()),
            6 => println!("  b = a >> {}", combo()),
            7 => println!("  c = a >> {}", combo()),
            _ => unreachable!(),
        }
    }
    println!(
        r#"
solver.add(a == 0)
solver.minimize(s)
assert str(solver.check()) == "sat"
print(solver.model().eval(s))"#
    );
    assert_eq!(
        exec(&[0, 1, 5, 4, 3, 0], [729, 0, 0]),
        [4, 6, 3, 5, 6, 3, 5, 2, 1, 0]
    );
    assert_eq!(exec(&[5, 0, 5, 1, 5, 4], [10, 0, 0]), [0, 1, 2]);
    assert_eq!(
        exec(&[0, 1, 5, 4, 3, 0], [2024, 0, 0],),
        [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]
    );
    // dbg!(exec(&program, regi));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe {
        exec(
            boxd(&[2, 4, 1, 7, 7, 5, 0, 3, 1, 7, 4, 1, 5, 5, 3, 0]),
            [64012472, 0, 0],
        )
    });
}
