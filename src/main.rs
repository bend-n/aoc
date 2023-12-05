#![feature(array_windows, test, slice_as_chunks, array_chunks)]
extern crate test;
pub mod prelude {
    pub use itertools::izip;
    pub use itertools::Itertools;
    pub use std::{
        collections::{HashMap, HashSet, VecDeque},
        fmt::{Debug, Display},
        hint::black_box as boxd,
        iter,
        ops::Range,
    };
}
pub use prelude::*;
use rayon::iter::{ParallelBridge, ParallelIterator};

const S: char = ' ';

fn solve(i: &str) -> impl Display {
    let mut lines = i.lines();
    let seeds: Box<[u64]> = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    lines.next().unwrap();
    macro_rules! section {
        ($of:ident) => {
            let mut $of = Vec::with_capacity(10);
            for line in lines.by_ref().skip(1) {
                if line == "" {
                    println!("finished section {}", stringify!($of));
                    break;
                }
                $of.push(
                    line.split_ascii_whitespace()
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect_tuple()
                        .map(|(s, d, r)| (s..s + r, d..d + r))
                        .unwrap(),
                );
            }
        };
    }
    section!(seed2soil);
    section!(soil2fertilizer);
    section!(fertilizer2water);
    section!(water2light);
    section!(light2temperature);
    section!(temperature2humidity);
    section!(humidity2location);
    fn index(v: &[(Range<u64>, Range<u64>)], get: u64) -> u64 {
        for (a, b) in v.iter().cloned() {
            for (a, b) in izip!(a, b) {
                if b == get {
                    return a;
                }
            }
        }
        get
    }
    seeds
        .array_chunks::<2>()
        .par_bridge()
        .flat_map(|&[s, r]| s..s + r)
        .map(|seed| {
            // print!("seed {seed} ");
            let soil = index(&seed2soil, seed);
            // print!("soil {soil} ");
            let fertilizer = index(&soil2fertilizer, soil);
            // print!("fertilizer {fertilizer} ");
            let water = index(&fertilizer2water, fertilizer);
            // print!("water {water} ");
            let light = index(&water2light, water);
            // print!("light {light} ");
            let temperature = index(&light2temperature, light);
            // print!("temp {temperature} ");
            let humidity = index(&temperature2humidity, temperature);
            // print!("humidity {humidity} ");
            let location = index(&humidity2location, humidity);
            // println!("location {location}");
            location
        })
        .min()
        .unwrap()

    // lines
    //     .map(|x| {
    //         //
    //     })
    //     .sum::<u64>()
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
