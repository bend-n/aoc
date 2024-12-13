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
    if_let_guard,
    get_many_mut,
    maybe_uninit_uninit_array,
    once_cell_get_mut,
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
const SIZE: usize = 140;

fn two([a, b]: [u8; 2]) -> i64 {
    (a - b'0') as i64 * 10 + (b - b'0') as i64
}

#[no_mangle]
pub fn run(i: &str) -> impl Display {
    let mut i = i.as_bytes();
    // let i = i.as_chunks_unchecked::<{ SIZE + 1 }>();
    // let get = |x, y| (x < SIZE && y < SIZE).then(|| i[y][x]);
    let mut sum = 0;
    loop {
        i.skip_n("button a: x+");
        let a_x = two(i.rd::<2>().ψ());
        i.skip_n(", y+");
        let a_y = two(i.rd::<2>().ψ());
        i.skip_n("\nbutton b: x+");
        let b_x = two(i.rd::<2>().ψ());
        i.skip_n(", y+");
        let b_y = two(i.rd::<2>().ψ());
        i.skip_n("\nprize: x=");
        let p_x: i64 = reading::until(&mut i, b',');
        i.skip_n(" y=");
        let p_y: i64 = reading::until(&mut i, b'\n');

        #[inline]
        fn dmod(a: i64, b: i64) -> (i64, i64) {
            unsafe {
                (
                    core::intrinsics::unchecked_div(a, b),
                    core::intrinsics::unchecked_rem(a, b),
                )
            }
        }
        // a_x * α + b_x * β = p_x
        // a_y * α + b_y * β = p_y
        let (β, ok) = dmod(
            a_y * p_x - a_x * p_y, //
            a_y * b_x - a_x * b_y,
        );
        if ok == 0 {
            let α = unsafe {
                core::intrinsics::unchecked_div(
                    b_y * p_x - b_x * p_y, //
                    a_x * b_y - a_y * b_x,
                )
            };
            sum += 3 * α + β;
        }

        if i.is_empty() {
            break;
        }
        i.skip(1);
    }
    sum
}

fn main() {
    // (1..u32::MAX as u64).for_each(|a| assert_eq!(a.ilog10() + 1, digs(a)));
    // let mut s = String::new();
    // for i in 0..1280 {
    let i = include_str!("inp.txt");
    //     s.push_str(i);i
    // }
    // std::fs::write("src/inp.txt", s);
    #[allow(unused_unsafe)]
    println!("{}", unsafe { run(i) });
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { run(i) });
}
