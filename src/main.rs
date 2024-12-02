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
    slice_swap_unchecked,
    generic_const_exprs,
    iter_array_chunks,
    get_many_mut,
    maybe_uninit_uninit_array,
    iter_collect_into,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    slice_take,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    core_intrinsics
)]
extern crate test;
pub mod util;
use atools::prelude::*;
// use hinted::HintExt;
pub use util::prelude::*;

// 3rd iteration https://godbolt.org/z/bz8qraE56
fn check(x: &[i8]) -> bool {
    let mut array = [0i8; 7];
    let mut set = 0xffu8;
    let n = x.array_windows::<2>().map(|[a, b]| a - b).ν(&mut array);
    let array = &array[..n];
    match array[0] {
        i8::MIN..=-1 => {
            array
                .iter()
                .zip(0..8u8)
                .map(|(x, index)| !(-3..=-1).contains(x) as u8 * (1 << index))
                .for_each(|x| set &= !x);
        }
        1.. => {
            array
                .iter()
                .zip(0..8u8)
                .map(|(x, index)| !(1..=3).contains(x) as u8 * (1 << index))
                .for_each(|x| set &= !x);
        }
        0 => return false,
    }
    set.count_zeros() == 0
}

pub fn run(i: &str) -> impl Display {
    let mut items = [0; 8];
    i.行()
        .filter(|x| {
            let mut len = 0;
            let mut s = 0;
            for &b in *x {
                match b {
                    b' ' => {
                        C! { items[len] = s as i8 };
                        len += 1;
                        s = 0;
                    }
                    b => s = s * 10 + (b - b'0'),
                }
            }
            C! { items[len] = s as i8 };
            let slice = C! { &items[..len + 1] };
            check(slice)
            // (0..items.len()).any(|n| {
            //     let mut items = items.clone();
            //     items.remove(n);
            //     check2(&items)
            // })
        })
        .count()
}

fn check_pof(x: &[i8]) -> Result<(), u8> {
    let state = match x.first_chunk::<2>().map(|[a, b]| a.cmp(b)).ψ() {
        Equal => return Err(0),
        Greater => false,
        Less => true,
    };
    // windows at home 😔
    for i in 1..x.len() as u8 - 1 {
        let [a, b] = util::nail(C! { &x[i as usize..=i as usize ] });
        match state {
            true if !(1..=3).contains(&(b - a)) => return Err(i),
            false if !(1..=3).contains(&(a - b)) => return Err(i),
            _ => (),
        }
    }
    Ok(())
}
#[no_mangle]
pub fn p2(i: &str) -> impl Display {
    let mut items = [0; 8];

    i.行()
        .filter(|x| {
            let mut len = 0;
            let mut s = 0;
            for &b in *x {
                match b {
                    b' ' => {
                        C! { items[len] = s as i8 };
                        len += 1;
                        s = 0;
                    }
                    b => {
                        s = s * 10 + (b - b'0');
                    }
                }
            }
            C! { items[len] = s as i8 };
            let slice = C! { &items[..len + 1] };
            match check_pof(slice) {
                Ok(()) => true,
                Err(i) => {
                    let i = i as usize;
                    let mut place = [0i8; 7];
                    macro_rules! rmv {
                        ($i:expr, $si: expr) => {{
                            place[..$i].copy_from_slice(&slice[..$i]);
                            let put = &slice[$si..];
                            place[$i..$i + put.len()].copy_from_slice(put);
                            &place[..slice.len() - 1]
                        }};
                    }
                    check(rmv!(i, i + 1)) // [1, 2, >i<, 4, 5]
                        || check(rmv!(i + 1, i + 2)) // [1, 2, i, >4<, 5]
                        || (i > 0 && check(rmv!(i - 1, i))) // [1, >2<, i, 4, 5]
                }
            }
        })
        .count()
}

fn main() {
    // let mut s = String::new();
    // for i in 0..1280 {
    let i = include_str!("inp.txt");
    //     s.push_str(i);
    // }
    // std::fs::write("src/inp.txt", s);
    // println!("{}", p1(i));
    println!("{}", run(i));
    println!("{}", p2(i));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| run(i));
}

#[bench]
fn p21(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt").trim());
    b.iter(|| p2(i));
}
