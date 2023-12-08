#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(array_windows, test, slice_as_chunks, array_chunks)]
extern crate test;
mod util;
pub use util::prelude::*;

#[inline(always)]
fn k(s: &str) -> u16 {
    unsafe { *s.as_ptr().cast::<[u8; 3]>() }
        .iter()
        .enumerate()
        .map(|(i, &b)| ((b - b'A') as u16) << (i * 5))
        .sum()
}

fn start(x: u16) -> bool {
    (x >> (2 * 5) & 0b11111) == 0u16
}

fn end(x: u16) -> bool {
    (x >> (2 * 5) & 0b11111) == (b'Z' - b'A') as u16
}

pub fn run(i: &str) -> impl Display {
    let mut lines = i.lines();
    let line = lines.by_ref().Δ().as_bytes();
    let map = lines
        .skip(1)
        .map(|x| {
            x.μ('=')
                .mr(|x| x.trim()[1..x.len() - 2].μ(',').mb(str::trim).mb(k))
                .ml(str::trim)
                .ml(k)
        })
        .collect::<HashMap<_, _>>();
    let mut positions = map
        .keys()
        .map(|&x| x)
        .filter(|&x| start(x))
        .collect::<Box<[_]>>();
    let mut steps = 1u64;
    let mut findings = HashMap::new();
    let mut cycle = HashMap::new();
    for &instruction in line.iter().cycle() {
        if cycle.len() >= positions.len() {
            break;
        }
        for p in &mut *positions {
            let at = map[p];
            *p = match instruction {
                b'L' => at.0,
                b'R' => at.1,
                _ => dang!(),
            };
            if end(*p) {
                if let Some(&c) = findings.get(p) {
                    match cycle.entry(*p) {
                        Entry::Occupied(_) => {}
                        Entry::Vacant(x) => {
                            x.insert(steps - c);
                        }
                    }
                } else {
                    findings.insert(*p, steps);
                }
            }
        }
        steps += 1;
    }
    lcm(cycle.values().copied())
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
