#![allow(confusable_idents, uncommon_codepoints, mixed_script_confusables)]
#![feature(array_windows, test, slice_as_chunks, array_chunks)]
extern crate test;
mod util;
pub use explicit_cast::prelude::*;
pub use util::prelude::*;

fn 力(手札: &str) -> u8 {
    let mut 品: HashMap<_, u8> = HashMap::new();
    for &文字 in 手札.as_bytes() {
        *品.entry(文字).or_default() += 1;
    }

    if 品.values().any(|v| *v == 5) {
        return 7;
    }

    if 品.values().any(|v| *v == 4) {
        return 6;
    }

    {
        let mut 数字 = 品.values();
        match 数字.by_ref().next().α() {
            2 => match 数字.by_ref().next().α() {
                3 => return 5,
                _ => {}
            },
            3 => match 数字.by_ref().next().α() {
                2 => return 5,
                _ => {}
            },
            _ => {}
        }
    }

    if 品.values().any(|v| *v == 3) {
        return 4;
    }

    {
        let mut 二人一組 = false;
        for &v in 品.values() {
            if v == 2 {
                if !二人一組 {
                    二人一組 = true
                } else {
                    return 3;
                }
            }
        }
    }

    if 品.values().count() == 4 {
        return 2; // one pair
    }

    if 品.values().count() == 5 {
        return 1; // high card
    }

    dang!();
}

#[test]
fn strengths() {
    assert_eq!(力("AAAAA"), 7);
    assert_eq!(力("AAAAQ"), 6);
    assert_eq!(力("AAAQQ"), 5);
    assert_eq!(力("AAA12"), 4);
    assert_eq!(力("AA110"), 3);
    assert_eq!(力("AA140"), 2);
    assert_eq!(力("A124Q"), 1);
}

fn solve(i: &str) -> impl Display {
    fn карта(x: u8) -> u8 {
        match x {
            b'0'..=b'9' => x - b'0',
            b'T' => 10,
            b'J' => 11,
            b'Q' => 12,
            b'K' => 13,
            b'A' => 14,
            _ => unreachable!(),
        }
    }
    i.lines()
        .map(|x| x.μ(' ').mr(|x| x.λ::<u64>()))
        .sorted_by(|(a, _), (b, _)| match 力(a).cmp(&力(b)) {
            x if matches!(x, Less | Greater) => x,
            _ => {
                for (a, b) in izip!(a.bytes().map(карта), b.bytes().map(карта)) {
                    match a.cmp(&b) {
                        x if matches!(x, Less | Greater) => return x,
                        _ => continue,
                    }
                }
                dang!();
            }
        })
        .rml()
        .ι1()
        .πολλαπλασιάζω_και_αθροίζω()
}

fn main() {
    let i = include_str!("inp.txt").trim();
    println!("{}", solve(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| solve(i));
}
