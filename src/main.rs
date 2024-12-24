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
#[derive(Debug)]
struct Gate {
    op: fn(u8, u8) -> u8,
    inputs: [usize; 2],
    out: usize,
    run: bool,
}

#[no_mangle]
pub fn p1(x: &str) -> impl Display {
    static mut wires: [u8; 15547] = [u8::MAX; 15547];
    let mut gates = Vec::with_capacity(128);
    fn h(gate: [u8; 3]) -> usize {
        // println!("{}", gate.p());
        if gate[1].is_ascii_digit() {
            let [_, b, c] = gate.map(|x| (x - b'0') as usize);
            15547 + b * 10 + c
        } else {
            let [a, b, c] = gate.map(|x| (x - b'a') as usize);
            a * 26 * 26 + b * 26 + c
        }
    }
    let mut i_ = x.行();
    let mut i = x.as_ptr();
    let mut x = 0;
    let mut y = 0;
    for j in 0..45u64 {
        x |= (unsafe { *i.add(5) - b'0' } as u64) << j;
        unsafe { i = i.add(7) };
    }
    for j in 0..45u64 {
        y |= (unsafe { *i.add(5) - b'0' } as u64) << j;
        unsafe { i = i.add(7) };
    }
    let mut i = i_;
    i.by_ref().take_while(|x| !x.is_empty()).for_each(drop);
    fn gate(x: &[u8]) -> [u8; 3] {
        x.try_into().unwrap()
    }
    fn and(a: u8, b: u8) -> u8 {
        a & b
    }
    fn or(a: u8, b: u8) -> u8 {
        a | b
    }
    fn xor(a: u8, b: u8) -> u8 {
        a ^ b
    }
    let mut z = 0;
    for connection in i {
        let mut i = connection.split(|x| *x == b' ');
        let [i1, op, i2, _, out] = std::array::from_fn(|_| i.Δ());
        let [i1, i2, out] = [i1, i2, out].map(gate);
        let op = match op {
            b"AND" => and,
            b"OR" => or,
            b"XOR" => xor,
            _ => unreachable!(),
        };
        if i1[0] == b'y' || i1[0] == b'x' {
            let index = (i1[1] - b'0') * 10 + (i1[2] - b'0');
            let res = op((y & 1 << index != 0) as u8, (x & 1 << index != 0) as u8);
            if out[0] != b'z' {
                unsafe { wires[h(out)] = res };
            } else {
                z |= (res as u64) << index;
            }
        } else {
            gates.push(Gate {
                op,
                run: false,
                inputs: [h(i1), h(i2)],
                out: h(out),
            });
        }
    }
    let mut all_run = false;
    while !all_run {
        all_run = true;
        for gate in &mut gates {
            if gate.run {
                continue;
            };
            unsafe {
                let [a, b] = gate.inputs;
                let a = wires[a];
                if a != u8::MAX
                    && let b = wires[b]
                    && b != u8::MAX
                {
                    gate.run = true;
                    if gate.out > 15547 {
                        let index = gate.out - 15547;
                        z |= ((gate.op)(a, b) as u64) << index;
                    } else {
                        wires[gate.out] = (gate.op)(a, b);
                    }
                }
            }
            all_run &= gate.run;
        }
    }
    z
}

fn main() {
    let s = include_str!("inp.txt");
    println!("{}", unsafe { p1(s) });

    // dbg!(exec(&program, regi));
}
#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
