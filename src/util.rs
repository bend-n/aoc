#![allow(non_snake_case)]
use std::{mem::swap, str::FromStr};

pub mod prelude {
    pub use super::{
        gcd, lcm, GreekTools, IterͶ, NumTupleIterTools, TupleIterTools, TupleUtils,
        UnifiedTupleUtils, Ͷ, Α, Κ, Λ, Μ,
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
    pub(crate) use {super::bits, super::dang};
}

macro_rules! dang {
    () => {
        panic!()
    };
}
pub(crate) use dang;

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
    return a << shift;
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

pub trait IterͶ {
    fn ͷ(self) -> impl Iterator<Item = u8>;
}

impl<I: Iterator<Item = u64>> IterͶ for I {
    fn ͷ(self) -> impl Iterator<Item = u8> {
        self.flat_map(Ͷ::ͷ)
    }
}

pub trait TupleIterTools<T, U> {
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

pub trait GreekTools<T> {
    fn Δ(&mut self) -> T;
    fn ι(&mut self) -> impl Iterator<Item = (T, u64)>;
    fn ι1(&mut self) -> impl Iterator<Item = (T, u64)>;
}

impl<T, I: Iterator<Item = T>> GreekTools<T> for I {
    fn Δ(&mut self) -> T {
        self.next().α()
    }

    fn ι(&mut self) -> impl Iterator<Item = (T, u64)> {
        self.zip(0..)
    }

    fn ι1(&mut self) -> impl Iterator<Item = (T, u64)> {
        self.zip(1..)
    }
}

pub trait TupleUtils<T, U> {
    fn map<V, W>(self, f: impl FnOnce((T, U)) -> (V, W)) -> (V, W);
    fn mr<W>(self, f: impl FnOnce(U) -> W) -> (T, W);
    fn ml<V>(self, f: impl FnOnce(T) -> V) -> (V, U);
    fn rev(self) -> (U, T);
}

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
