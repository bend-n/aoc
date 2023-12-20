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

pub use util::prelude::*;

#[derive(Eq, Debug, PartialEq)]
enum ModuleT<'a> {
    Flip(bool),
    Cj(HashMap<&'a [u8], bool>),
    Untyped,
}

struct Module<'a> {
    ty: ModuleT<'a>,
    output: Box<[&'a [u8]]>,
}

impl<'a> Module<'a> {
    pub fn pass<'b>(
        &mut self,
        myname: &'a [u8],
        from: &[u8],
        x: bool,
        stack: &'b mut VecDeque<(&'a [u8], &'a [u8], bool)>,
    ) {
        match self.ty {
            ModuleT::Flip(ref mut state) => {
                if x {
                    return;
                }
                *state = !*state;
                for o in &*self.output {
                    stack.push_back((myname, o, *state));
                }
            }
            ModuleT::Cj(ref mut mem) => {
                *mem.get_mut(from).unwrap() = x;
                let s = !mem.values().all(|&x| x);
                for o in &*self.output {
                    stack.push_back((myname, o, s));
                }
            }
            ModuleT::Untyped => {
                for x in &*self.output {
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

fn parse<'a>(
    i: impl Iterator<Item = (&'a [u8], impl Iterator<Item = &'a [u8]>)>,
) -> HashMap<&'a [u8], Module<'a>> {
    let mut modules = HashMap::new();
    let mut rest = vec![];
    i.map(|(mut from, to)| {
        let to: Box<_> = to.collect();
        match from[0] {
            b'%' => {
                from.skip(1);
                modules.insert(
                    from,
                    Module {
                        ty: ModuleT::Flip(false),
                        output: to,
                    },
                );
            }
            b'&' => {
                from.skip(1);
                rest.push((from, to));
                return;
            }
            _ => {
                modules.insert(
                    from,
                    Module {
                        ty: ModuleT::Untyped,
                        output: to,
                    },
                );
            }
        };
    })
    .Θ();
    let r = rest.clone();
    for (name, output) in rest {
        let mut inps: HashMap<&[u8], bool> = modules
            .iter()
            .filter(|(_, x)| x.output.contains(&name))
            .map(|(x, _)| (*x, false))
            .collect();
        inps.extend(
            r.iter()
                .filter(|(x, _)| *x != name)
                .filter(|(_, x)| x.contains(&name))
                .map(|(x, _)| (*x, false)),
        );
        modules.insert(
            name,
            Module {
                ty: ModuleT::Cj(inps),
                output,
            },
        );
    }
    modules
}

fn p1(i: &str) -> usize {
    let mut modules = parse(split(i.行()));
    fn push(modules: &mut HashMap<&[u8], Module<'_>>) -> (usize, usize) {
        let (mut lo, mut hi) = (0, 0);
        let mut stack = VecDeque::new();
        stack.push_back((&b"upstairs"[..], &b"broadcaster"[..], false));
        while let Some((m, to, x)) = stack.pop_front() {
            if x {
                hi += 1
            } else {
                lo += 1;
            }
            if let Some(o) = modules.get_mut(to) {
                o.pass(to, m, x, &mut stack)
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
        (&b"xp"[..], None::<u64>),
        (&b"fc"[..], None),
        (&b"dd"[..], None),
        (&b"fh"[..], None),
    ]);

    let mut lens = vec![];
    for when in 0.. {
        let mut stack = VecDeque::new();
        stack.push_back((&b"upstairs"[..], &b"broadcaster"[..], false));
        while let Some((m, to, x)) = stack.pop_front() {
            if !x && let Some(x) = from.get_mut(to) {
                if let Some(y) = x {
                    lens.push(when - *y);
                    from.remove(to);
                    if from.len() == 0 {
                        return lens.iter().product();
                    }
                } else {
                    *x = Some(when);
                }
            }
            if let Some(o) = modules.get_mut(to) {
                o.pass(to, m, x, &mut stack)
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
