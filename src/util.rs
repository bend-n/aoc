#![allow(non_snake_case, unused_macros, unused_unsafe)]

use Default::default;
use regex::Regex;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;
use std::collections::VecDeque;
use std::iter::Zip;
use std::iter::empty;
use std::iter::successors;
use std::ops::RangeFrom;
use std::sync::LazyLock;
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fmt::{Debug, Display, Write},
    hash::Hash,
    mem::swap,
    ops::RangeInclusive,
    str::FromStr,
};

pub mod prelude {
    pub use super::{
        AndF, BoolTools, DigiCount, Dir, FilterBy, FilterBy3, FirstMax, GreekTools, GridFind,
        IntoCombinations, IntoLines, IterͶ, Left, MapWith, NumTupleIterTools, PRead, ParseIter,
        PartitionByKey, Position, Printable, Skip, Splib, SplitU8, Str, TakeLine, TupleIterTools2,
        TupleIterTools2R, TupleIterTools3, TupleUtils, TwoWayMapCollect, UnifiedTupleUtils,
        UnsoundUtilities, Widen, countmap, even, gcd, gt, infinite_successors, l, lcm, lt, nail,
        pa, python, r, rand, reading, reading::Ext, sort, spiral, twice, Ͷ, Α, Ι, Κ, Λ, Μ,
    };
    #[allow(unused_imports)]
    pub(crate) use super::{C, bits, dang, leek, mat, shucks};
    pub use Default::default;
    pub use itertools::Itertools;
    pub use itertools::iproduct;
    pub use itertools::izip;
    pub use rustc_hash::FxHashMap as HashMap;
    pub use rustc_hash::FxHashSet as HashSet;
    pub use std::{
        cmp::Ordering::*,
        cmp::{max, min},
        collections::{VecDeque, hash_map::Entry},
        fmt::{Debug, Display},
        hint::black_box as boxd,
        intrinsics::transmute_unchecked as transmute,
        io::{self, Read, Write},
        iter,
        iter::{chain, once, repeat_n, successors, zip},
        mem::{replace as rplc, swap},
        ops::Range,
    };
}

macro_rules! C {
    ($obj:ident.$what:ident$($tt:tt)+) => {{
        let x = &mut $obj.$what;
        C!( x$($tt)+ )
    }};
    (&$buf:ident[$n:expr]) => {{
        #[allow(unused_unsafe)]
        unsafe {
            $buf.get_unchecked($n)
        }
    }};
    ($buf:ident[$n:expr]) => {{
        #[allow(unused_unsafe)]
        *unsafe {
            $buf.get_unchecked($n)
        }
    }};
    (&mut $buf:ident[$n:expr]) => {{
        #[allow(unused_unsafe)]
        unsafe {
            $buf.get_unchecked_mut($n)
        }
    }};
    ($buf:ident[$a:expr] = $rbuf:ident[$b:expr]) => {
        *unsafe { $buf.get_unchecked_mut($a) } = unsafe { *$rbuf.get_unchecked($b) }
    };
    ($buf:ident[$n:expr] = $e:expr) => {
        *unsafe { $buf.get_unchecked_mut($n) } = $e
    };
    ($buf:ident[$a:expr][$b:expr]) => {
        unsafe { *$buf.get_unchecked($a).get_unchecked($b) }
    };
    ($buf:ident[$a:expr][$b:expr] = $rbuf:ident[$ra:expr]) => {
        *unsafe { $buf.get_unchecked_mut($a).get_unchecked_mut($b) } =
            unsafe { *$rbuf.get_unchecked($ra) }
    };
    ($buf:ident[$a:expr][$b:expr] = $rbuf:ident[$ra:expr][$rb:expr]) => {
        *unsafe { $buf.get_unchecked_mut($a).get_unchecked_mut($b) } =
            unsafe { *$rbuf.get_unchecked($ra).get_unchecked($rb) }
    };
    ($buf:ident[$a:expr][$b:expr] = $c:expr) => {{
        #[allow(unused_unsafe)]
        {
            *unsafe { $buf.get_unchecked_mut($a).get_unchecked_mut($b) } = unsafe { $c }
        }
    }};
}
pub(crate) use C;

macro_rules! shucks {
    () => {
        if cfg!(debug_assertions) {
            unreachable!();
        } else {
            unsafe { std::hint::unreachable_unchecked() }
        }
    };
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        if cfg!(debug_assertions) {
            unreachable!($fmt $(, $args)*);
        } else {
            unsafe { std::hint::unreachable_unchecked() }
        }
    };
    (if $x:expr) => {
        if $x {
            if cfg!(debug_assertions) {
                unreachable!();
            } else {
                unsafe { std::hint::unreachable_unchecked() }
            }
        }
    };
}
pub(crate) use shucks;

macro_rules! dang {
    () => {
        panic!()
    };
}
pub(crate) use dang;

macro_rules! leek {
    ($($allocation:ident)+) => {
        $(std::mem::forget($allocation);)+
    };
}
pub(crate) use leek;

macro_rules! mat {
    ($thing:ident { $($what:pat => $b:expr,)+ }) => {
        match $thing { $($what => { $b })+ _ => shucks!() }
    };
}
pub(crate) use mat;

#[cfg(target_feature = "avx2")]
unsafe fn count_avx<const N: usize>(hay: &[u8; N], needle: u8) -> usize {
    use std::arch::x86_64::*;
    let find = _mm256_set1_epi8(needle as i8);
    let mut counts = _mm256_setzero_si256();
    for i in 0..(N / 32) {
        counts = _mm256_sub_epi8(
            counts,
            _mm256_cmpeq_epi8(
                _mm256_loadu_si256(hay.as_ptr().add(i * 32) as *const _),
                find,
            ),
        );
    }
    const MASK: [u8; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ];
    counts = _mm256_sub_epi8(
        counts,
        _mm256_and_si256(
            _mm256_cmpeq_epi8(
                _mm256_loadu_si256(hay.as_ptr().add(N - 32) as *const _),
                find,
            ),
            _mm256_loadu_si256(MASK.as_ptr().add(N % 32) as *const _),
        ),
    );

    let sums = _mm256_sad_epu8(counts, _mm256_setzero_si256());
    (_mm256_extract_epi64(sums, 0)
        + _mm256_extract_epi64(sums, 1)
        + _mm256_extract_epi64(sums, 2)
        + _mm256_extract_epi64(sums, 3)) as usize
}

pub fn count<const N: usize>(hay: &[u8; N], what: u8) -> usize {
    #[cfg(target_feature = "avx2")]
    return unsafe { count_avx(hay, what) };
    #[cfg(not(target_feature = "avx2"))]
    hay.iter().filter(|&&x| x == what).count()
}

pub fn lcm(n: impl IntoIterator<Item = u64>) -> u64 {
    let mut x = n.into_iter();
    let mut lcm = x.by_ref().next().expect("cannot compute LCM of 0 numbers");
    let mut gcd;
    for x in x {
        gcd = crate::util::gcd(x, lcm);
        lcm = (lcm * x) / gcd;
    }
    lcm
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Dir {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
}

impl Dir {
    pub const ALL: [Self; 4] = [Self::N, Self::E, Self::S, Self::W];
    pub fn urdl(x: u8) -> Self {
        match x {
            b'U' => Self::N,
            b'R' => Self::E,
            b'D' => Self::S,
            b'L' => Self::W,
            x => unreachable!("{}", x as char),
        }
    }
    pub fn add_1(self) -> Self {
        use Dir::*;
        unsafe { std::mem::transmute((self as u8 + 1) % 4) }
    }
    pub fn sub_1(self) -> Self {
        use Dir::*;
        unsafe { std::mem::transmute(((self as u8).wrapping_sub(1)) % 4) }
    }
    pub fn turdl(self) -> u8 {
        match self {
            Self::N => b'U',
            Self::E => b'R',
            Self::S => b'D',
            Self::W => b'L',
        }
    }
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

    pub fn reset(&mut self) {
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

    pub fn group_size(&self, group: usize) -> usize {
        self.s[group]
    }
}

pub trait UnsoundUtilities<T> {
    fn ψ(self) -> T;
}

impl<T> UnsoundUtilities<T> for Option<T> {
    #[cfg_attr(debug_assertions, track_caller)]
    fn ψ(self) -> T {
        if cfg!(debug_assertions) && self.is_none() {
            panic!();
        }
        unsafe { self.unwrap_unchecked() }
    }
}

impl<T, E> UnsoundUtilities<T> for Result<T, E> {
    #[cfg_attr(debug_assertions, track_caller)]
    fn ψ(self) -> T {
        if cfg!(debug_assertions) && self.is_err() {
            panic!();
        }
        unsafe { self.unwrap_unchecked() }
    }
}

pub struct LMap<K, V>(HashMap<K, V>, fn(&K) -> V)
where
    K: Eq + Hash + Clone;
impl<K: Ord + Eq + Debug + Hash + Clone, V: Copy + Debug> LMap<K, V> {
    pub fn new(f: fn(&K) -> V) -> Self {
        Self {
            0: HashMap::default(),
            1: f,
        }
    }

    pub fn with_cap(f: fn(&K) -> V, cap: usize) -> Self {
        Self {
            0: HashMap::with_capacity_and_hasher(cap, rustc_hash::FxBuildHasher::default()),
            1: f,
        }
    }

    pub fn get(&mut self, k: K) -> V {
        if let Some(x) = self.0.get(&k) {
            return *x;
        }
        let elm = self.1(&k);
        self.0.insert(k.clone(), elm);
        elm
    }
}

macro_rules! memoize {
    (|$pat:pat_param in $in:ty| -> $out:ty $body:block; $arg:expr) => {{
        static mut MEMOIZER: std::sync::OnceLock<crate::util::LMap<$in, $out>> =
            std::sync::OnceLock::new();
        unsafe {
            MEMOIZER.get_mut_or_init(|| crate::util::LMap::new(|$pat: &$in| -> $out { $body }))
        }
        .get($arg)
    }};
    (|$pat:pat_param in $in:ty| -> $out:ty $body:block; $arg:expr; with cap $cap:literal) => {{
        static mut MEMOIZER: std::sync::OnceLock<crate::util::LMap<$in, $out>> =
            std::sync::OnceLock::new();
        unsafe {
            MEMOIZER.get_mut_or_init(|| {
                crate::util::LMap::with_cap(|$pat: $in| -> $out { $body }, $cap)
            })
        }
        .get($arg)
    }};
}
#[allow(unused_imports)]
pub(crate) use memoize;

pub fn countg_with_check<N: Debug + PartialEq + Hash + Eq + Copy, I: Iterator<Item = N>>(
    start: N,
    graph: &mut impl Fn(N) -> I,
    ok: &mut impl Fn(N, N) -> bool,
    sum: &mut usize,
    end: &mut impl Fn(N) -> bool,
) {
    if end(start) {
        *sum += 1;
    } else {
        graph(start).for_each(|x| {
            if ok(start, x) {
                // println!("\"{start:?}\" -> \"{x:?}\"");
                countg_with_check(x, graph, ok, sum, end);
            }
        });
    }
}

pub fn countg_uniq_with_check<N: Debug + PartialEq + Hash + Eq + Copy, I: Iterator<Item = N>>(
    start: N,
    graph: &mut impl Fn(N) -> I,
    ok: &mut impl Fn(N, N) -> bool,
    sum: &mut usize,
    end: &mut impl Fn(N) -> bool,
    has: &mut HashSet<N>,
) {
    if end(start) && has.insert(start) {
        *sum += 1;
    } else {
        graph(start).for_each(|x| {
            if ok(start, x) {
                countg_uniq_with_check(x, graph, ok, sum, end, has);
            }
        });
    }
}

pub fn countg<N: Debug + PartialEq + Hash + Eq + Clone, I: Iterator<Item = N>>(
    start: N,
    graph: &mut impl FnMut(N) -> I,
    end: &mut impl FnMut(&N) -> bool,
    sum: &mut usize,
    has: &mut HashSet<N>,
) {
    if end(&start) {
        *sum += 1;
    } else {
        graph(start).for_each(|x| {
            countg(x, graph, end, sum, has);
        });
    }
}

// pub fn appearances(x: )

pub fn iterg<N: Debug, I: Iterator<Item = N>>(
    start: N,
    graph: &mut impl Fn(N) -> I,
    end: &mut impl Fn(&N) -> bool,
    finally: &mut impl FnMut(N),
) {
    if end(&start) {
        finally(start);
    } else {
        graph(start).for_each(|x: N| iterg(x, graph, end, finally));
    };
}

pub fn show<N: Debug + Eq + Hash + Copy + Ord, I: Iterator<Item = (N, u16)>, D: Display>(
    start: N,
    graph: impl Fn(N) -> I,
    end: impl Fn(N) -> bool,
    name: impl Fn(N) -> D,
) {
    println!("digraph {{");
    let mut s = HashSet::default();
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, start)));
    while let Some(Reverse((c, n))) = q.pop() {
        if end(n) {
            println!("}}");
            return;
        }
        if !s.insert(n) {
            continue;
        }
        print!("\t{}", name(n));
        for (n, d) in graph(n) {
            if s.contains(&n) {
                continue;
            }
            print!(" -> {}", name(n));
            q.push(Reverse((c + d, n)));
        }
        println!(";");
    }
    dang!();
}

fn memo_countg_<T, FN, IN, FS>(
    start: T,
    successors: &mut FN,
    success: &mut FS,
    cache: &mut HashMap<T, usize>,
) -> usize
where
    T: Eq + Hash,
    FN: FnMut(&T) -> IN,
    IN: IntoIterator<Item = T>,
    FS: FnMut(&T) -> bool,
{
    if let Some(&n) = cache.get(&start) {
        return n;
    }

    let count = if success(&start) {
        1
    } else {
        successors(&start)
            .into_iter()
            .map(|successor| memo_countg_(successor, successors, success, cache))
            .sum()
    };

    cache.insert(start, count);

    count
}

pub fn memo_countg<T, FN, IN, FS>(start: T, mut successors: FN, mut success: FS) -> usize
where
    T: Eq + Hash,
    FN: FnMut(&T) -> IN,
    IN: IntoIterator<Item = T>,
    FS: FnMut(&T) -> bool,
{
    memo_countg_(
        start,
        &mut successors,
        &mut success,
        &mut HashMap::default(),
    )
}

pub fn reachable<D: Hash + Copy, N: Debug + Eq + Hash + Copy + Ord, I: Iterator<Item = (N, D)>>(
    start: (N, D),
    graph: impl Fn((N, D)) -> I,
) -> HashSet<N> {
    let mut map = HashSet::default();
    let mut stack = VecDeque::from_iter([start]);
    while let Some((x, d)) = stack.pop_front() {
        if map.insert(x) {
            stack.extend(graph((x, d)));
        }
    }
    map
}

pub fn ccomponents<I: Iterator<Item = usize>>(graph: impl Fn(usize) -> I, length: usize) -> usize {
    let mut s = UnionFind::new(length);
    for n in 0..length {
        graph(n).map(|y| s.union(n, y)).θ();
    }
    (0..length).map(|x| s.find(x)).collect::<HashSet<_>>().len()
}

pub fn dijkstra_h<N: Debug + Eq + Hash + Copy + Ord, I: Iterator<Item = (N, u16)>>(
    start: N,
    graph: impl Fn(N) -> I,
    end: impl Fn(N) -> bool,
    h: impl Fn(N) -> u16,
) -> u16 {
    let mut q = BinaryHeap::new();
    let mut s = HashSet::default();
    q.push(Reverse((h(start), 0, start)));
    while let Some(Reverse((_, c, n))) = q.pop() {
        if end(n) {
            return c;
        }
        if !s.insert(n) {
            continue;
        }
        for (n, d) in graph(n) {
            if s.contains(&n) {
                continue;
            }
            q.push(Reverse((h(n) + c + d, c + d, n)));
        }
    }
    dang!()
}
#[derive(Debug, Clone, Copy)]
pub struct Left<T, U>(pub T, pub U);

impl<T: Hash, U> Hash for Left<T, U> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: PartialEq, U> PartialEq for Left<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: Eq, U> Eq for Left<T, U> {}
impl<T: PartialOrd, U> PartialOrd for Left<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<T: Ord, U> Ord for Left<T, U> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
pub fn steps<N: Debug + Eq + Hash + Clone, I: Iterator<Item = N>>(
    start: N,
    graph: impl Fn(N) -> I,
    end: impl Fn(&N) -> bool,
) -> Option<Left<N, usize>> {
    bfs(
        Left(start, 0usize),
        |Left(n, steps)| graph(n).map(move |x| Left(x, steps + 1)),
        |x| end(&x.0),
    )
}

pub fn steps_astar<N: Debug + Eq + Hash + Clone, I: Iterator<Item = (N, i16)>>(
    start: N,
    graph: impl Fn(N) -> I,
    heuristic: impl Fn(&N) -> i16,
    goal: impl Fn(&N) -> bool,
) -> Option<(i16, Left<N, usize>)> {
    astar(
        Left(start, 0usize),
        |Left(n, steps)| graph(n).map(move |(x, cost)| (Left(x, steps + 1), cost)),
        |x| heuristic(&x.0),
        |x| goal(&x.0),
    )
}

pub fn bfs<N: Debug + Eq + Hash + Clone, I: Iterator<Item = N>>(
    start: N,
    graph: impl Fn(N) -> I,
    end: impl Fn(&N) -> bool,
) -> Option<N> {
    let mut q = VecDeque::with_capacity(1024);
    let mut s = HashSet::with_capacity_and_hasher(1024, rustc_hash::FxBuildHasher::default());
    q.push_back(start);
    while let Some(n) = q.pop_front() {
        if end(&n) {
            return Some(n);
        }
        if !s.insert(n.clone()) {
            continue;
        }
        for n in graph(n) {
            if s.contains(&n) {
                continue;
            }
            // print!("{n:?} ");
            q.push_back(n);
        }
    }
    None
}

pub fn astar<N: Debug + Eq + Hash + Clone, I: Iterator<Item = (N, i16)>>(
    start: N,
    graph: impl Fn(N) -> I,
    heuristic: impl Fn(&N) -> i16,
    goal: impl Fn(&N) -> bool,
) -> Option<(i16, N)> {
    let mut q = BinaryHeap::with_capacity(1024);
    let mut score = HashMap::default();
    score.insert(start.clone(), 0);
    q.push(Reverse(Left(heuristic(&start), start)));
    while let Some(Reverse(Left(c, n))) = q.pop() {
        if goal(&n) {
            return Some((c, n));
        }
        for (n_, d) in graph(n.clone()) {
            let g = score[&n] + d;
            let of = score.get(&n_).unwrap_or(&i16::MAX);
            if g < *of {
                score.insert(n_.clone(), g);
                let f = g + heuristic(&n_);
                q.push(Reverse(Left(f, n_)));
            }
        }
    }
    None
}

pub fn dijkstra<N: Debug + Eq + Hash + Clone + Ord, I: Iterator<Item = (N, u16)>>(
    start: N,
    graph: impl Fn(N) -> I,
    end: impl Fn(&N) -> bool,
) -> Option<(u16, N)> {
    let mut q = BinaryHeap::new();
    let mut s = HashSet::default();
    q.push(Reverse(Left(0, start)));
    while let Some(Reverse(Left(c, n))) = q.pop() {
        if end(&n) {
            return Some((c, n));
        }
        if !s.insert(n.clone()) {
            continue;
        }
        for (n, d) in graph(n) {
            if s.contains(&n) {
                continue;
            }
            // print!("{n:?} ");
            q.push(Reverse(Left(c + d, n)));
        }
    }
    None
}

impl std::ops::Add<(i64, i64)> for Dir {
    type Output = (i64, i64);
    fn add(self, (x, y): (i64, i64)) -> Self::Output {
        match self {
            Dir::N => (x, y - 1),
            Dir::E => (x + 1, y),
            Dir::S => (x, y + 1),
            Dir::W => (x - 1, y),
        }
    }
}
impl std::ops::Add<(u32, u32)> for Dir {
    type Output = Option<(u32, u32)>;
    fn add(self, (x, y): (u32, u32)) -> Self::Output {
        Some(match self {
            Dir::N => (x, y.checked_sub(1)?),
            Dir::E => (x + 1, y),
            Dir::S => (x, y + 1),
            Dir::W => (x.checked_sub(1)?, y),
        })
    }
}

impl Dir {
    pub fn lim_add(
        self,
        (x, y): (usize, usize),
        [minx, maxx]: [usize; 2],
        [miny, maxy]: [usize; 2],
    ) -> Option<(usize, usize)> {
        let (x, y) = match self {
            Dir::N => (x, y.checked_sub(1)?),
            Dir::E => (x.wrapping_add(1), y),
            Dir::S => (x, y.wrapping_add(1)),
            Dir::W => (x.checked_sub(1)?, y),
        };

        ((minx..maxx).contains(&x) & (miny..maxy).contains(&y)).then_some((x, y))
    }
}

impl std::ops::Add<(usize, usize)> for Dir {
    type Output = Option<(usize, usize)>;
    fn add(self, (x, y): (usize, usize)) -> Self::Output {
        Some(match self {
            Dir::N => (x, y.checked_sub(1)?),
            Dir::E => (x + 1, y),
            Dir::S => (x, y + 1),
            Dir::W => (x.checked_sub(1)?, y),
        })
    }
}

impl std::ops::Add<(i32, i32)> for Dir {
    type Output = (i32, i32);
    fn add(self, (x, y): (i32, i32)) -> Self::Output {
        match self {
            Dir::N => (x, y - 1),
            Dir::E => (x + 1, y),
            Dir::S => (x, y + 1),
            Dir::W => (x - 1, y),
        }
    }
}

impl std::ops::Add<(u16, u16)> for Dir {
    type Output = (u16, u16);

    fn add(self, (x, y): (u16, u16)) -> Self::Output {
        match self {
            Dir::N => (x, y - 1),
            Dir::E => (x + 1, y),
            Dir::S => (x, y + 1),
            Dir::W => (x - 1, y),
        }
    }
}

impl std::ops::Add<(i16, i16)> for Dir {
    type Output = (i16, i16);
    fn add(self, (x, y): (i16, i16)) -> Self::Output {
        match self {
            Dir::N => (x, y - 1),
            Dir::E => (x + 1, y),
            Dir::S => (x, y + 1),
            Dir::W => (x - 1, y),
        }
    }
}

impl std::ops::Add<(u8, u8)> for Dir {
    type Output = (u8, u8);

    fn add(self, (x, y): (u8, u8)) -> Self::Output {
        match self {
            Dir::N => (x, y.wrapping_sub(1)),
            Dir::E => (x.wrapping_add(1), y),
            Dir::S => (x, y.wrapping_add(1)),
            Dir::W => (x.wrapping_sub(1), y),
        }
    }
}

impl std::ops::Add<i8> for Dir {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        unsafe { std::mem::transmute(((self as u8 as i8) + rhs).rem_euclid(4)) }
    }
}

impl std::ops::Add<u8> for Dir {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        unsafe { std::mem::transmute(((self as u8).wrapping_add(rhs)) % 4) }
    }
}

impl Dir {
    pub fn turn_90(self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }
    pub fn turn_90ccw(self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
        }
    }
}

pub fn pa<T: std::fmt::Debug>(a: &[T]) {
    for e in a {
        print!("{e:?}");
    }
    println!();
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 || b == 0 {
        return a | b;
    }
    let shift = (a | b).trailing_zeros();
    a >>= shift;
    loop {
        b >>= b.trailing_zeros();
        if a > b {
            swap(&mut a, &mut b);
        }
        b -= a;
        if b == 0 {
            break;
        }
    }
    a << shift
}

pub trait Λ {
    fn λ<T: FromStr>(&self) -> T
    where
        <T as FromStr>::Err: std::fmt::Display;
}

impl Λ for String {
    fn λ<T: FromStr>(&self) -> T
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        self.as_str().λ()
    }
}
impl Λ for [u8] {
    #[cfg_attr(debug_assertions, track_caller)]
    fn λ<T: FromStr>(&self) -> T
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        std::str::from_utf8(self).α().λ()
    }
}
impl Λ for &str {
    /// parse, unwrap
    #[cfg_attr(debug_assertions, track_caller)]
    fn λ<T: FromStr>(&self) -> T
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        match self.parse() {
            Ok(v) => v,
            Err(e) => {
                panic!(
                    "{e}: {self} should parse into {}",
                    std::any::type_name::<T>()
                )
            }
        }
    }
}
pub trait Κ {
    fn κ<T: FromStr>(self) -> impl Iterator<Item = T>
    where
        <T as FromStr>::Err: std::fmt::Display;
}

impl Κ for &[u8] {
    fn κ<T: FromStr>(self) -> impl Iterator<Item = T>
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        self.str().κ()
    }
}

impl Κ for &str {
    fn κ<T: FromStr>(self) -> impl Iterator<Item = T>
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        self.split_ascii_whitespace().map(|x| x.λ())
    }
}

pub trait Α<T> {
    fn α(self) -> T;
}

impl<T, E: std::fmt::Display> Α<T> for Result<T, E> {
    #[cfg_attr(debug_assertions, track_caller)]
    fn α(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                panic!("unwrap failed: {e}");
            }
        }
    }
}
impl<T> Α<T> for Option<T> {
    #[cfg_attr(debug_assertions, track_caller)]
    fn α(self) -> T {
        match self {
            Some(v) => v,
            None => panic!("nothingness!"),
        }
    }
}

pub trait DigiCount {
    fn ͱ(self) -> u32;
}

pub const powers: [u64; 20] = car::from_fn!(|x| 10u64.pow(x as u32));
// https://stackoverflow.com/a/9721570
impl DigiCount for u64 {
    fn ͱ(self) -> u32 {
        static powers: [u64; 20] = car::from_fn!(|x| 10u64.pow(x as u32));
        static mdigs: [u32; 65] = car::from_fn!(|x| 2u128.pow(x as u32).ilog10() + 1);
        let bit = std::mem::size_of::<Self>() * 8 - self.leading_zeros() as usize;
        let mut digs = mdigs[bit];
        if self < C! { powers[digs as usize - 1] } {
            digs -= 1;
        }
        digs
    }
}

impl DigiCount for u32 {
    fn ͱ(self) -> Self {
        static powers: [u32; 10] = car::from_fn!(|x| 10u32.pow(x as u32));
        static mdigs: [u32; 33] = car::from_fn!(|x| 2u128.pow(x as u32).ilog10() + 1);
        let bit = std::mem::size_of::<Self>() * 8 - self.leading_zeros() as usize;
        let mut digs = mdigs[bit];
        if self < C! { powers[digs as usize - 1] } {
            digs -= 1;
        }
        digs
    }
}

impl DigiCount for u16 {
    fn ͱ(self) -> u32 {
        self.checked_ilog10().ψ() + 1
    }
}

impl DigiCount for u8 {
    fn ͱ(self) -> u32 {
        self.checked_ilog10().ψ() + 1
    }
}

impl DigiCount for u128 {
    fn ͱ(self) -> u32 {
        self.checked_ilog10().ψ() + 1
    }
}

pub trait Ͷ {
    fn ͷ(self) -> impl Iterator<Item = u8>;
    fn Ͷ(self, i: u8) -> u8;
}

macro_rules! digs {
    ($for:ty) => {
        impl Ͷ for $for {
            fn ͷ(self) -> impl Iterator<Item = u8> {
                let digits = self.ͱ() as u8;
                (0..digits).rev().map(move |n| self.Ͷ(n))
            }
            fn Ͷ(self, i: u8) -> u8 {
                ((self / (10 as $for).pow(i as _)) % 10) as u8
            }
        }
    };
}
digs!(u128);
digs!(u64);
digs!(u32);
digs!(u16);
digs!(u8);

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Ronge {
    pub begin: u16,
    pub end: u16,
}

impl Debug for Ronge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.begin, self.end)
    }
}

impl Display for Ronge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.begin, self.end)
    }
}

impl From<RangeInclusive<u16>> for Ronge {
    fn from(value: RangeInclusive<u16>) -> Self {
        Self {
            begin: *value.start(),
            end: *value.end(),
        }
    }
}

impl PartialEq<RangeInclusive<u16>> for Ronge {
    fn eq(&self, other: &RangeInclusive<u16>) -> bool {
        self == &Self::from(other.clone())
    }
}

impl Ronge {
    pub fn sane(self) -> bool {
        self.end >= self.begin
    }
    pub fn checked_len(self) -> Option<u16> {
        self.sane().then(|| self.len())
    }
    pub fn len(self) -> u16 {
        self.end - self.begin
    }

    /// push up
    pub fn pushu(&mut self, to: u16) {
        self.begin = self.begin.max(to);
    }

    /// push down
    pub fn pushd(&mut self, to: u16) {
        self.end = self.end.min(to);
    }

    pub fn intersect(self, with: Self) -> Self {
        Self {
            begin: self.begin.max(with.begin),
            end: self.end.min(with.end),
        }
    }

    pub fn news(&self, begin: u16) -> Self {
        Self {
            begin,
            end: self.end,
        }
    }

    pub fn newe(&self, end: u16) -> Self {
        Self {
            begin: self.begin,
            end,
        }
    }

    pub fn shrink(&mut self, with: Ronge) {
        self.pushu(with.begin);
        self.pushd(with.end);
    }
}

impl IntoIterator for Ronge {
    type Item = u16;

    type IntoIter = std::ops::Range<u16>;

    fn into_iter(self) -> Self::IntoIter {
        self.begin..self.end
    }
}

pub trait Μ where
    Self: Sized,
{
    fn μ(self, d: char) -> (Self, Self);
    fn μκ<T: FromStr>(self, d: char) -> impl Iterator<Item = (T, T)>
    where
        <T as FromStr>::Err: std::fmt::Display;

    fn μ1(self, d: char) -> Self {
        self.μ(d).1
    }

    fn μ0(self, d: char) -> Self {
        self.μ(d).0
    }

    fn between(self, a: char, b: char) -> Self {
        self.μ1(a).μ0(b)
    }
}

impl Μ for &[u8] {
    fn μ(self, d: char) -> (Self, Self) {
        let i = memchr::memchr(d as u8, self)
            .unwrap_or_else(|| shucks!("{} should split at {d} fine", self.p()));
        (&self[..i], &self[i + 1..])
    }

    fn μκ<T: FromStr>(self, d: char) -> impl Iterator<Item = (T, T)>
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        let (α, β) = self.μ(d);
        α.κ::<T>().zip(β.κ::<T>())
    }
}

pub fn gt<A: std::cmp::PartialOrd<T>, T>(n: T) -> impl Fn(A) -> bool {
    move |a| a > n
}

pub fn lt<A: std::cmp::PartialOrd<T>, T>(n: T) -> impl Fn(A) -> bool {
    move |a| a < n
}

pub trait SplitU8 {
    fn μₙ(&self, x: u8) -> impl Iterator<Item = &[u8]> + Clone;
}
impl SplitU8 for [u8] {
    fn μₙ(&self, x: u8) -> impl Iterator<Item = &[u8]> + Clone {
        self.split(move |&y| y == x)
    }
}

impl Μ for &str {
    fn μ(self, d: char) -> (Self, Self) {
        self.split_once(d)
            .unwrap_or_else(|| shucks!("{self} should split at {d} fine"))
    }

    fn μκ<T: FromStr>(self, d: char) -> impl Iterator<Item = (T, T)>
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        let (α, β) = self.μ(d);
        α.κ::<T>().zip(β.κ::<T>())
    }
}

pub trait IterͶ: Iterator {
    fn ͷ(self) -> impl Iterator<Item = u8>;
}

impl<I: Iterator<Item = u64>> IterͶ for I {
    fn ͷ(self) -> impl Iterator<Item = u8> {
        self.flat_map(Ͷ::ͷ)
    }
}

pub trait TupleIterTools3<T, U, V>: Iterator {
    fn l(self) -> impl Iterator<Item = T>;
    fn m(self) -> impl Iterator<Item = U>;
    fn r(self) -> impl Iterator<Item = V>;
    fn lm(self) -> impl Iterator<Item = (T, U)>;
    fn lr(self) -> impl Iterator<Item = (T, V)>;
    fn mr(self) -> impl Iterator<Item = (U, V)>;
}

pub trait TupleIterTools2<T, U>: Iterator {
    fn l(self) -> impl Iterator<Item = T>;
    fn r(self) -> impl Iterator<Item = U>;
    fn map_l<V>(self, f: impl FnMut(T) -> V) -> impl Iterator<Item = (V, U)>;
    fn map_r<V>(self, f: impl FnMut(U) -> V) -> impl Iterator<Item = (T, V)>;
}

pub trait TupleIterTools2R<T, U>: Iterator {
    fn l(self) -> impl Iterator<Item = T>;
    fn r(self) -> impl Iterator<Item = U>;
}

pub fn l<R, T, U>(f: impl Fn(T) -> R) -> impl Fn((T, U)) -> R {
    move |(x, _)| f(x)
}
pub fn r<R, T, U>(f: impl Fn(U) -> R) -> impl Fn((T, U)) -> R {
    move |(_, x)| f(x)
}

pub trait FilterBy3<T, U, V>: Iterator {
    fn fl(self, f: impl Fn(T) -> bool) -> impl Iterator<Item = (T, U, V)>;
    fn fm(self, f: impl Fn(U) -> bool) -> impl Iterator<Item = (T, U, V)>;
    fn fr(self, f: impl Fn(V) -> bool) -> impl Iterator<Item = (T, U, V)>;
}

impl<T: Copy, U: Copy, V: Copy, I: Iterator<Item = (T, U, V)>> FilterBy3<T, U, V> for I {
    fn fl(self, f: impl Fn(T) -> bool) -> impl Iterator<Item = (T, U, V)> {
        self.filter(move |(x, _, _)| f(*x))
    }

    fn fm(self, f: impl Fn(U) -> bool) -> impl Iterator<Item = (T, U, V)> {
        self.filter(move |(_, x, _)| f(*x))
    }
    fn fr(self, f: impl Fn(V) -> bool) -> impl Iterator<Item = (T, U, V)> {
        self.filter(move |(_, _, x)| f(*x))
    }
}
pub trait FilterBy<T, U>: Iterator {
    fn fl(self, f: impl FnMut(T) -> bool) -> impl Iterator<Item = (T, U)>;
    fn fr(self, f: impl FnMut(U) -> bool) -> impl Iterator<Item = (T, U)>;
}

impl<T: Copy, U: Copy, I: Iterator<Item = (T, U)>> FilterBy<T, U> for I {
    fn fl(self, mut f: impl FnMut(T) -> bool) -> impl Iterator<Item = (T, U)> {
        self.filter(move |(x, _)| f(*x))
    }

    fn fr(self, mut f: impl FnMut(U) -> bool) -> impl Iterator<Item = (T, U)> {
        self.filter(move |(_, x)| f(*x))
    }
}

pub trait NumTupleIterTools {
    fn πολλαπλασιάζω_και_αθροίζω(&mut self) -> u64;
}

impl<I: Iterator<Item = (u64, u64)>> NumTupleIterTools for I {
    fn πολλαπλασιάζω_και_αθροίζω(&mut self) -> u64 {
        self.map(|(a, b)| a * b).sum()
    }
}

impl<T, U, I: Iterator<Item = (T, U)>> TupleIterTools2<T, U> for I {
    fn l(self) -> impl Iterator<Item = T> {
        self.map(|(x, _)| x)
    }

    fn r(self) -> impl Iterator<Item = U> {
        self.map(|(_, x)| x)
    }

    fn map_l<V>(self, mut f: impl FnMut(T) -> V) -> impl Iterator<Item = (V, U)> {
        self.map(move |(x, y)| (f(x), y))
    }

    fn map_r<V>(self, mut f: impl FnMut(U) -> V) -> impl Iterator<Item = (T, V)> {
        self.map(move |(x, y)| (x, f(y)))
    }
}

impl<'a, T: Copy + 'a, U: Copy + 'a, I: Iterator<Item = &'a (T, U)>> TupleIterTools2R<T, U> for I {
    fn l(self) -> impl Iterator<Item = T> {
        self.map(|&(x, _)| x)
    }
    fn r(self) -> impl Iterator<Item = U> {
        self.map(|&(_, x)| x)
    }
}

impl<T, U, V, I: Iterator<Item = (T, U, V)>> TupleIterTools3<T, U, V> for I {
    fn l(self) -> impl Iterator<Item = T> {
        self.map(|(x, _, _)| x)
    }

    fn m(self) -> impl Iterator<Item = U> {
        self.map(|(_, x, _)| x)
    }

    fn r(self) -> impl Iterator<Item = V> {
        self.map(|(_, _, x)| x)
    }

    fn lm(self) -> impl Iterator<Item = (T, U)> {
        self.map(|(a, b, _)| (a, b))
    }

    fn lr(self) -> impl Iterator<Item = (T, V)> {
        self.map(|(a, _, b)| (a, b))
    }

    fn mr(self) -> impl Iterator<Item = (U, V)> {
        self.map(|(_, a, b)| (a, b))
    }
}

pub trait GreekTools<T>: Iterator {
    fn Δ(&mut self) -> T;
    fn ν<const N: usize>(self, into: &mut [T; N]) -> usize;
    fn θ(self);
}

pub trait ParseIter {
    fn κ<T: FromStr>(&mut self) -> impl Iterator<Item = T>
    where
        <T as FromStr>::Err: std::fmt::Display;
}

impl<'x, I: Iterator<Item = &'x [u8]>> ParseIter for I {
    fn κ<T: FromStr>(&mut self) -> impl Iterator<Item = T>
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        self.flat_map(|x| x.κ())
    }
}

pub trait Ι<T>: Iterator<Item = T> + Sized {
    fn ι<N: Default + std::iter::Step>(self) -> Zip<Self, RangeFrom<N>> {
        self.zip(N::default()..)
    }
    fn ι1<N: Default + std::iter::Step>(self) -> Zip<Self, std::iter::Skip<RangeFrom<N>>> {
        self.zip((N::default()..).skip(1))
    }
}
impl<T, I: Iterator<Item = T>> Ι<T> for I {}

pub fn nail<const N: usize, T: Copy>(x: &[T]) -> &[T; N] {
    unsafe { &*(x.as_ptr() as *const [T; N]) }
}

pub mod reading {
    pub struct Integer<'a, 'b, T, const BY: u8 = b'\n'> {
        pub x: &'a mut &'b [u8],
        pub _ph: std::marker::PhantomData<T>,
    }

    impl<'a, 'b, T: Copy, const BY: u8> Integer<'a, 'b, T, BY> {
        pub fn new(x: &'a mut &'b [u8]) -> Self {
            Self {
                x,
                _ph: std::marker::PhantomData,
            }
        }
    }
    impl<
        'a,
        'b,
        const BY: u8,
        T: Default + std::ops::Mul<T, Output = T> + Add<T, Output = T> + From<u8> + Copy + Ten + Debug,
    > Iterator for Integer<'a, 'b, T, BY>
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            (!self.x.is_empty()).then(|| read_until(self.x, BY))
        }
    }
    #[inline]
    pub fn 八(n: u64) -> u64 {
        // reinterpret as u64 ("92233721" => 92233721)
        // let n = u64::from_le_bytes(s);
        // combine 4 pairs of single digits:
        // split pieces into odd and even
        //  1_7_3_2_ (le repr)
        // _2_3_2_9
        // then combine
        // _21_37_23_92 (le repr, each byte as 2 digits)
        let n = ((n & 0x0f000f000f000f00) >> 8) + ((n & 0x000f000f000f000f) * 10);
        // combine 2 pairs of 2 digits:
        // split again
        // _21___23__
        // ___37___92
        // then combine
        // __14|137__36|7 (le repr, pipes separating bytes)
        let n = ((n & 0x00ff000000ff0000) >> 16) + ((n & 0x000000ff000000ff) * 100);
        // combine pair of 4 digits
        // split again
        // __14|137____ (then moved to ______14|137, as u64:3721)
        // ______36|07 (as u64: 9223)
        // then combine
        ((n & 0x0000ffff00000000) >> 32) + ((n & 0x000000000000ffff) * 10000)
    }

    use std::{
        io::{self, Read},
        ops::{Add, BitOrAssign, Shl},
    };
    pub trait Ext {
        fn rd<const N: usize>(&mut self) -> io::Result<[u8; N]>;
        fn by(&mut self) -> io::Result<u8> {
            Ok(self.rd::<1>()?[0])
        }
    }

    impl<T: Read> Ext for T {
        fn rd<const N: usize>(&mut self) -> io::Result<[u8; N]> {
            let mut buf = [0; N];
            self.read_exact(&mut buf)?;
            Ok(buf)
        }
    }
    use crate::util::prelude::*;
    pub fn κ<
        T: Default + std::ops::Mul<T, Output = T> + Add<T, Output = T> + From<u8> + Copy + Ten + Debug,
    >(
        x: &[u8],
        v: &mut [T],
    ) -> usize {
        let mut n = 0;
        let mut s = T::default();
        for &b in x {
            match b {
                b' ' => {
                    C! { v[n] = s };
                    n += 1;
                    s = T::default();
                }
                b => {
                    s = s * T::TEN + T::from(b - b'0');
                }
            }
        }
        C! {v[n] = s};
        n + 1
    }
    pub trait Ten {
        const TEN: Self;
        const ONE: Self;
    }
    macro_rules! tenz {
        ($for:ty) => {
            impl Ten for $for {
                const TEN: Self = 10;
                const ONE: Self = 1;
            }
        };
    }
    tenz!(u8);
    tenz!(u16);
    tenz!(u32);
    tenz!(u64);
    tenz!(u128);
    tenz!(usize);
    tenz!(i8);
    tenz!(i16);
    tenz!(i32);
    tenz!(i64);
    tenz!(i128);
    tenz!(isize);

    const DIG: [u8; 256] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 13, 14, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    pub fn hex_dig(b: u8) -> u8 {
        DIG[b.nat()]
        // (b & 0xF) + 9 * (b >> 6)
    }

    pub fn hexN<
        T: From<u8> + TryFrom<usize> + Shl<T, Output = T> + BitOrAssign<T>,
        const N: usize,
    >(
        a: [u8; N],
    ) -> T {
        let mut c = T::from(hex_dig(a[0])) << T::try_from((N - 1) * 4).ψ();
        for (&n, sh) in a[1..].iter().zip((0..N - 1).rev()) {
            c |= T::from(hex_dig(n)) << T::try_from(sh * 4).ψ();
        }
        c
    }

    pub fn hex(mut d: &[u8]) -> Result<u32, ()> {
        let &b = d.split_off_first().ok_or(())?;
        let mut num = hex_dig(b) as u32;
        while let Some(&b) = d.split_off_first() {
            num = num * 16 + hex_dig(b) as u32;
        }
        Ok(num)
    }

    pub fn read_until<
        T: Default + std::ops::Mul<T, Output = T> + Add<T, Output = T> + From<u8> + Copy + Ten,
    >(
        x: &mut &[u8],
        until: u8,
    ) -> T {
        let mut n = T::from(x.by().ψ() - b'0');
        loop {
            let x = x.by().ψ();
            if x == until {
                return n;
            }
            n = n * T::TEN + T::from(x - b'0')
        }
    }

    pub fn 負迄(x: &mut &[u8], until: u8) -> i64 {
        let (sign, mut n) = match x.by().ψ() {
            b'-' => (-1, 0),
            b => (1, i64::from(b - b'0')),
        };
        loop {
            let byte = x.by().ψ();
            if byte == until {
                return n * sign as i64;
            }
            n = n * 10 + i64::from(byte - b'0');
        }
    }

    pub fn until<T: std::ops::Mul<T, Output = T> + Add<T, Output = T> + From<u8> + Copy + Ten>(
        x: &mut &[u8],
        until: u8,
    ) -> T {
        let mut n = T::from(x.by().ψ() - b'0');
        loop {
            let byte = x.by().ψ();
            if byte == until {
                return n;
            }
            n = n * T::TEN + T::from(byte - b'0');
        }
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub fn all<
        T: Default + std::ops::Mul<T, Output = T> + Add<T, Output = T> + From<u8> + Copy + Ten,
    >(
        x: &[u8],
    ) -> T {
        let mut n = T::default();
        for &byte in x {
            n = n * T::TEN + T::from(byte - b'0');
        }
        n
    }
}

pub fn even(x: &usize) -> bool {
    x % 2 == 0
}

impl<T, I: Iterator<Item = T>> GreekTools<T> for I {
    #[cfg_attr(debug_assertions, track_caller)]
    fn Δ(&mut self) -> T {
        self.next().ψ()
    }

    fn ν<const N: usize>(mut self, into: &mut [T; N]) -> usize {
        let mut set = 0;
        for e in into {
            let Some(y) = self.next() else { break };
            *e = y;
            set += 1;
        }
        set
    }

    fn θ(self) {
        for _ in self {}
    }
}

pub trait TupleUtils<T, U> {
    fn mr<W>(self, f: impl FnOnce(U) -> W) -> (T, W);
    fn ml<V>(self, f: impl FnOnce(T) -> V) -> (V, U);
    fn rev(self) -> (U, T);
}

pub trait Widen<Wide> {
    fn nat(self) -> usize;
    fn widen(self) -> Wide;
}

macro_rules! wide {
    ($t:ty: $upper:ty) => {
        impl Widen<$upper> for $t {
            fn nat(self) -> usize {
                self as _
            }

            fn widen(self) -> $upper {
                self as _
            }
        }
    };
}
wide!(u8: u16);
wide!(u16: u32);
wide!(u32: u64);
wide!(u64: u128);
wide!(i64: i128);

pub trait UnifiedTupleUtils<T> {
    fn mb<U>(self, f: impl FnMut(T) -> U) -> (U, U);
}

impl<T> UnifiedTupleUtils<T> for (T, T) {
    fn mb<U>(self, mut f: impl FnMut(T) -> U) -> (U, U) {
        (f(self.0), f(self.1))
    }
}

impl<T, U> TupleUtils<T, U> for (T, U) {
    fn mr<W>(self, f: impl FnOnce(U) -> W) -> (T, W) {
        (self.0, f(self.1))
    }
    fn ml<V>(self, f: impl FnOnce(T) -> V) -> (V, U) {
        (f(self.0), self.1)
    }
    fn rev(self) -> (U, T) {
        (self.1, self.0)
    }
}

#[allow(dead_code)]
fn cast_to<T: From<bool>>(x: bool, _to: T) -> T {
    x.into()
}

#[allow(unused_macros)]
macro_rules! bits {
    ($bitset:ident + $bit:expr) => {
        $bitset |= 1 << $bit
    };
    ($holder:ident[$index:expr] + $bit:expr) => {
        $holder[$index] |= 1 << $bit;
    };
    ($bitset:ident[$bit:expr]) => {
        ($bitset & 1 << $bit) != 0
    };
    ($holder:ident[$index:expr][$bit:expr]) => {
        ($holder[$index] & 1 << $bit) != 0
    };
    ($holder:ident[$index:expr][$index2:expr][$bit:expr]) => {
        ($holder[$index][$index2] & 1 << $bit) != 0
    };
    ($holder:ident[$index:expr][$index2:expr] + $bit:expr) => {
        $holder[$index][$index2] |= 1 << $bit
    };
    ($bitset:ident[$bit:expr] = $val:expr) => {
        $bitset = ($bitset & !(1 << $bit)) | (crate::util::cast_to($val, $bitset) << $bit)
    };
    ($bitset:ident - $bit:expr) => {
        $bitset &= !(1 << $bit)
    };
    ($bitset:ident ! $bit:expr) => {
        $bitset ^= 1 << $bit
    };
}
pub(crate) use bits;

#[derive(Clone, Copy)]
pub struct Lines<'a> {
    pub bytes: &'a [u8],
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.bytes.take_line()
    }
}

impl<'a> std::iter::FusedIterator for Lines<'a> {}

impl<'a> DoubleEndedIterator for Lines<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.bytes.take_backline()
    }
}

#[link(name = "c")]
unsafe extern "C" {
    fn mmap(addr: *mut u8, len: usize, prot: i32, flags: i32, fd: i32, offset: i64) -> *mut u8;
    fn lseek(fd: i32, offset: i64, whence: i32) -> i64;
}

#[allow(dead_code)]
#[unsafe(no_mangle)]
unsafe fn mmaped<'a>() -> (*const u8, i64) {
    let seek_end = 2;
    let size = lseek(0, 0, seek_end);
    if size == -1 {
        unsafe { std::hint::unreachable_unchecked() }
    }
    let prot_read = 0x01;
    let map_private = 0x02;
    let map_populate = 0x08000;
    let ptr = mmap(
        0 as _,
        size as usize,
        prot_read,
        map_private | map_populate,
        0,
        0,
    );
    if ptr as isize == -1 {
        unsafe { std::hint::unreachable_unchecked() }
    }
    (ptr, size)
}

pub trait IntoLines {
    fn 行<'a>(&'a self) -> Lines<'a>;
}
impl IntoLines for [u8] {
    fn 行(&self) -> Lines<'_> {
        Lines { bytes: self }
    }
}

impl IntoLines for str {
    fn 行(&self) -> Lines<'_> {
        Lines {
            bytes: self.as_ref(),
        }
    }
}

pub trait Str {
    fn str(&self) -> &str;
}
impl Str for [u8] {
    #[cfg_attr(debug_assertions, track_caller)]
    fn str(&self) -> &str {
        std::str::from_utf8(self).ψ()
    }
}

pub trait Printable {
    fn p(&self) -> impl std::fmt::Display;
}

struct PrintU8s<'a>(&'a [u8]);
impl std::fmt::Display for PrintU8s<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &b in self.0 {
            if b.is_ascii() {
                f.write_char(b as char)?;
            } else {
                write!(f, "\\x{b:x}")?;
            }
        }
        Ok(())
    }
}

struct PrintManyU8s<'a, T: AsRef<[u8]>>(&'a [T]);
impl<T: AsRef<[u8]>> std::fmt::Display for PrintManyU8s<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.as_ref() {
            write!(f, "{},", row.as_ref().p())?;
        }
        Ok(())
    }
}
impl Printable for [Vec<u8>] {
    fn p(&self) -> impl std::fmt::Display {
        PrintManyU8s(self)
    }
}

impl Printable for [&&[u8]] {
    fn p(&self) -> impl std::fmt::Display {
        PrintManyU8s(self)
    }
}

impl Printable for [&[u8]] {
    fn p(&self) -> impl std::fmt::Display {
        PrintManyU8s(self)
    }
}

impl Printable for [u8] {
    fn p(&self) -> impl std::fmt::Display {
        PrintU8s(self)
    }
}

impl Printable for Vec<u8> {
    fn p(&self) -> impl std::fmt::Display {
        PrintU8s(self)
    }
}

pub fn sort<T: Ord>(mut x: Vec<T>) -> Vec<T> {
    x.sort_unstable();
    x
}

pub trait TakeLine<'b> {
    fn take_line<'a>(&'a mut self) -> Option<&'b [u8]>;
    fn take_backline<'a>(&'a mut self) -> Option<&'b [u8]>;
}

impl<'b> TakeLine<'b> for &'b [u8] {
    fn take_line<'a>(&'a mut self) -> Option<&'b [u8]> {
        match memchr::memchr(b'\n', self) {
            None if self.is_empty() => None,
            None => Some(std::mem::replace(self, b"")),
            Some(end) => {
                let line = C! { &self[..end]};
                *self = C! { &self[end + 1..]};
                Some(line)
            }
        }
    }

    fn take_backline<'a>(&'a mut self) -> Option<&'b [u8]> {
        let end = self.len().checked_sub(1)?;
        match memchr::memrchr(b'\n', &self[..end]) {
            None => Some(std::mem::replace(self, b"")),
            Some(end) => {
                let line = &self[end + 1..];
                *self = &self[..end];
                Some(line)
            }
        }
    }
}

impl<'b> TakeLine<'b> for &'b str {
    fn take_line<'a>(&'a mut self) -> Option<&'b [u8]> {
        match memchr::memchr(b'\n', self.as_bytes()) {
            None if self.is_empty() => None,
            None => Some(std::mem::replace(self, "").as_bytes()),
            Some(end) => {
                let line = self[..end].as_bytes();
                *self = &self[end + 1..];
                Some(line)
            }
        }
    }

    fn take_backline<'a>(&'a mut self) -> Option<&'b [u8]> {
        let end = self.len().checked_sub(1)?;
        match memchr::memrchr(b'\n', &self.as_bytes()[..end]) {
            None => Some(std::mem::replace(self, "").as_bytes()),
            Some(end) => {
                let line = &self[end + 1..];
                *self = &self[..end];
                Some(line.as_bytes())
            }
        }
    }
}

pub trait IntoCombinations<T: Copy>: Iterator {
    /// LEAKY
    fn combine(self) -> impl Iterator<Item = (T, T)>;
}

impl<T: Copy + 'static, I: Iterator<Item = T>> IntoCombinations<T> for I {
    fn combine(self) -> impl Iterator<Item = (T, T)> {
        let x = Box::leak(self.collect::<Box<[_]>>());
        x.iter()
            .enumerate()
            .flat_map(|(i, &a)| x[i..].iter().map(move |&b| (a, b)))
    }
}

pub trait Skip {
    fn skip(&mut self, n: usize);
    fn skip_n(&mut self, n: &'static str) {
        self.skip(n.len())
    }
}

impl<T> Skip for &[T] {
    #[cfg_attr(debug_assertions, track_caller)]
    fn skip(&mut self, n: usize) {
        if cfg!(debug_assertions) {
            *self = &self[n..];
        } else {
            *self = C! { &self[n..] };
        }
    }
}

impl Skip for &str {
    #[cfg_attr(debug_assertions, track_caller)]
    fn skip(&mut self, n: usize) {
        if cfg!(debug_assertions) {
            *self = &self[n..];
        } else {
            *self = C! { &self[n..] };
        }
    }
}

/// WYRAND based rng's
pub mod rand {
    /// WYRAND
    pub fn u64() -> u64 {
        static mut STATE: u64 = 0;
        let tmp = unsafe {
            STATE = STATE.wrapping_add(0x60bee2bee120fc15);
            (STATE as u128).wrapping_mul(0xa3b195354a39b70d)
        };
        let m1 = (tmp >> 64) ^ tmp;
        let tmp = m1.wrapping_mul(0x1b03738712fad5c9);
        ((tmp >> 64) ^ tmp) as u64
    }

    /// 0..N
    pub mod limit {
        use crate::Widen;

        pub fn u64(of: u64) -> u64 {
            ((super::u64().widen().wrapping_mul(of.widen())) >> 64) as u64
        }
    }

    pub fn u32() -> u32 {
        u64() as u32
    }

    pub fn u16() -> u16 {
        u64() as u16
    }

    pub fn f32() -> f32 {
        (1.0 / ((1u32 << 24) as f32)) * ((u32() >> 8) as f32)
    }

    pub fn f64() -> f64 {
        (1.0 / ((1u64 << 53) as f64)) * ((u64() >> 11) as f64)
    }
}

pub fn manhattan((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

pub fn ints<T: FromStr>(x: &'static [u8]) -> impl Iterator<Item = T>
where
    T::Err: std::fmt::Display,
{
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("-?[0-9]+").unwrap());
    RE.find_iter(x.str()).map(|x| x.as_str().λ())
}
pub fn uints<T: FromStr>(x: &[u8]) -> impl Iterator<Item = T>
where
    T::Err: std::fmt::Display,
{
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("[0-9]+").unwrap());
    RE.find_iter(x.str()).map(|x| x.as_str().λ())
}

pub fn nb<
    T: Default + std::ops::Sub<T, Output = T> + std::ops::Add<T, Output = T> + Copy + reading::Ten,
>(
    (x, y): (T, T),
) -> [(T, T); 8] {
    [
        (x - T::ONE, y - T::ONE),
        (x, y - T::ONE),
        (x + T::ONE, y - T::ONE),
        (x - T::ONE, y),
        // this
        (x + T::ONE, y),
        (x - T::ONE, y + T::ONE),
        (x, y + T::ONE),
        (x + T::ONE, y + T::ONE),
    ]
}
pub fn twice<T: Copy>(x: T) -> impl Iterator<Item = T> + Clone + ExactSizeIterator {
    std::iter::repeat_n(x, 2)
}

pub fn infinite_successors<T: Copy>(x: T, mut f: impl FnMut(T) -> T) -> impl Iterator<Item = T> {
    successors(Some(x), move |x| Some(f(*x)))
}
pub fn countmap<T: Hash + Eq + Ord + Copy>(
    x: impl Iterator<Item = T>,
) -> impl Iterator<Item = (T, usize)> {
    let mut h = HashMap::<_, usize>::default();
    for e in x {
        *h.entry(e).or_default() += 1;
    }
    let mut v = h.into_iter().collect::<Vec<_>>();
    v.sort_unstable_by_key(move |&(x, y)| (Reverse(y), x));
    v.into_iter()
}

pub trait MapWith<T: Copy> {
    fn map_w<U>(self, f: impl FnMut(T) -> U) -> impl Iterator<Item = (T, U)>;
    fn fmap_w<U>(self, f: impl FnMut(T) -> Option<U>) -> impl Iterator<Item = (T, U)>;
}
impl<T: Copy, I: Iterator<Item = T>> MapWith<T> for I {
    fn map_w<U>(self, mut f: impl FnMut(T) -> U) -> impl Iterator<Item = (T, U)> {
        self.map(move |x| (x, f(x)))
    }
    fn fmap_w<U>(self, mut f: impl FnMut(T) -> Option<U>) -> impl Iterator<Item = (T, U)> {
        self.flat_map(move |x| Some(x).zip(f(x)))
    }
}

pub fn md5(x: &[u8]) -> [u8; 16] {
    use md5::Digest;
    let mut hasher = md5::Md5::new();
    hasher.update(x);
    hasher.finalize().into()
}

pub fn md5s(x: &[u8]) -> String {
    let mut s = String::with_capacity(32);
    for element in md5(x) {
        write!(s, "{element:02x}").unwrap();
    }
    s
}
pub trait PartitionByKey<T> {
    fn partition_by_key<V, U: Extend<V> + Default>(
        self,
        f: impl FnMut(&T) -> bool,
        key: impl FnMut(T) -> V,
    ) -> [U; 2];
}

impl<T, I: Iterator<Item = T>> PartitionByKey<T> for I {
    fn partition_by_key<V, U: Extend<V> + Default>(
        self,
        mut f: impl FnMut(&T) -> bool,
        mut key: impl FnMut(T) -> V,
    ) -> [U; 2] {
        let mut o: [U; 2] = [default(), default()];
        for x in self {
            match f(&x) {
                true => &mut o[0],
                false => &mut o[1],
            }
            .extend_one(key(x));
        }
        o
    }
}

pub trait BoolTools {
    fn thn(self, f: impl FnMut()) -> Self;
}
impl BoolTools for bool {
    fn thn(self, mut f: impl FnMut()) -> Self {
        if self {
            f();
        }
        self
    }
}

pub trait GridFind {
    fn find(self, c: u8) -> (usize, usize);
    fn find_iter(&self, c: u8) -> impl Iterator<Item = (usize, usize)>;
}
impl<const N: usize, const M: usize> GridFind for [[u8; N]; M] {
    fn find(self, c: u8) -> (usize, usize) {
        let i = memchr::memchr(c, self.as_flattened()).ψ();
        (i % N, i / N)
    }

    fn find_iter(&self, c: u8) -> impl Iterator<Item = (usize, usize)> {
        memchr::memchr_iter(c, self.as_flattened()).map(|i| (i % N, i / N))
    }
}
impl GridFind for &[&[u8]] {
    fn find(self, c: u8) -> (usize, usize) {
        self.iter()
            .zip(0..)
            .find_map(|(x, y)| x.iter().position(|&x| x == c).map(|x| (x, y)))
            .unwrap()
    }

    fn find_iter(&self, c: u8) -> impl Iterator<Item = (usize, usize)> {
        self.iter().zip(0..).flat_map(move |(x, y)| {
            x.iter()
                .ι::<usize>()
                .fl(move |&x| x == c)
                .map(move |(_, x)| (x, y))
        })
    }
}

impl GridFind for &[Vec<u8>] {
    fn find(self, c: u8) -> (usize, usize) {
        self.iter()
            .zip(0..)
            .find_map(|(x, y)| x.iter().position(|&x| x == c).map(|x| (x, y)))
            .unwrap()
    }

    fn find_iter(&self, c: u8) -> impl Iterator<Item = (usize, usize)> {
        self.iter().zip(0..).flat_map(move |(x, y)| {
            x.iter()
                .ι::<usize>()
                .fl(move |&x| x == c)
                .map(move |(_, x)| (x, y))
        })
    }
}

pub trait TwoWayMapCollect<K, V>: Iterator {
    fn collect_twm(self) -> HashMap<(K, K), V>;
}
impl<K: Eq + Hash + Clone, V: Clone, I: Iterator<Item = ((K, K), V)>> TwoWayMapCollect<K, V> for I {
    fn collect_twm(self) -> HashMap<(K, K), V> {
        let mut m = HashMap::default();
        for ((a, b), c) in self {
            m.insert((a.clone(), b.clone()), c.clone());
            m.insert((b, a), c);
        }
        m
    }
}

/// for spiral such as
///  9 10 11 12
///  8  1  2 13
///  7  0  3 14
///  6  5  4 15
pub mod spiral {
    // https://www.desmos.com/calculator/qpn25p39v9
    pub fn coordinate_of(n: u64) -> (i64, i64) {
        let m = n.isqrt() as i64;

        (
            (-1i64).pow(m as u32 + 1)
                * (((n as i64 - m * (m + 1)) // _
                * (((2.0 * (n as f64).sqrt()).floor() as i64 + 1) % 2))
                    + (m + 1) / 2),
            (-1i64).pow(m as u32)
                * ((n as i64 - m * (m + 1)) // _
                * ((2.0 * (n as f64).sqrt()).floor() as i64 % 2)
                    - (m + 1) / 2),
        )
    }

    //https://math.stackexchange.com/a/1860731
    pub fn index_at((x, y): (i64, i64)) -> i64 {
        let s = if x.abs() >= y.abs() { x } else { -y };

        if s >= 0 {
            4 * s * s - x + -y
        } else {
            4 * s * s + (-1_i64).pow((s == x) as u32) * (2 * s + x + -y)
        }
    }
}

pub trait AndF {
    fn and<T>(self, f: impl FnOnce(&Self) -> T) -> Self;
}
impl<T> AndF for T {
    fn and<U>(self, f: impl FnOnce(&Self) -> U) -> Self {
        f(&self);
        self
    }
}

pub trait MapF: Sized {
    #[doc(alias = "map")]
    fn 𝙢𝙖𝙥<T>(self, f: impl FnOnce(Self) -> T) -> T;
}
impl<U> MapF for U {
    fn 𝙢𝙖𝙥<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}

pub trait Splib {
    fn splib(&'_ self, seq: &'static [u8]) -> impl Iterator<Item = &'_ [u8]> + '_;
}
impl Splib for [u8] {
    fn splib(&'_ self, seq: &'static [u8]) -> impl Iterator<Item = &'_ [u8]> + '_ {
        self.str().split(seq.str()).map(str::as_bytes)
    }
}

pub fn parse_digraph<'a, N: Hash + Ord, D>(
    x: &'a [u8],
    mut f: impl FnMut(&'a [u8]) -> (N, D),
) -> HashMap<N, (D, Option<Vec<&'a [u8]>>)> {
    let mut map = HashMap::default();
    x.行().for_each(|x| {
        let (node, connections) = match memchr::memmem::find(x, b"->") {
            Some(index) => (
                &x[..index - 1],
                Some(
                    x[index + 3..]
                        .μₙ(b',')
                        .map(<[u8]>::trim_ascii)
                        .collect::<Vec<_>>(),
                ),
            ),
            None => (x, None),
        };
        let (k, d) = f(node);
        map.insert(k, (d, connections));
    });
    map
}
pub fn parse_graph<'a, N: Hash + Ord + Clone>(
    x: &'a [u8],
    mut f: impl FnMut(&'a [u8]) -> N + Copy,
) -> HashMap<N, Vec<N>> {
    let mut map = HashMap::default();
    x.行().for_each(|x| {
        let (node, connections) = match memchr::memmem::find(x, b"--") {
            Some(index) => (
                &x[..index - 1],
                x[index + 2..]
                    .μₙ(b',')
                    .map(<[u8]>::trim_ascii)
                    .map(f)
                    .collect::<Vec<_>>(),
            ),
            None => (x, vec![]),
        };

        // for c in &connections {
        //     map.entry(c.clone()).or_default().push(c);
        // }
        map.entry(f(node)).or_insert(connections);
    });
    map
}

pub mod python {

    use std::cell::RefCell;
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::sync::Mutex;
    use std::sync::RwLock;

    // struct R(proc_macro::TokenStream);
    // impl<'py, 's> FromPyObject<'py> for R {
    //     fn extract_bound(x: &Bound<'py, PyAny>) -> Result<R, PyErr> {
    //         let tree: TokenTree = match () {
    //             () if let Ok(x) = x.extract::<i128>() => Literal::i128_unsuffixed(x).into(),
    //             () if let Ok(x) = x.extract::<f64>() => Literal::f64_unsuffixed(x).into(),
    //             () if let Ok(x) = x.extract::<bool>() => {
    //                 Ident::new(&x.to_string(), Span::call_site()).into()
    //             }
    //             () if let Ok(x) = x.extract::<String>() => {
    //                 return Ok(R(x
    //                     .parse::<TokenStream>()
    //                     .unwrap_or(quote::quote!(compile_error!("lex failure")).into())));
    //             }

    //             // () if let Ok(x) = x.downcast::<PyList>() => {
    //             //     if let Ok(y) = x.get_item(0) {
    //             //         match () {
    //             //             () if y.is_instance_of::<PyFloat>() => Val::Array(Array::Float(
    //             //                 x.into_iter().map(|x| x.extract::<f64>()).try_collect()?,
    //             //             )),
    //             //             () if y.is_instance_of::<PyInt>() => Val::Array(Array::Int(
    //             //                 x.into_iter().map(|x| x.extract::<i128>()).try_collect()?,
    //             //             )),
    //             //             _ => {
    //             //                 return Err(PyTypeError::new_err("bad array types"));
    //             //             }
    //             //         }
    //             //     } else {
    //             //         Val::Array(Array::Int(vec![]))
    //             //     }
    //             // }
    //             // () if let Ok(x) = x.downcast::<PySet>() => Val::Set(
    //             //     x.into_iter()
    //             //         .map(|x| x.extract::<Val<'s>>())
    //             //         .try_collect()?,
    //             // ),
    //             _ => return Err(PyTypeError::new_err("bad types")),
    //         };
    //         let mut t = TokenStream::new();
    //         t.extend([tree]);
    //         Ok(R(t))
    //     }
    // }
    macro_rules! eval {
        ($fmt:literal $(, $args:expr)* $(,)? => $t:ty) => {{
            use pyo3::*;
            use pyo3::exceptions::PyTypeError;
            use pyo3::prelude::*;
            use pyo3::types::*;
            pyo3::Python::initialize();
            pyo3::Python::attach(|g| {
                g.eval(&std::ffi::CString::new(format!($fmt $(, $args)*)).unwrap(), None, None)
                    .unwrap_or_else(|x| {
                        eprintln!("error:");
                        x.display(g);
                        std::process::exit(1)
                    })
                    .extract::<$t>()
            })
            .unwrap()
        }};
    }
    pub(crate) use eval;
}

pub mod hexagon {
    use atools::prelude::*;
    pub const n: [i64; 2] = [0, -1];
    pub const ne: [i64; 2] = [1, -1];
    pub const se: [i64; 2] = [1, 0];
    pub const s: [i64; 2] = [0, 1];
    pub const sw: [i64; 2] = [-1, 1];
    pub const nw: [i64; 2] = [-1, 0];
    pub fn distance(a: [i64; 2], b: [i64; 2]) -> i64 {
        let [q, r] = a.asub(b);
        ((q).abs() + (q + r).abs() + (r).abs()) / 2
    }
}

pub fn each_bit<
    T: reading::Ten
        + Default
        + std::ops::Shl<usize, Output = T>
        + std::ops::BitAnd<T, Output = T>
        + std::cmp::PartialEq
        + Copy,
>(
    b: T,
) -> [bool; size_of::<T>() * 8] {
    use atools::prelude::*;
    atools::range()
        .rev()
        .map(|x| b & (T::ONE << x) != T::default())
}

pub trait Position<T> {
    fn position(&self, x: T) -> usize;
}

impl<T: PartialEq> Position<T> for [T] {
    fn position(&self, x: T) -> usize {
        self.iter().position(|y| &x == y).ψ()
    }
}
pub trait PRead<T> {
    unsafe fn λ(&mut self) -> T;
}
impl<T> PRead<T> for *const T {
    unsafe fn λ(&mut self) -> T {
        let b = self.read();
        *self = self.add(1);
        b
    }
}

pub trait FirstMax<T> {
    fn fmax_by_left(self) -> T;
}

impl<T: Ord + Default, U: Default, I: Iterator<Item = (T, U)>> FirstMax<(T, U)> for I {
    fn fmax_by_left(self) -> (T, U) {
        let mut best = (T::default(), U::default());
        for el in self {
            if best.0 < el.0 {
                best = el
            }
        }
        best
    }
}
