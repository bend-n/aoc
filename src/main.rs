#![allow(
    confusable_idents,
    uncommon_codepoints,
    non_upper_case_globals,
    internal_features,
    mixed_script_confusables,
    static_mut_refs,
    incomplete_features
)]
#![feature(
    iter_repeat_n,
    slice_swap_unchecked,
    generic_const_exprs,
    iter_array_chunks,
    get_many_mut,
    maybe_uninit_uninit_array,
    iter_collect_into,
    hint_assert_unchecked,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    vec_into_raw_parts,
    try_blocks,
    slice_take,
    portable_simd,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    core_intrinsics
)]
extern crate test;
pub mod util;
pub use util::prelude::*;

const SIZE: usize = 50;

#[no_mangle]
pub fn p2(i: &str) -> impl Display {
    #[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
    enum Item {
        File(usize, u8),
        No,
        Space,
        Spaces(u8),
    }
    #[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
    enum SimplifiedItem {
        File(usize),
        Space,
    }
    let i = i.as_bytes();
    let mut files = 0;
    let mut map = i
        .iter()
        .ι::<usize>()
        .flat_map(|(x, i)| {
            if *x == b'\n' {
                return vec![];
            }
            if i % 2 == 1 {
                vec![Item::Space; (*x - b'0') as usize]
            } else {
                files += 1;
                vec![Item::File(i / 2, *x - b'0')]
            }
        })
        .collect_vec();
    for i in (0..files).rev() {
        let (i, _, &size) = map
            .iter()
            .enumerate()
            .rev()
            .find_map(|(j, x)| match x {
                Item::File(a, n) => (*a == i).then_some((j, a, n)),
                _ => None,
            })
            .unwrap();

        let empty = map
            .iter()
            .ι::<usize>()
            .group_by(|&(&x, _)| x == Item::Space)
            .into_iter()
            .filter(|x| x.0)
            .map(|(_, mut x)| {
                let (_, i) = x.Δ();
                let n = x.count() + 1;
                (i, n)
            })
            .find(|&(_, x)| x >= size as usize)
            .map(|(x, _)| x);

        if let Some(empty) = empty
            && empty < i
        {
            let f = &mut map[i];
            map[empty as usize] = std::mem::replace(f, Item::Spaces(size));
            for elem in 1..size {
                map[empty + elem as usize] = Item::No;
            }
        }
        // #[cfg(debug_assertions)]
        // println!(
        //     "{}",
        //     map.iter()
        //         .flat_map(|&x| match x {
        //             Item::File(id, n) => std::iter::repeat_n(SimplifiedItem::File(id), n as usize),
        //             Item::Space => std::iter::repeat_n(SimplifiedItem::Space, 1),
        //             Item::Spaces(n) => std::iter::repeat_n(SimplifiedItem::Space, n as usize),
        //             Item::No => std::iter::repeat_n(SimplifiedItem::Space, 0),
        //         })
        //         .map(|x| {
        //             match x {
        //                 SimplifiedItem::File(i) => format!("{i}"),
        //                 SimplifiedItem::Space => format!("."),
        //             }
        //         })
        //         .collect::<String>()
        // );
    }
    map.into_iter()
        .flat_map(|x| match x {
            Item::File(id, n) => std::iter::repeat_n(SimplifiedItem::File(id), n as usize),
            Item::Space => std::iter::repeat_n(SimplifiedItem::Space, 1),
            Item::Spaces(n) => std::iter::repeat_n(SimplifiedItem::Space, n as usize),
            Item::No => std::iter::repeat_n(SimplifiedItem::Space, 0),
        })
        .ι::<usize>()
        .filter_map(|(x, i)| match x {
            SimplifiedItem::File(x) => Some((x, i)),
            SimplifiedItem::Space => None,
        })
        .map(|(id, i)| id * i)
        .sum::<usize>()
    // 0
}

#[no_mangle]
pub fn run(i: &str) -> impl Display {
    let i = i.as_bytes().trim_ascii_end();
    const SPACE: u16 = u16::MAX;
    let map = i
        .iter()
        .ι::<u16>()
        .flat_map(|(x, i)| {
            let times = (*x - b'0') as usize;
            std::iter::repeat_n(if i % 2 == 1 { SPACE } else { i / 2 }, times)
        })
        .collect_vec();
    let (map, len, _) = map.into_raw_parts();
    let eight_bit = unsafe { std::slice::from_raw_parts(map as *const u8, len * 2) };
    let mut emptys = memchr::memmem::find_iter(eight_bit, &[0xff; 2]).map(|x| x / 2);
    for i in (0..len).rev() {
        if unsafe { *map.add(i) == SPACE } {
            continue;
        }
        let empty = emptys.Δ();
        if empty > i {
            break;
        }
        unsafe { map.add(empty).swap(map.add(i)) };
    }
    unsafe { std::slice::from_raw_parts(map, memchr::memmem::find(eight_bit, &[0xff; 2]).ψ() / 2) }
        .iter()
        .copied()
        .ι::<usize>()
        .map(|(id, i)| id as usize * i)
        .sum::<usize>()
    // 0
}

fn main() {
    // (1..u32::MAX as u64).for_each(|a| assert_eq!(a.ilog10() + 1, digs(a)));
    // let mut s = String::new();
    // for i in 0..1280 {
    let i = include_str!("inp.txt");
    //     s.push_str(i);
    // }
    // std::fs::write("src/inp.txt", s);
    println!("{}", p2(i));
    println!("{}", run(i));
    // println!("{}", p1(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| run(i));
}
