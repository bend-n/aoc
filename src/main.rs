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
use util::Ronge;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum When {
    // m>N
    Gt(What, u16),
    Lt(What, u16),
    Always,
}

impl std::fmt::Display for When {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            When::Gt(a, b) => write!(f, "{a:?}>{b}"),
            When::Lt(a, b) => write!(f, "{a:?}<{b}"),
            When::Always => Ok(()),
        }
    }
}

impl When {
    fn accepts(self) -> Option<What> {
        match self {
            Self::Gt(y, _) | Self::Lt(y, _) => Some(y),
            Self::Always => None,
        }
    }

    fn test(self, w: u16) -> bool {
        match self {
            Self::Gt(_, x) => w > x,
            Self::Lt(_, x) => w < x,
            Self::Always => true,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Then<'a> {
    Go(&'a [u8]),
    Accept = b'A',
    Reject = b'R',
}

impl Display for Then<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Then::Go(x) => write!(f, "{}", x.p()),
            Then::Accept => write!(f, "A"),
            Then::Reject => write!(f, "R"),
        }
    }
}

impl<'a> Then<'a> {
    pub fn get<'b>(self, from: &'a [Workflow<'b>]) -> &'a Workflow<'b> {
        mat!(self {
            Then::Go(x) => from.iter().find(|w| w.name == x).α(),
        })
    }

    pub fn from(x: u8) -> Self {
        mat!(x {
            b'A' => Self::Accept,
            b'R' => Self::Reject,
        })
    }

    pub fn from2(x: &'a [u8]) -> Self {
        if let &[x] = x {
            Self::from(x)
        } else {
            Self::Go(x)
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types, dead_code)]
enum What {
    x = b'x',
    m = b'm',
    a = b'a',
    s = b's',
}

impl What {
    pub fn select<T>(self, [x, m, a, s]: [T; 4]) -> T {
        match self {
            What::x => x,
            What::m => m,
            What::a => a,
            What::s => s,
        }
    }

    pub fn select_mut<T>(self, [x, m, a, s]: &mut [T; 4]) -> &mut T {
        match self {
            What::x => x,
            What::m => m,
            What::a => a,
            What::s => s,
        }
    }

    pub fn from(x: u8) -> Self {
        unsafe { rint(x) }
    }
}

#[derive(Debug, Copy, Clone)]
struct Rule<'a> {
    condition: When,
    then: Then<'a>,
}

impl Display for Rule<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.condition, self.then)
    }
}

impl<'a> Rule<'a> {
    fn takes(self) -> Option<What> {
        self.condition.accepts()
    }

    fn consider(self, x: u16) -> Option<Then<'a>> {
        self.condition.test(x).then_some(self.then)
    }
}

struct Workflow<'a> {
    name: &'a [u8],
    rules: Box<[Rule<'a>]>,
}

impl<'a> Workflow<'a> {
    fn test(&self, x: [u16; 4]) -> Option<Then> {
        for rule in &*self.rules {
            if let Some(x) = rule.takes().map(|y| y.select(x)) {
                if let Some(x) = rule.consider(x) {
                    return Some(x);
                }
            } else {
                return Some(rule.then);
            }
        }
        dang!()
    }

    fn new(name: &'a [u8], from: impl Iterator<Item = &'a [u8]>) -> Self {
        let mut rules = vec![];
        for rule in from {
            if let &[b] = rule {
                rules.push(Rule {
                    condition: When::Always,
                    then: Then::from(b),
                })
            } else {
                let Some((cond, then)) = rule.split_once(|&x| x == b':') else {
                    rules.push(Rule {
                        condition: When::Always,
                        then: Then::from2(rule),
                    });
                    continue;
                };

                if let Some((&[x], y)) = cond.split_once(|&x| x == b'<') {
                    rules.push(Rule {
                        condition: When::Lt(What::from(x), y.λ()),
                        then: Then::from2(then),
                    })
                } else if let Some((&[x], y)) = cond.split_once(|&x| x == b'>') {
                    rules.push(Rule {
                        condition: When::Gt(What::from(x), y.λ()),
                        then: Then::from2(then),
                    })
                } else {
                    shucks!()
                }
            }
        }
        Self {
            rules: rules.into_boxed_slice(),
            name,
        }
    }
}

pub fn p2(i: &str) -> u64 {
    let mut workflows = vec![];
    let mut i = i.行();
    for x in i.by_ref() {
        if x == b"" {
            break;
        }
        let (work, rules) = x.μ('{').mr(|x| x.μ0('}').split(|&x| x == b','));
        let flow = Workflow::new(work, rules);
        workflows.push(flow);
    }
    let mut s = 0;
    let h = Workflow {
        name: b"",
        rules: Box::new([]),
    };
    util::iterg(
        (Then::Go(b"in"), [Ronge::from(1..=4001); 4]),
        &mut |(work, mut r): (Then<'_>, [Ronge; 4])| {
            let work = match work {
                Then::Reject => &h, // why are you like this
                g => g.get(&workflows),
            };
            work.rules.iter().map(move |x| {
                let mut r2 = r;
                match x.condition {
                    When::Gt(c, x) => {
                        c.select_mut(&mut r2).begin = x + 1;
                        c.select_mut(&mut r).end = x + 1;
                    }
                    When::Lt(c, x) => {
                        c.select_mut(&mut r2).end = x;
                        c.select_mut(&mut r).begin = x;
                    }
                    When::Always => (),
                }
                (x.then, r2)
            })
        },
        &mut |(x, _)| x == Then::Accept,
        &mut |(_, r)| {
            s += r
                .iter()
                .map(|x| x.end.abs_diff(x.begin) as u64)
                .product::<u64>()
        },
    );
    s
}

pub fn run(i: &str) -> impl Display {
    p2(i)
}

pub fn p1(i: &str) -> u32 {
    let mut workflows = vec![];
    let mut first = None;
    let mut i = i.行();
    for x in i.by_ref() {
        if x == b"" {
            break;
        }
        let (work, rules) = x.μ('{').mr(|x| x.μ0('}').split(|&x| x == b','));
        let flow = Workflow::new(work, rules);
        if work == b"in" {
            first = Some(flow)
        } else {
            workflows.push(flow);
        }
    }
    let first = first.unwrap();
    let mut acc = 0;
    for x in i {
        let mut w = &first;
        let a = x
            .between('{', '}')
            .split(|&x| x == b',')
            .map(|x| x.μ1('=').λ())
            .Ν();
        loop {
            if let Some(x) = w.test(a) {
                match x {
                    Then::Accept => {
                        acc += a.iter().map(|&x| x as u32).sum::<u32>();
                        break;
                    }
                    Then::Go(y) => w = workflows.iter().find(|x| x.name == y).α(),
                    Then::Reject => break,
                }
            }
        }
    }
    acc
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
