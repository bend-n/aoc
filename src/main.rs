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

#[no_mangle]
pub fn p2(x: &str) -> impl Display {
    let mut gates = Vec::with_capacity(128);
    let x = &x.as_bytes()[7 * 45 * 2 + 1..];
    fn gate(x: &[u8]) -> [u8; 3] {
        x.try_into().unwrap()
    }
    const XOR: u8 = 0;
    const AND: u8 = 1;
    const OR: u8 = 2;
    for connection in x.行() {
        let mut i = connection.split(|x| *x == b' ');
        let [i1, op, i2, _, out] = std::array::from_fn(|_| i.Δ());
        let [i1, i2, out] = [i1, i2, out].map(gate);
        let op = match op {
            b"XOR" => XOR,
            b"AND" => AND,
            b"OR" => OR,
            _ => shucks!(),
        };
        gates.push((i1, i2, out, op));
    }
    let find = |i1, op: u8, i2| {
        gates
            .iter()
            .find(|x| {
                (x.0 == i1 && x.1 == i2 && x.3 == op) || (x.1 == i1 && x.0 == i2 && x.3 == op)
            })
            .map(|x| x.2)
    };
    let find_using = |op: u8, i2| {
        gates.iter().find_map(|x| {
            (x.1 == i2 && x.3 == op)
                .then_some((x.0, x.2))
                .or((x.0 == i2 && x.3 == op).then_some((x.1, x.2)))
        })
    };
    // search for full adders (this is a ripple carry adder)
    // x ^ y -> t0
    // x & y -> t1
    // c0 & t0 -> t2
    // c0 ^ t0 -> z
    // t1 | t2 -> c1
    let mut swaps = vec![];
    for [x, y, should_z] in const {
        let input: [[[u8; 3]; 3]; 44] = car::from_fn!(|i| {
            let i = i + 1;
            let x = [b'x', (i as u8 / 10) + b'0', (i as u8 % 10) + b'0'];
            let y = [b'y', (i as u8 / 10) + b'0', (i as u8 % 10) + b'0'];
            let z = [b'z', (i as u8 / 10) + b'0', (i as u8 % 10) + b'0'];
            [x, y, z]
        });
        input
    } {
        let t0 = find(x, XOR, y).unwrap(); // x ^ y -> t0

        // println!("{} ^ {} -> {} (t0)", x.p(), y.p(), t0.p());

        let t1 = find(x, AND, y).unwrap(); // x & y -> t1

        // println!("{} & {} -> {} (t1)", x.p(), y.p(), t1.p());

        // c0 & t0 -> t2
        let Some((_c0, _t2)) = find_using(AND, t0) else {
            swaps.push(t0);
            swaps.push(t1);
            // println!("swap {} with {}", t0.p(), t1.p());
            continue;
        };
        // println!("(c0) {} & {} (t0) -> {} (t2)", c0.p(), t0.p(), t2.p());
        let (_, z) = find_using(XOR, t0).unwrap(); // c0 ^ t0 -> z

        // println!("(c0) {} ^ {} (t0) -> {} (z)", c0.p(), t0.p(), z.p());
        if z != should_z {
            swaps.push(z);
            swaps.push(should_z);
            // println!("swap {} with {}", z.p(), format!("z{i:02}"));
            continue;
        }
        // t1 | t2 -> c1
        // let c1 = find(t1, b"OR", t2).unwrap();
    }
    swaps.sort_unstable();
    let mut out = String::with_capacity(40);
    for swap in swaps {
        use std::fmt::Write;
        _ = write!(out, "{},", swap.p());
    }
    out
}

fn main() {
    let s = include_str!("inp.txt");
    println!("{}", unsafe { p2(s) });

    // dbg!(exec(&program, regi));
}
#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
