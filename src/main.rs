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
use atools::CollectArray;
pub use util::prelude::*;

const SIZE: usize = 50;
// enum Bloc {
//     Wall,
//     Block,
//     Space,
// }

// impl Bloc {
//     fn push((x, y): (usize, usize), dir: Dir, grid: &mut [[Bloc; SIZE]], commit: bool) -> bool {
//         match dir {
//             Dir::N => match grid[y - 1][x] {},
//             Dir::E => todo!(),
//             Dir::S => todo!(),
//             Dir::W => todo!(),
//         }
//     }
// }
// impl std::fmt::Debug for Bloc {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Wall => write!(f, "##"),
//             Block => write!(f, "[]"),
//             Space => write!(f, ".."),
//         }
//     }
// }
// use Bloc::*;
pub unsafe fn p2(i: &str) -> impl Display {
    let i = i.as_bytes();
    let bot = memchr::memchr(b'@', i).ψ();
    let (mut x, mut y) = ((bot % (SIZE + 1)) * 2, bot / (SIZE + 1));
    let grid = i[..(SIZE + 1) * SIZE]
        .array_chunks::<{ SIZE + 1 }>()
        .flat_map(|x| {
            x.iter().take(SIZE).copied().flat_map(|x| match x {
                b'#' => [x; 2],
                b'O' => *b"[]",
                b'@' | b'.' => *b"..",
                _ => unreachable!(),
            })
        })
        .collect::<Vec<_>>()
        .leak()
        .as_chunks_unchecked_mut::<{ SIZE * 2 }>();
    // for y in 0..SIZE {
    //     for x in 0..SIZE * 2 {
    //         if (px, py) == (x, y) {
    //             print!("@");
    //         } else {
    //             print!("{}", grid[y][x] as char);
    //         }
    //     }
    //     println!();
    // }
    // println!("{grid/:?}");
    // let grid = i[..(SIZE + 1) * SIZE]
    //     .to_vec()
    //     .leak()
    //     .as_chunks_unchecked_mut::<{ SIZE + 1 }>();
    // grid[y][x * 2] = b'.';
    let i = &i[((SIZE + 1) * SIZE) + 1..];
    fn push((x, y): (usize, usize), dir: Dir, grid: &mut [[u8; SIZE * 2]], commit: bool) -> bool {
        match dir {
            Dir::N => match [grid[y - 1][x], grid[y - 1][x + 1]] {
                [_, b'#'] | [b'#', _] => {}
                [b'.', b'.'] => {
                    if commit {
                        grid[y - 1][x] = b'[';
                        grid[y - 1][x + 1] = b']';
                        grid[y][x] = b'.';
                        grid[y][x + 1] = b'.';
                    }
                    return true;
                    // swap(&mut grid[y - 1][x], &mut grid[y][x]),
                }
                [b']', b'['] => {
                    let val = push((x - 1, y - 1), dir, grid, false)
                        && push((x + 1, y - 1), dir, grid, false);
                    if commit && val {
                        push((x - 1, y - 1), dir, grid, commit);
                        push((x + 1, y - 1), dir, grid, commit);
                        grid[y - 1][x] = b'[';
                        grid[y - 1][x + 1] = b']';
                        grid[y][x] = b'.';
                        grid[y][x + 1] = b'.';
                    }
                    return val;
                }
                [b']', b'.'] => {
                    let val = push((x - 1, y - 1), dir, grid, commit);
                    if commit && val {
                        grid[y - 1][x] = b'[';
                        grid[y - 1][x + 1] = b']';
                        grid[y][x] = b'.';
                        grid[y][x + 1] = b'.';
                    }
                    return val;
                }
                [b'.', b'['] => {
                    let val = push((x + 1, y - 1), dir, grid, commit);
                    if commit && val {
                        grid[y - 1][x] = b'[';
                        grid[y - 1][x + 1] = b']';
                        grid[y][x] = b'.';
                        grid[y][x + 1] = b'.';
                    }
                    return val;
                }
                // "simple" case
                [b'[', b']'] => {
                    let val = push((x, y - 1), dir, grid, commit);
                    if val && commit {
                        grid[y - 1][x] = b'[';
                        grid[y - 1][x + 1] = b']';
                        grid[y][x] = b'.';
                        grid[y][x + 1] = b'.';
                    }
                    return val;
                }
                x => unreachable!("{x:?}"),
            },
            Dir::S => match [grid[y + 1][x], grid[y + 1][x + 1]] {
                [_, b'#'] | [b'#', _] => {}
                [b'.', b'.'] => {
                    if commit {
                        grid[y + 1][x] = b'[';
                        grid[y + 1][x + 1] = b']';
                        grid[y][x] = b'.';
                        grid[y][x + 1] = b'.';
                    }
                    return true;
                    // swap(&mut grid[y - 1][x], &mut grid[y][x]),
                }
                [b']', b'['] => {
                    let val = push((x - 1, y + 1), dir, grid, false)
                        && push((x + 1, y + 1), dir, grid, false);
                    if commit && val {
                        push((x - 1, y + 1), dir, grid, commit);
                        push((x + 1, y + 1), dir, grid, commit);
                        grid[y + 1][x] = b'[';
                        grid[y + 1][x + 1] = b']';
                        grid[y][x] = b'.';
                        grid[y][x + 1] = b'.';
                    }
                    return val;
                }
                [b']', b'.'] => {
                    let val = push((x - 1, y + 1), dir, grid, commit);
                    if commit && val {
                        grid[y + 1][x] = b'[';
                        grid[y + 1][x + 1] = b']';
                        grid[y][x] = b'.';
                        grid[y][x + 1] = b'.';
                    }
                    return val;
                }
                [b'.', b'['] => {
                    let val = push((x + 1, y + 1), dir, grid, commit);
                    if commit && val {
                        grid[y + 1][x] = b'[';
                        grid[y + 1][x + 1] = b']';
                        grid[y][x] = b'.';
                        grid[y][x + 1] = b'.';
                    }
                    return val;
                }
                [b'[', b']'] => {
                    let val = push((x, y + 1), dir, grid, commit);
                    if val && commit {
                        grid[y + 1][x] = b'[';
                        grid[y + 1][x + 1] = b']';
                        grid[y][x] = b'.';
                        grid[y][x + 1] = b'.';
                    }
                    return val;
                }
                x => unreachable!("{x:?}"),
            },
            Dir::E => match grid[y][x + 2] {
                b'.' => {
                    grid[y][x + 2] = b']';
                    grid[y][x + 1] = b'[';
                    grid[y][x] = b'.';
                    return true;
                    // swap(&mut grid[y - 1][x], &mut grid[y][x]),
                }
                b'[' => {
                    if push((x + 2, y), dir, grid, commit) {
                        grid[y][x + 2] = b']';
                        grid[y][x + 1] = b'[';
                        grid[y][x] = b'.';
                        return true;
                    }
                }
                b'#' => {}
                x => unreachable!("{}", x as char),
            },

            Dir::W => match grid[y][x - 1] {
                b'.' => {
                    grid[y][x - 1] = b'[';
                    grid[y][x] = b']';
                    grid[y][x + 1] = b'.';
                    return true;
                    // swap(&mut grid[y - 1][x], &mut grid[y][x]),
                }
                b']' => {
                    if push((x - 2, y), dir, grid, commit) {
                        grid[y][x - 1] = b'[';
                        grid[y][x] = b']';
                        grid[y][x + 1] = b'.';
                        return true;
                    }
                }
                b'#' => {}
                x => unreachable!("{}", x as char),
            },
        }
        false
    }
    for input in i {
        // println!("{}", *input as char);
        match input {
            b'<' => match grid[y][x - 1] {
                b'.' => x = x - 1,
                b'#' => (),
                b']' => {
                    if push((x - 2, y), Dir::W, grid, true) {
                        x = x - 1;
                    }
                }
                x => unreachable!("{}", x as char),
            },
            b'>' => match grid[y][x + 1] {
                b'.' => x = x + 1,
                b'#' => (),
                b'[' => {
                    if push((x + 1, y), Dir::E, grid, true) {
                        x = x + 1;
                    }
                }
                x => unreachable!("{}", x as char),
            },

            b'^' => match grid[y - 1][x] {
                b'.' => y = y - 1,
                b'#' => (),
                b']' => {
                    if push((x - 1, y - 1), Dir::N, grid, true) {
                        y = y - 1;
                    }
                }
                b'[' => {
                    if push((x, y - 1), Dir::N, grid, true) {
                        y = y - 1;
                    }
                }
                x => unreachable!("{}", x as char),
            },
            b'v' => match grid[y + 1][x] {
                b'.' => y = y + 1,
                b'#' => (),
                b'[' => {
                    if push((x, y + 1), Dir::S, grid, true) {
                        y = y + 1;
                    }
                }
                b']' => {
                    if push((x - 1, y + 1), Dir::S, grid, true) {
                        y = y + 1;
                    }
                }
                x => unreachable!("{}", x as char),
            },
            _ => {}
        }
        // grid[y][x] = b'@';
        // for row in &*grid {
        //     println!("{}", row.p());
        // }
        // grid[y][x] = b'.';
    }
    let mut sum = 0;
    for (row, y) in grid.into_iter().ι::<u32>() {
        for (col, x) in row.into_iter().ι::<u32>() {
            if *col == b'[' {
                sum += 100 * y + x
            }
        }
    }

    sum
}

#[no_mangle]
pub unsafe fn run(i: &str) -> impl Display {
    let i = i.as_bytes();
    let bot = memchr::memchr(b'@', i).ψ();
    let (mut x, mut y) = (bot % (SIZE + 1), bot / (SIZE + 1));
    let grid = i[..(SIZE + 1) * SIZE]
        .to_vec()
        .leak()
        .as_chunks_unchecked_mut::<{ SIZE + 1 }>();
    grid[y][x] = b'.';
    let i = &i[((SIZE + 1) * SIZE) + 1..];
    fn push((x, y): (usize, usize), dir: Dir, grid: &mut [[u8; SIZE + 1]]) -> bool {
        match dir {
            Dir::N => match grid[y - 1][x] {
                b'.' => {
                    grid[y - 1][x] = b'O';
                    grid[y][x] = b'.';
                    return true;
                    // swap(&mut grid[y - 1][x], &mut grid[y][x]),
                }
                b'O' => {
                    if push((x, y - 1), dir, grid) {
                        grid[y - 1][x] = b'O';
                        grid[y][x] = b'.';
                        return true;
                    }
                }
                b'#' => {}
                x => unreachable!("{}", x as char),
            },
            Dir::E => match grid[y][x + 1] {
                b'.' => {
                    grid[y][x + 1] = b'O';
                    grid[y][x] = b'.';
                    return true;
                    // swap(&mut grid[y - 1][x], &mut grid[y][x]),
                }
                b'O' => {
                    if push((x + 1, y), dir, grid) {
                        grid[y][x + 1] = b'O';
                        grid[y][x] = b'.';
                        return true;
                    }
                }
                b'#' => {}
                x => unreachable!("{}", x as char),
            },
            Dir::S => match grid[y + 1][x] {
                b'.' => {
                    grid[y + 1][x] = b'O';
                    grid[y][x] = b'.';
                    return true;
                    // swap(&mut grid[y - 1][x], &mut grid[y][x]),
                }
                b'O' => {
                    if push((x, y + 1), dir, grid) {
                        grid[y + 1][x] = b'O';
                        grid[y][x] = b'.';
                        return true;
                    }
                }
                b'#' => {}
                x => unreachable!("{}", x as char),
            },
            Dir::W => match grid[y][x - 1] {
                b'.' => {
                    grid[y][x - 1] = b'O';
                    grid[y][x] = b'.';
                    return true;
                    // swap(&mut grid[y - 1][x], &mut grid[y][x]),
                }
                b'O' => {
                    if push((x - 1, y), dir, grid) {
                        grid[y][x - 1] = b'O';
                        grid[y][x] = b'.';
                        return true;
                    }
                }
                b'#' => {}
                x => unreachable!("{}", x as char),
            },
        }
        false
    }
    for input in i {
        match input {
            b'<' => match grid[y][x - 1] {
                b'.' => x = x - 1,
                b'#' => (),
                b'O' => {
                    if push((x - 1, y), Dir::W, grid) {
                        x = x - 1;
                    }
                }
                x => unreachable!("{}", x as char),
            },
            b'>' => match grid[y][x + 1] {
                b'.' => x = x + 1,
                b'#' => (),
                b'O' => {
                    if push((x + 1, y), Dir::E, grid) {
                        x = x + 1;
                    }
                }
                x => unreachable!("{}", x as char),
            },

            b'^' => match grid[y - 1][x] {
                b'.' => y = y - 1,
                b'#' => (),
                b'O' => {
                    if push((x, y - 1), Dir::N, grid) {
                        y = y - 1;
                    }
                }
                x => unreachable!("{}", x as char),
            },
            b'v' => match grid[y + 1][x] {
                b'.' => y = y + 1,
                b'#' => (),
                b'O' => {
                    if push((x, y + 1), Dir::S, grid) {
                        y = y + 1;
                    }
                }
                x => unreachable!("{}", x as char),
            },
            _ => {}
        }
    }
    let mut sum = 0;
    for (row, y) in grid.into_iter().ι::<u32>() {
        for (col, x) in row.into_iter().ι::<u32>() {
            if *col == b'O' {
                sum += 100 * y + x
            }
        }
    }

    sum
}

fn main() {
    // (1..u32::MAX as u64).for_each(|a| assert_eq!(a.ilog10() + 1, digs(a)));
    // let mut s = String::new();
    // for i in 0..1280 {
    let i = include_str!("inp.txt");
    //     s.push_str(i);i
    // }w
    // std::fs::write("src/inp.txt", s);
    #[allow(unused_unsafe)]
    println!("{}", unsafe { p2(i) });
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { run(i) });
}
