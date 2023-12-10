#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(
    array_windows,
    slice_take,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    byte_slice_trim_ascii
)]
extern crate test;
mod util;
pub use util::prelude::*;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Pipe {
    /// │
    NS = b'|',
    /// ─
    EW = b'-',
    /// ╰
    NE = b'L',
    /// ╯
    NW = b'J',
    /// ╮
    SW = b'7',
    /// ╭
    SE = b'F',
    #[allow(dead_code)] // transmute() go brr
    Ground = b'.',
    Start = b'S',
}

#[derive(Copy, Clone, Debug)]
enum D {
    N,
    E,
    S,
    W,
}

impl Pipe {
    fn ch(self) -> char {
        match self {
            Pipe::NS => '│',
            Pipe::EW => '─',
            Pipe::NE => '╰',
            Pipe::NW => '╯',
            Pipe::SW => '╮',
            Pipe::SE => '╭',
            Pipe::Ground => '.',
            Pipe::Start => 'S',
        }
    }

    fn n(self) -> bool {
        matches!(self, Pipe::NS | Pipe::NE | Pipe::NW)
    }

    fn e(self) -> bool {
        matches!(self, Pipe::EW | Pipe::NE | Pipe::SE)
    }

    fn s(self) -> bool {
        matches!(self, Pipe::SW | Pipe::NS | Pipe::SE)
    }

    fn w(self) -> bool {
        matches!(self, Pipe::EW | Pipe::NW | Pipe::SW)
    }
}

impl std::fmt::Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ch())
    }
}

struct Map<const S: usize> {
    map: [[Pipe; S]; S],
}

impl<const N: usize> std::fmt::Display for Map<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in self.map {
            for y in x {
                write!(f, "{y}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const S: usize> Map<S> {
    fn at(&self, x: u8, y: u8) -> Pipe {
        self.map[y.nat()][x.nat()]
    }

    fn start(&self, x: u8, y: u8) -> (Pipe, D) {
        match [
            self.at(x - 1, y).s() as u8,
            self.at(x, y + 1).w() as u8,
            self.at(x, y - 1).e() as u8,
            self.at(x + 1, y).n() as u8,
        ] {
            [1, 1, 0, 0] => (Pipe::NE, D::E),
            [1, 0, 1, 0] => (Pipe::NS, D::S),
            [1, 0, 0, 1] => (Pipe::NW, D::W),
            [0, 1, 1, 0] => (Pipe::SE, D::S),
            [0, 1, 0, 1] => (Pipe::EW, D::W),
            [0, 0, 1, 1] => (Pipe::SW, D::S),
            _ => dang!(),
        }
    }

    fn go(&self, p: Pipe, x: &mut u8, y: &mut u8, d: D) {
        use D::*;
        #[rustfmt::skip]
        macro_rules! n { () => { *y -= 1 }; }
        #[rustfmt::skip]
        macro_rules! e { () => { *x += 1 }; }
        #[rustfmt::skip]
        macro_rules! s { () => { *y += 1 }; }
        #[rustfmt::skip]
        macro_rules! w { () => { *x -= 1 }; }
        mat!(p {
            Pipe::NS => mat!(d {
                N => n!(),
                S => s!(),
            }),
            Pipe::EW => mat!(d {
                E => e!(),
                W => w!(),
            }),
            Pipe::NE => mat!(d {
                N => n!(),
                E => e!(),
            }),
            Pipe::NW => mat!(d {
                N => n!(),
                W => w!(),
            }),
            Pipe::SW => mat!(d {
                S => s!(),
                W => w!(),
            }),
            Pipe::SE => mat!(d {
                E => e!(),
                S => s!(),
            }),
        })
    }

    fn turn(&self, p: Pipe, d: &mut D) {
        use D::*;
        #[rustfmt::skip]
        macro_rules! n { () => { *d = N }; }
        #[rustfmt::skip]
        macro_rules! e { () => { *d = E }; }
        #[rustfmt::skip]
        macro_rules! s { () => { *d = S }; }
        #[rustfmt::skip]
        macro_rules! w { () => { *d = W }; }
        mat!(p {
            Pipe::NS => mat!(d {
                N => n!(), // keep going north
                S => s!(), // keep going south
            }),
            Pipe::EW => mat!(d {
                E => e!(), // keep going east
                W => w!(), // keep going west
            }),
            Pipe::NE => mat!(d {
                W => n!(), // start going north
                S => e!(), // start going east
            }),
            Pipe::NW => mat!(d {
                E => n!(), // start going north
                S => w!(), // start going west
            }),
            Pipe::SW => mat!(d {
                E => s!(), // start going south
                N => w!(), // start going west
            }),
            Pipe::SE => mat!(d {
                N => e!(), // start going east
                W => s!(), // start going south
            }),
        })
    }

    fn p2(&self) -> usize {
        let (mut x, mut y) = self.search();
        let mut network = [[None; S]; S];
        let (mut at, mut dir) = self.start(x, y);
        network[y.nat()][x.nat()] = Some(at);
        loop {
            self.go(at, &mut x, &mut y, dir);
            at = self.at(x, y);
            if at == Pipe::Start {
                break;
            }
            network[y.nat()][x.nat()] = Some(at);
            self.turn(at, &mut dir);
        }

        let mut inside = 0;
        for row in network {
            let mut up = 0;
            let mut down = 0;
            for x in row {
                match x {
                    Some(x) => {
                        if x.n() {
                            up += 1;
                        }
                        if x.s() {
                            down += 1;
                        }
                    }
                    None => {
                        if up % 2 != 0 && down % 2 != 0 {
                            inside += 1;
                        }
                    }
                }
            }
        }
        inside
    }

    fn search(&self) -> (u8, u8) {
        for (r, j) in self.map.iter().zip(0..) {
            for (&c, i) in r.iter().zip(0..) {
                if c == Pipe::Start {
                    return (i, j);
                }
            }
        }
        dang!();
    }

    fn p1(&self) -> usize {
        let (mut x, mut y) = self.search();
        let (mut at, mut dir) = self.start(x, y);
        let mut steps = 1;
        loop {
            self.go(at, &mut x, &mut y, dir);
            at = self.at(x, y);
            if at == Pipe::Start {
                break;
            }
            self.turn(at, &mut dir);
            steps += 1;
        }
        steps / 2
    }
}

impl<const S: usize> From<&[u8]> for Map<S> {
    fn from(mut i: &[u8]) -> Self {
        Self {
            map: (0..S)
                .map(|n| {
                    let inner = i
                        .take(..S)
                        .α()
                        .iter()
                        .map(|&b| unsafe { std::mem::transmute(b) })
                        .Ν();
                    if n != S - 1 {
                        i.skip(1);
                    }
                    inner
                })
                .Ν(),
        }
    }
}

pub fn run(i: &str) -> impl Display {
    let map = Map::<140>::from(i.as_bytes());
    map.p2()
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
