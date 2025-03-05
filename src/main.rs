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
    stdarch_x86_avx512,
    iter_partition_in_place,
    slice_swap_unchecked,
    generic_const_exprs,
    ptr_sub_ptr,
    iter_array_chunks,
    slice_from_ptr_range,
    if_let_guard,
    maybe_uninit_uninit_array,
    once_cell_get_mut,
    iter_collect_into,
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
    import_trait_associated_functions,
    core_intrinsics
)]
extern crate test;
pub mod util;

use atools::prelude::*;
pub use util::prelude::*;

#[derive(Debug)]
struct Gate<'s> {
    inp: GateTy<'s>,
    run: bool,
}
impl<'s> Gate<'s> {
    pub fn new(x: GateTy<'s>) -> Self {
        Gate { inp: x, run: false }
    }
}
#[derive(Debug)]
enum GateTy<'s> {
    Unary(fn(u16) -> u16, &'s str, &'s str),
    Binary(fn(u16, u16) -> u16, [&'s str; 2], &'s str),
    With(fn(u16, u16) -> u16, u16, &'s str, &'s str),
}
#[no_mangle]
pub fn p1(x: &str) -> impl Display {
    let mut wires = HashMap::default();
    let mut gates = Vec::with_capacity(128);
    let mut ending = None;
    for connection in x.行() {
        if connection.starts_with(b"NOT") {
            let [_, x, _, out] = connection.str().split(' ').carr();
            gates.push(Gate {
                inp: GateTy::Unary(|x| !x, x, out),
                run: false,
            });
            continue;
        }
        let [a, op, out] = connection.μₙ(b' ').carr::<3>();

        if op == b"->" {
            if let Ok(x) = a.str().parse::<u16>() {
                wires.insert(out.str(), x);
            } else {
                ending = Some(a.str());
            }
            continue;
        }
        let [a, _, b, _, out] = connection.μₙ(b' ').carr();
        if let Ok(x) = a.str().parse::<u16>() {
            gates.push(Gate {
                inp: GateTy::With(|a, b| a & b, x, b.str(), out.str()),
                run: false,
            });
            continue;
        }

        let [a, op, b, _, out] = connection.str().split(' ').carr();
        gates.push(match op {
            "AND" => Gate::new(GateTy::Binary(|a, b| a & b, [a, b], out)),
            "OR" => Gate::new(GateTy::Binary(|a, b| a | b, [a, b], out)),
            "LSHIFT" => Gate::new(GateTy::With(|a, b| a << b, b.λ(), a, out)),
            "RSHIFT" => Gate::new(GateTy::With(|a, b| a >> b, b.λ(), a, out)),
            x => panic!("{}", x),
        });
    }
    // wires.insert("b", 956);
    let mut all_run = false;
    while !all_run {
        all_run = true;
        for gate in &mut gates {
            if gate.run {
                continue;
            };
            match gate.inp {
                GateTy::Unary(op, input, output) => wires.get(input).copied().map(|x| {
                    gate.run = true;
                    wires.insert(output, op(x))
                }),
                GateTy::Binary(op, [a, b], output) => wires
                    .get(a)
                    .copied()
                    .zip(wires.get(b).copied())
                    .map(|(a, b)| {
                        gate.run = true;
                        wires.insert(output, op(a, b))
                    }),
                GateTy::With(op, with, input, output) => wires.get(input).copied().map(|x| {
                    gate.run = true;
                    wires.insert(output, op(x, with))
                }),
            };
            all_run &= gate.run;
        }
    }
    wires[ending.unwrap()]
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
