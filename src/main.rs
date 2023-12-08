#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(array_windows, test, slice_as_chunks, array_chunks)]
extern crate test;
mod util;
pub use explicit_cast::prelude::*;
pub use util::prelude::*;

fn solve(i: &str) -> impl Display {
    let mut lines = i.lines();
    let line = lines.by_ref().Δ().as_bytes();
    let map = lines
        .skip(1)
        .map(|x| {
            x.μ('=')
                .mr(|x| x.trim()[1..x.len() - 2].μ(',').ml(str::trim).mr(str::trim))
                .ml(str::trim)
        })
        .collect::<HashMap<_, _>>();
    let mut positions = map
        .keys()
        .map(|&x| x)
        .filter(|x| x.ends_with('A'))
        .collect::<Box<[_]>>();
    let mut steps = 1u64;
    let mut findings = HashMap::new();
    let mut cycle = HashMap::new();
    for &instruction in line.iter().cycle() {
        if cycle.len() >= positions.len() {
            break;
        }
        for p in &mut *positions {
            let at = map[*p];
            *p = match instruction {
                b'L' => at.0,
                b'R' => at.1,
                _ => dang!(),
            };
            if p.ends_with('Z') {
                if let Some(&c) = findings.get(*p) {
                    if !cycle.contains_key(*p) {
                        println!("cycle {} ({steps})", steps - c);
                        cycle.insert(*p, steps - c);
                    }
                } else {
                    println!("register {p} ({steps})");
                    findings.insert(*p, steps);
                }
            }
        }
        steps += 1;
    }
    print!("lcm(");
    for cycle in cycle.values() {
        print!("{cycle},")
    }
    println!(")");
    0
}

fn main() {
    let i = include_str!("inp.txt").trim();
    println!("{}", solve(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| solve(i));
}
