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

#[no_mangle]
fn check(x: &[i8]) -> bool {
    if x.len() > 8 {
        unsafe { std::hint::unreachable_unchecked() }
    }
    let state = unsafe { x.first_chunk::<2>().map(|[a, b]| a < b).unwrap_unchecked() };
    x.array_windows::<2>().all(|[a, b]| match state {
        true if !(1..=3).contains(&(b - a)) => return false,
        false if !(1..=3).contains(&(a - b)) => return false,
        _ => true,
    })
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

#[no_mangle]
fn check_pof(x: &[i8]) -> Result<(), u8> {
    if x.len() > 8 {
        unsafe { std::hint::unreachable_unchecked() }
    }
    let state = match unsafe {
        x.first_chunk::<2>()
            .map(|[a, b]| a.cmp(b))
            .unwrap_unchecked()
    } {
        std::cmp::Ordering::Equal => return Err(0),
        std::cmp::Ordering::Greater => false,
        std::cmp::Ordering::Less => true,
    };
    // windows at home 😔
    for i in 1..x.len() as u8 - 1 {
        let [a, b] = util::nail(&x[i as usize..]);
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
