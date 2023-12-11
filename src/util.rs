#![allow(non_snake_case, unused_macros)]
use std::{
    mem::{swap, MaybeUninit},
    str::FromStr,
};

pub mod prelude {
    pub use super::{
        gcd, lcm, pa, GreekTools, IntoCombinations, IntoLines, IterͶ, NumTupleIterTools, Skip,
        TakeLine, TupleIterTools, TupleUtils, UnifiedTupleUtils, Widen, Ͷ, Α, Κ, Λ, Μ,
    };
    pub use itertools::izip;
    pub use itertools::Itertools;
    pub use std::{
        cmp::Ordering::*,
        collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
        fmt::{Debug, Display},
        hint::black_box as boxd,
        iter,
        ops::Range,
    };
    #[allow(unused_imports)]
    pub(crate) use {super::bits, super::dang, super::leek, super::mat};
}

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
        match $thing { $($what => { $b })+ _ => unsafe { std::hint::unreachable_unchecked() } }
    };
}
pub(crate) use mat;

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
        self.split_once(|&x| x == d as u8).unwrap_or_else(|| {
            panic!(
                "{} should split at {d} fine",
                std::str::from_utf8(self).expect("utf8")
            )
        })
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
            .unwrap_or_else(|| panic!("{self} should split at {d} fine"))
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

pub trait TupleIterTools<T, U>: Iterator {
    fn rml(self) -> impl Iterator<Item = U>;
    fn rmr(self) -> impl Iterator<Item = T>;
}

pub trait NumTupleIterTools {
    fn πολλαπλασιάζω_και_αθροίζω(&mut self) -> u64;
}

impl<I: Iterator<Item = (u64, u64)>> NumTupleIterTools for I {
    fn πολλαπλασιάζω_και_αθροίζω(&mut self) -> u64 {
        self.map(|(a, b)| a * b).sum()
    }
}

impl<T, U, I: Iterator<Item = (T, U)>> TupleIterTools<T, U> for I {
    fn rml(self) -> impl Iterator<Item = U> {
        self.map(|(_, x)| x)
    }
    fn rmr(self) -> impl Iterator<Item = T> {
        self.map(|(x, _)| x)
    }
}

pub trait GreekTools<T>: Iterator {
    fn Δ(&mut self) -> T;
    fn ι(&mut self) -> impl Iterator<Item = (T, u64)>;
    fn ι1(&mut self) -> impl Iterator<Item = (T, u64)>;
    fn Ν<const N: usize>(&mut self) -> [T; N];
    fn ν<const N: usize>(&mut self, into: &mut [T; N]) -> usize;
}

impl<T, I: Iterator<Item = T>> GreekTools<T> for I {
    #[track_caller]
    fn Δ(&mut self) -> T {
        self.next().α()
    }

    fn ι(&mut self) -> impl Iterator<Item = (T, u64)> {
        self.zip(0..)
    }

    fn ι1(&mut self) -> impl Iterator<Item = (T, u64)> {
        self.zip(1..)
    }

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
}

pub trait TupleUtils<T, U> {
    fn map<V, W>(self, f: impl FnOnce((T, U)) -> (V, W)) -> (V, W);
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
    fn map<V, W>(self, f: impl FnOnce((T, U)) -> (V, W)) -> (V, W) {
        f(self)
    }
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
