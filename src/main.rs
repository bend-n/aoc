#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(
    array_windows,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    byte_slice_trim_ascii
)]
extern crate test;
mod util;
pub use util::prelude::*;

#[inline(always)]
fn k(s: &[u8]) -> u16 {
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

pub fn run(mut i: &str) -> impl Display {
    let line = i.take_line().unwrap();
    let map = i
        .as_bytes()
        .array_chunks::<17>()
        .map(|x| (k(&x[1..4]), (k(&x[8..11]), k(&x[13..16]))))
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
    let v = lcm(cycle.values().copied());
    leek!(cycle findings positions);
    v
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
