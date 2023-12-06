#![allow(non_snake_case)]
use std::str::FromStr;

pub mod prelude {
    pub use super::{GreekTools, IterͶ, Ͷ, Α, Κ, Λ, Μ};
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

pub trait Λ<T>
where
    T: FromStr,
{
    fn λ(&self) -> T
    where
        <T as FromStr>::Err: std::fmt::Display;
}

impl<T: FromStr> Λ<T> for &str {
    /// parse, unwrap
    fn λ(&self) -> T
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

pub trait GreekTools<T> {
    fn Δ(&mut self) -> T;
}

impl<T, I: Iterator<Item = T>> GreekTools<T> for I {
    fn Δ(&mut self) -> T {
        self.next().α()
    }
}
