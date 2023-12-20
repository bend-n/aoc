#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(
    inline_const,
    slice_flatten,
    iter_collect_into,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    unchecked_math,
    array_windows,
    slice_take,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    byte_slice_trim_ascii
)]
extern crate test;
pub mod util;

use std::mem::MaybeUninit;

use arrayvec::ArrayVec;
pub use util::prelude::*;

#[derive(Eq, Debug, PartialEq)]
enum ModuleT {
    Flip(bool),
    Cj(ArrayVec<(u16, bool), 11>),
    Untyped,
}

#[derive(Debug)]
struct Module {
    ty: ModuleT,
    output: Box<[u16]>,
}

impl Module {
    pub fn pass<'b>(
        &mut self,
        myname: u16,
        from: u16,
        x: bool,
        stack: &'b mut VecDeque<(u16, u16, bool)>,
    ) {
        match self.ty {
            ModuleT::Flip(ref mut state) => {
                if x {
                    return;
                }
                *state = !*state;
                for &o in &*self.output {
                    stack.push_back((myname, o, *state));
                }
            }
            ModuleT::Cj(ref mut mem) => {
                mem.iter_mut().find(|(x, _)| x == &from).unwrap().1 = x;
                let s = !mem.iter().all(|&(_, x)| x);
                for &o in &*self.output {
                    stack.push_back((myname, o, s));
                }
            }
            ModuleT::Untyped => {
                for &x in &*self.output {
                    stack.push_back((myname, x, false));
                }
            }
        }
    }
}

fn split<'a>(
    i: impl Iterator<Item = &'a [u8]>,
) -> impl Iterator<Item = (&'a [u8], impl Iterator<Item = &'a [u8]>)> {
    i.map(|mut x| {
        let n = x.iter().position(|&x| x == b' ').unwrap();
        let a = &x[..n];
        x.skip(n + 4);
        (a, x.split(|&x| x == b',').map(<[u8]>::trim_ascii))
    })
}

impl<'a, T> std::ops::IndexMut<u16> for Hold<T> {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        unsafe { self.names.get_unchecked_mut(index.nat()).assume_init_mut() }
    }
}

impl<'a, T> std::ops::Index<u16> for Hold<T> {
    type Output = T;
    fn index(&self, index: u16) -> &Self::Output {
        unsafe { self.names.get_unchecked(index.nat()).assume_init_ref() }
    }
}

struct Hold<T> {
    names: [MaybeUninit<T>; 26 * 26],
}

impl<'a, T> std::ops::IndexMut<&[u8]> for Hold<T> {
    fn index_mut(&mut self, index: &[u8]) -> &mut Self::Output {
        unsafe {
            self.names
                .get_unchecked_mut(hash(index).nat())
                .assume_init_mut()
        }
    }
}

impl<'a, T> std::ops::Index<&[u8]> for Hold<T> {
    type Output = T;

    fn index(&self, index: &[u8]) -> &Self::Output {
        unsafe {
            self.names
                .get_unchecked(hash(index).nat())
                .assume_init_ref()
        }
    }
}

fn hash(x: &[u8]) -> u16 {
    if let &[a, b] = x {
        (a - b'a').widen() * 26 + (b - b'a').widen()
    } else {
        0
    }
}

impl<T> Hold<T> {
    #[inline]
    fn new() -> Self {
        Self {
            names: [const { MaybeUninit::uninit() }; 26 * 26],
        }
    }

    fn set(&mut self, index: u16, to: T) {
        unsafe { self.names.get_unchecked_mut(index.nat()).write(to) };
    }
}

fn parse<'a>(i: impl Iterator<Item = (&'a [u8], impl Iterator<Item = &'a [u8]>)>) -> Hold<Module> {
    let mut modules = Hold::new();
    let mut backwards = [const { vec![] }; 26 * 26];
    let mut rest = vec![];
    i.map(|(mut from, to)| {
        let to: Box<[u16]> = to.map(hash).collect();
        let t = match C! { from[0] } {
            b'%' => {
                from.skip(1);
                Some(ModuleT::Flip(false))
            }
            b'&' => {
                from.skip(1);
                None
            }
            _ => Some(ModuleT::Untyped),
        };
        let f = hash(from);
        for &elem in &*to {
            let x = C! { &mut backwards[elem.nat()]};
            if !x.contains(&f) {
                x.push(f);
            };
        }

        if let Some(ty) = t {
            modules.set(f, Module { ty, output: to });
        } else {
            rest.push((f, to));
        }
    })
    .Θ();
    for (name, output) in rest {
        modules.set(
            name,
            Module {
                ty: ModuleT::Cj(
                    C! { backwards[name.nat()] }
                        .iter()
                        .map(|&x| (x, false))
                        .collect(),
                ),
                output,
            },
        );
    }
    modules
}

fn p1(i: &str) -> usize {
    let mut modules = parse(split(i.行()));
    fn push(modules: &mut Hold<Module>) -> (usize, usize) {
        let (mut lo, mut hi) = (0, 0);
        let mut stack = VecDeque::new();
        stack.push_back((0, 0, false));
        while let Some((m, to, x)) = stack.pop_front() {
            if x {
                hi += 1
            } else {
                lo += 1;
            }
            if to != hash(b"rx") {
                modules[to].pass(to, m, x, &mut stack)
            };
        }
        (lo, hi)
    }

    let (lo, hi) = (0..1000).fold((0, 0), |(lo, hi), _| {
        let (lo2, hi2) = push(&mut modules);
        (lo + lo2, hi + hi2)
    });
    lo * hi
}

fn p2(i: &str) -> u64 {
    let mut modules = parse(split(i.行()));
    let mut from = HashMap::from([
        (hash(&b"xp"[..]), None::<u64>),
        (hash(&b"fc"[..]), None),
        (hash(&b"dd"[..]), None),
        (hash(&b"fh"[..]), None),
    ]);

    let mut lens = vec![];
    for when in 0.. {
        let mut stack = VecDeque::new();
        stack.push_back((0, 0, false));
        while let Some((m, to, x)) = stack.pop_front() {
            if !x && let Some(x) = from.get_mut(&to) {
                if let Some(y) = x {
                    lens.push(when - *y);
                    from.remove(&to);
                    if from.len() == 0 {
                        return lens.iter().product();
                    }
                } else {
                    *x = Some(when);
                }
            }
            if to != hash(b"rx") {
                modules[to].pass(to, m, x, &mut stack)
            };
        }
    }
    dang!()
}

pub fn run(i: &str) -> impl Display {
    p1(i)
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
