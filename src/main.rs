#![allow(
    unexpected_cfgs,
    confusable_idents,
    uncommon_codepoints,
    non_upper_case_globals,
    internal_features,
    mixed_script_confusables,
    static_mut_refs,
    incomplete_features,
    unused_imports,
    unsafe_op_in_unsafe_fn,
    redundant_semicolons
)]
#![feature(
    iter_from_coroutine,
    iterator_try_reduce,
    step_trait,
    cmp_minmax,
    custom_inner_attributes,
    pattern_types,
    pattern_type_macro,
    extend_one,
    slice_as_array,
    impl_trait_in_bindings,
    coroutines,
    stmt_expr_attributes,
    pattern_type_range_trait,
    const_trait_impl,
    coroutine_trait,
    iter_partition_in_place,
    slice_swap_unchecked,
    generic_const_exprs,
    iter_array_chunks,
    slice_from_ptr_range,
    if_let_guard,
    once_cell_get_mut,
    iter_collect_into,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    vec_into_raw_parts,
    try_blocks,
    portable_simd,
    test,
    slice_split_once,
    import_trait_associated_functions,
    core_intrinsics
)]
extern crate test;
pub mod util;

pub use atools::prelude::*;
use atools::{CollectArray, prelude::*};
use itertools::chain;
use lower::apply;
use md5::{Digest, Md5};
use memchr::memmem;
use regex::bytes::Regex;
use rustc_hash::FxBuildHasher;
use std::{
    cmp::{Reverse, minmax},
    hash::Hash,
    mem::take,
    ops::{Coroutine, Deref},
    pin::Pin,
    simd::prelude::*,
};
use swizzle::array;
pub use util::prelude::*;

#[allow(warnings)]
type u32x3 = Simd<u32, 3>;

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.name.get(), self.kind)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]

struct Item {
    name: identifier,
    kind: ItemK,
}
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum identifier {
    u1,
    u2,
    u3,
    u4,
    u5,
    u6,
    u7,
    u8,
    u9,
}
impl identifier {
    fn get(self) -> u8 {
        self as u8
    }
}
impl PartialEq for identifier {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}
impl Eq for identifier {}
impl PartialOrd for identifier {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get().partial_cmp(&other.get())
    }
}
impl Ord for identifier {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get().cmp(&other.get())
    }
}
impl Hash for identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get().hash(state)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum ItemK {
    Chip,
    Gen,
}
use ItemK::*;
#[unsafe(no_mangle)]
#[implicit_fn::implicit_fn]
pub unsafe fn p1(x: &'static [u8; ISIZE]) -> impl Debug {
    let re = Regex::new(r"([0-9a-z]+) generator").unwrap();
    let rechip = Regex::new(r"([0-9a-z]+)-compatible microchip").unwrap();
    let mut floors = vec![vec![]; 4];
    x.行().zip(&mut floors).for_each(|(x, to)| {
        to.extend(re.captures_iter(x).map(|x| Item {
            name: transmute(x.get(1).unwrap().as_bytes()[0] - b'0'),
            kind: Gen,
        }));
        to.extend(rechip.captures_iter(x).map(|x| Item {
            name: transmute(x.get(1).unwrap().as_bytes()[0] - b'0'),
            kind: Chip,
        }));
    });
    // if a chip is ever left in the same area as another RTG, and it's not connected to its own RTG, the chip will be fried.
    fn not_fried(floor: impl Iterator<Item = Item> + Clone) -> bool {
        floor.clone().all(_.kind == Chip)
            || floor
                .clone()
                .filter(_.kind == Chip) // every chip
                .all(|x| floor.clone().filter(_.kind == Gen).any(_.name == x.name)) // is connected
    };

    util::steps_astar(
        (0, floors),
        |(e, floor)| {
            [(e < 3).then(|| e + 1), (e > 0).then(|| e - 1)]
                .into_iter()
                .flatten()
                .flat_map(move |v| {
                    floor[e]
                        .clone()
                        .into_iter()
                        .ι()
                        .tuple_combinations()
                        .map(|(a, b)| vec![a, b])
                        .chain(floor[e].clone().into_iter().ι().map(|a| vec![a]))
                        .map({
                            let floor = floor.clone();
                            move |mut x| {
                                let mut floor = floor.clone();
                                floor[v].extend(x.iter().map(|x| x.0));
                                x.sort_by_key(|x| x.1);
                                x.iter().rev().map(|(_, i)| floor[e].swap_remove(*i)).θ();
                                floor[v].sort();
                                floor[e].sort();
                                (v, floor)
                            }
                        })
                })
                .filter(|(e, floor)| not_fried(floor[*e].iter().copied()))
                .map(|n| (n, 1))
        },
        |x| -(x.1[3].len() as i16),
        |(_, f)| f[0..3].iter().all(_.is_empty()),
    )
}
const ISIZE: usize = include_bytes!("inp.txt").len();
fn main() {
    unsafe { println!("{:?}", p1(include_bytes!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_bytes!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
