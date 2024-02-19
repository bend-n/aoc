#![allow(
    confusable_idents,
    uncommon_codepoints,
    internal_features,
    mixed_script_confusables,
    incomplete_features
)]
#![feature(
    slice_swap_unchecked,
    generic_const_exprs,
    maybe_uninit_uninit_array,
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
    core_intrinsics,
    byte_slice_trim_ascii
)]
extern crate test;
pub mod util;
pub use util::prelude::*;

pub struct Graph {
    pub v: usize,
    pub edges: Vec<(usize, usize)>,
}

pub struct UnionFind {
    p: Vec<usize>,
    s: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            s: vec![1; size],
            p: (0..size).collect(),
        }
    }

    fn reset(&mut self) {
        self.s.fill(1);
        self.p
            .iter_mut()
            .enumerate()
            .for_each(|(idx, val)| *val = idx);
    }

    pub fn find(&mut self, key: usize) -> usize {
        if self.p[key] == key {
            return key;
        }
        let parent = self.find(self.p[key]);
        self.p[key] = parent;
        parent
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let α = self.find(a);
        let β = self.find(b);
        if α == β {
            return false;
        }
        let a = self.s[α];
        let b = self.s[β];
        if a >= b {
            self.s[α] += b;
            self.p[β] = α;
        } else {
            self.s[β] += a;
            self.p[α] = β;
        }
        true
    }

    fn group_size(&self, group: usize) -> usize {
        self.s[group]
    }
}

pub fn karg(graph: &Graph) -> Option<usize> {
    let mut v = graph.v;
    let mut s = UnionFind::new(v);
    while v > 2 {
        let i = rand::limit::u64(graph.edges.len() as u64);
        let α = s.find(graph.edges[i.nat()].0);
        let β = s.find(graph.edges[i.nat()].1);
        if α == β {
            continue;
        }
        v -= 1;
        s.union(α, β);
    }

    let chop = graph
        .edges
        .iter()
        .filter(|&&(src, dest)| s.find(src) != s.find(dest))
        .count();

    if chop == 3 {
        let root = s.find(0);
        let size = s.group_size(root);
        return Some((graph.v - size) * size);
    }
    None
}

fn base26([a, b, c]: [u8; 3]) -> u32 {
    a.widen().widen() + b.widen().widen() * 26 + c.widen().widen() * 26 * 26
}

pub fn run(i: &str) -> impl Display {
    let mut z = 0;
    let mut n = HashMap::new();
    let mut e: HashMap<u32, HashSet<u32>> = HashMap::new();
    i.行()
        .map(|x| {
            let (k, v) = x
                .μ(':')
                .mr(|x| {
                    x.split(|&x| x == b' ')
                        .map(<[u8]>::trim_ascii)
                        .filter(|x| !x.is_empty())
                        .map(|x| base26(x.try_into().unwrap()))
                        .collect_vec()
                })
                .ml(<[u8]>::trim_ascii)
                .ml(|x| base26(x.try_into().unwrap()));
            match n.entry(k) {
                Entry::Occupied(_) => {}
                Entry::Vacant(x) => {
                    x.insert(z);
                    z += 1;
                }
            }
            for j in v {
                match e.entry(k) {
                    Entry::Occupied(x) => {
                        x.into_mut().insert(j);
                    }
                    Entry::Vacant(x) => {
                        x.insert(HashSet::from_iter([j]));
                    }
                }
                match n.entry(j) {
                    Entry::Occupied(_) => {}
                    Entry::Vacant(x) => {
                        x.insert(z);
                        z += 1;
                    }
                }
            }
        })
        .Θ();
    let mut edges = vec![];
    for (from, to) in e {
        for to in to {
            edges.push((n[&from], n[&to]));
        }
    }
    let g = Graph { edges, v: z };
    loop {
        if let Some(x) = karg(&g) {
            return x;
        }
    }
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
