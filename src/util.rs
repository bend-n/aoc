#![allow(non_snake_case, unused_macros)]
use std::{
    fmt::Write,
    mem::{swap, MaybeUninit},
    str::FromStr,
};

pub mod prelude {
    #[allow(unused_imports)]
    pub(crate) use super::{bits, dang, leek, mat, shucks, C};
    pub use super::{
        even, gcd, lcm, pa, GreekTools, IntoCombinations, IntoLines, IterͶ, NumTupleIterTools,
        ParseIter, Printable, Skip, TakeLine, TupleIterTools2, TupleIterTools3, TupleUtils,
        UnifiedTupleUtils, Widen, 読む, Ͷ, Α, Κ, Λ, Μ,
    };
    pub use itertools::izip;
    pub use itertools::Itertools;
    pub use std::{
        cmp::Ordering::*,
        cmp::{max, min},
        collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
        fmt::{Debug, Display},
        hint::black_box as boxd,
        iter,
        mem::{replace as rplc, swap, transmute as rint},
        ops::Range,
    };
}

macro_rules! C {
    ($buf:ident[$n:expr]) => {
        unsafe { *$buf.get_unchecked($n) }
    };
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

impl Λ for &str {
    /// parse, unwrap
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
                );
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
        std::str::from_utf8(self).unwrap().κ()
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
    #[track_caller]
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
    #[track_caller]
    fn α(self) -> T {
        match self {
            Some(v) => v,
            None => panic!("nothingness!"),
        }
    }
}

pub trait Ͷ {
    fn ͷ(self) -> impl Iterator<Item = u8>;
}

impl Ͷ for u64 {
    fn ͷ(self) -> impl Iterator<Item = u8> {
        let digits = (self.ilog10() + 1) as u8;
        (0..digits)
            .rev()
            .map(move |n| ((self / 10u64.pow(u32::from(n))) % 10) as u8)
    }
}

pub trait Μ where
    Self: Sized,
{
    fn μ(self, d: char) -> (Self, Self);
    fn μκ<T: FromStr>(self, d: char) -> impl Iterator<Item = (T, T)>
    where
        <T as FromStr>::Err: std::fmt::Display;
    fn μ1<T: FromStr>(self, d: char) -> impl Iterator<Item = T>
    where
        <T as FromStr>::Err: std::fmt::Display;
    fn μ0<T: FromStr>(self, d: char) -> impl Iterator<Item = T>
    where
        <T as FromStr>::Err: std::fmt::Display;
}

impl Μ for &[u8] {
    fn μ(self, d: char) -> (Self, Self) {
        let i = self
            .iter()
            .position(|&x| x == d as u8)
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

    fn μ1<T: FromStr>(self, d: char) -> impl Iterator<Item = T>
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        self.μ(d).1.κ()
    }

    fn μ0<T: FromStr>(self, d: char) -> impl Iterator<Item = T>
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        self.μ(d).0.κ()
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

    fn μ1<T: FromStr>(self, d: char) -> impl Iterator<Item = T>
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        self.μ(d).1.κ()
    }

    fn μ0<T: FromStr>(self, d: char) -> impl Iterator<Item = T>
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        self.μ(d).0.κ()
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
    fn ι<N>(&mut self) -> impl Iterator<Item = (T, N)>
    where
        Self: Ι<T, N>;
    fn ι1<N>(&mut self) -> impl Iterator<Item = (T, N)>
    where
        Self: Ι<T, N>;
    fn Ν<const N: usize>(&mut self) -> [T; N];
    fn ν<const N: usize>(&mut self, into: &mut [T; N]) -> usize;
    fn Θ(&mut self);
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

pub trait Ι<T, N>: Iterator {
    fn ι(&mut self) -> impl Iterator<Item = (T, N)>;
    fn ι1(&mut self) -> impl Iterator<Item = (T, N)>;
}

macro_rules! ι {
    ($t:ty) => {
        impl<T, I: Iterator<Item = T>> Ι<T, $t> for I {
            fn ι(&mut self) -> impl Iterator<Item = (T, $t)> {
                self.zip(0..)
            }

            fn ι1(&mut self) -> impl Iterator<Item = (T, $t)> {
                self.zip(1..)
            }
        }
    };
}
ι!(u8);
ι!(u16);
ι!(u32);
ι!(u64);
ι!(usize);

pub mod 読む {
    use crate::util::prelude::*;
    pub fn κ(x: &[u8], v: &mut Vec<u64>) {
        let mut s = 0;
        for &b in x {
            match b {
                b' ' => {
                    v.push(s);
                    s = 0;
                }
                b => {
                    s = s * 10 + (b - b'0') as u64;
                }
            }
        }
    }

    pub fn 不全の(x: &mut &[u8]) -> u64 {
        let mut n = 0;
        loop {
            let byte = x[0];
            x.skip(1);
            if byte == b' ' {
                return n;
            }
            n = n * 10 + (byte - b'0') as u64;
        }
    }

    pub fn 完了(x: &[u8]) -> u64 {
        let mut n = 0;
        for &byte in x {
            n = n * 10 + (byte - b'0') as u64;
        }
        n
    }
}

pub fn even(x: &usize) -> bool {
    x % 2 == 0
}

impl<T, I: Iterator<Item = T>> GreekTools<T> for I {
    #[track_caller]
    fn Δ(&mut self) -> T {
        self.next().α()
    }

    #[track_caller]
    fn Ν<const N: usize>(&mut self) -> [T; N] {
        let mut array: [MaybeUninit<Self::Item>; N] =
            // SAFETY: mu likes this
            unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..array.len() {
            array[i].write(self.Δ());
        }

        // SAFETY: init
        array.map(|elem| unsafe { elem.assume_init() })
    }

    fn ν<const N: usize>(&mut self, into: &mut [T; N]) -> usize {
        let mut set = 0;
        for e in into {
            let Some(y) = self.next() else { break };
            *e = y;
            set += 1;
        }
        set
    }

    fn ι<N>(&mut self) -> impl Iterator<Item = (T, N)>
    where
        Self: Ι<T, N>,
    {
        self.ι()
    }

    fn ι1<N>(&mut self) -> impl Iterator<Item = (T, N)>
    where
        Self: Ι<T, N>,
    {
        self.ι1()
    }

    fn Θ(&mut self) {
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

#[test]
fn do_bits() {
    let mut bitset = 0u128;
    bits!(bitset + 5);
    assert!(bits!(bitset[5]));
    bits!(bitset ! 5);
    assert!(!bits!(bitset[5]));
    bits!(bitset ! 5);
    assert!(bits!(bitset[5]));
    bits!(bitset - 5);
    assert!(!bits!(bitset[5]));
    bits!(bitset[4] = true);
    assert!(bits!(bitset[4]));
}

pub struct Lines<'a> {
    bytes: &'a [u8],
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.bytes.take_line()
    }
}

impl<'a> std::iter::FusedIterator for Lines<'a> {}

pub trait IntoLines {
    fn 行(&self) -> Lines<'_>;
}

impl<T: AsRef<[u8]>> IntoLines for T {
    fn 行(&self) -> Lines<'_> {
        Lines {
            bytes: self.as_ref(),
        }
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

pub trait TakeLine<'b> {
    fn take_line<'a>(&'a mut self) -> Option<&'b [u8]>;
}

impl<'b> TakeLine<'b> for &'b [u8] {
    fn take_line<'a>(&'a mut self) -> Option<&'b [u8]> {
        match memchr::memchr(b'\n', self) {
            None if self.is_empty() => None,
            None => {
                let line = *self;
                *self = b"";
                Some(line)
            }
            Some(end) => {
                let line = &self[..end];
                *self = &self[end + 1..];
                Some(line)
            }
        }
    }
}

impl<'b> TakeLine<'b> for &'b str {
    fn take_line<'a>(&'a mut self) -> Option<&'b [u8]> {
        match memchr::memchr(b'\n', self.as_bytes()) {
            None if self.is_empty() => None,
            None => {
                let line = self.as_bytes();
                *self = "";
                Some(line)
            }
            Some(end) => {
                let line = self[..end].as_bytes();
                *self = &self[end + 1..];
                Some(line)
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
}

impl<T> Skip for &[T] {
    #[track_caller]
    fn skip(&mut self, n: usize) {
        *self = &self[n..];
    }
}

impl Skip for &str {
    #[track_caller]
    fn skip(&mut self, n: usize) {
        *self = &self[n..];
    }
}
