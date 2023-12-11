#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(
    core_intrinsics,
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
    #[allow(dead_code)]
    New = 10,
}

#[derive(Copy, Clone, Debug)]
enum D {
    N,
    E,
    S,
    W,
}

impl std::fmt::Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8 as char)
    }
}

impl Pipe {
    fn ch(self) -> char {
        match self {
            Self::New => panic!(),
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

const S: u8 = 140;
const WI: u16 = S as u16 + 1;

struct Map<'a> {
    map: &'a [Pipe],
}

impl<'a> std::ops::Index<u16> for Map<'a> {
    type Output = Pipe;

    fn index(&self, index: u16) -> &'a Self::Output {
        unsafe { self.map.get_unchecked(index.nat()) }
    }
}

impl std::fmt::Display for Map<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        pa(self.map);
        for x in 0..S {
            for y in 0..S {
                write!(f, "{}", self.at(x, y))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Map<'_> {
    fn at(&self, x: u8, y: u8) -> Pipe {
        self[y.widen() * WI + x.widen()]
    }

    fn start(&self, offset: u16) -> (Pipe, D) {
        if self[offset - 1].e() && self[offset - WI] == Pipe::NW && self[offset + WI].n() {
            return (Pipe::SW, D::S);
        }
        match [
            self[offset - 1].s() as u8,
            self[offset + WI].w() as u8,
            self[offset - WI].e() as u8,
            self[offset + 1].n() as u8,
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

    fn go(&self, p: Pipe, offset: &mut u16, d: D) {
        use D::*;
        #[rustfmt::skip]
        macro_rules! n { () => { *offset -= WI }; }
        #[rustfmt::skip]
        macro_rules! e { () => { *offset += 1 }; }
        #[rustfmt::skip]
        macro_rules! s { () => { *offset += WI }; }
        #[rustfmt::skip]
        macro_rules! w { () => { *offset -= 1 }; }
        mat!(p {
            Pipe::EW => mat!(d {
                E => e!(),
                W => w!(),
            }),
            Pipe::SW => mat!(d {
                S => s!(),
                W => w!(),
            }),
            Pipe::NS => mat!(d {
                N => n!(),
                S => s!(),
            }),
            Pipe::SE => mat!(d {
                E => e!(),
                S => s!(),
            }),
            Pipe::NE => mat!(d {
                N => n!(),
                E => e!(),
            }),
            Pipe::NW => mat!(d {
                N => n!(),
                W => w!(),
            }),
        })
    }

    fn turn(&self, p: Pipe, d: &mut D) {
        use D::*;
        *d = mat!(p {
            Pipe::EW => mat!(d {
                E => E, // keep going east
                W => W, // keep going west
            }),
            Pipe::NW => mat!(d {
                E => N, // start going north
                S => W, // start going west
            }),
            Pipe::SW => mat!(d {
                E => S, // start going south
                N => W, // start going west
            }),
            Pipe::SE => mat!(d {
                N => E, // start going east
                W => S, // start going south
            }),
            Pipe::NS => mat!(d {
                N => N, // keep going north
                S => S, // keep going south
            }),
            Pipe::NE => mat!(d {
                W => N, // start going north
                S => E, // start going east
            }),
        });
    }

    fn p2(&self) -> usize {
        let mut offset = self.search();
        let mut network = vec![None; WI as usize * WI as usize];
        let (mut at, mut dir) = self.start(offset);
        network[offset.nat()] = Some(at);
        loop {
            self.go(at, &mut offset, dir);
            at = self[offset];
            if at == Pipe::Start {
                break;
            }
            network[offset.nat()] = Some(at);
            self.turn(at, &mut dir);
        }

        let mut inside = 0;
        for row in unsafe { network.as_chunks_unchecked::<141>() } {
            let mut up = 0u32;
            let mut down = 0u32;
            for &x in row.iter().take(140) {
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

    fn search(&self) -> u16 {
        self.map.iter().position(|&x| x == Pipe::Start).unwrap() as u16
    }

    fn p1(&self) -> usize {
        let mut offset = self.search();
        let (mut at, mut dir) = self.start(offset);
        let mut steps = 1;
        loop {
            self.go(at, &mut offset, dir);
            at = self[offset];
            if at == Pipe::Start {
                break;
            }
            self.turn(at, &mut dir);
            steps += 1;
        }
        steps / 2
    }
}

impl From<&[u8]> for Map<'_> {
    fn from(i: &[u8]) -> Self {
        Self {
            map: unsafe { core::mem::transmute(i) },
        }
    }
}

pub fn run(i: &str) -> impl Display {
    let map = Map::from(i.as_bytes());
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
