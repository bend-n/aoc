#![feature(generic_const_exprs, portable_simd, stmt_expr_attributes)]
use std::{hint::assert_unchecked, ops::Range, simd::prelude::*};
#[unsafe(no_mangle)]
pub unsafe fn run(x: &[u8]) -> u64 {
    let i = unsafe { x.as_chunks_unchecked::<{ 141 + 1 }>() };
    #[rustfmt::skip]
    static  LUT: [(usize, usize); 141] = [(0, 0), (0, 0), (70, 70), (0, 0), (69, 71), (0, 0), (68, 72), (0, 0), (67, 73), (0, 0), (66, 74), (0, 0), (65, 75), (0, 0), (64, 76), (0, 0), (63, 77), (0, 0), (62, 78), (0, 0), (61, 79), (0, 0), (60, 80), (0, 0), (59, 81), (0, 0), (58, 82), (0, 0), (57, 83), (0, 0), (56, 84), (0, 0), (55, 85), (0, 0), (54, 86), (0, 0), (53, 87), (0, 0), (52, 88), (0, 0), (51, 89), (0, 0), (50, 90), (0, 0), (49, 91), (0, 0), (48, 92), (0, 0), (47, 93), (0, 0), (46, 94), (0, 0), (45, 95), (0, 0), (44, 96), (0, 0), (43, 97), (0, 0), (42, 98), (0, 0), (41, 99), (0, 0), (40, 100), (0, 0), (39, 101), (0, 0), (38, 102), (0, 0), (37, 103), (0, 0), (36, 104), (0, 0), (35, 105), (0, 0), (34, 106), (0, 0), (33, 107), (0, 0), (32, 108), (0, 0), (31, 109), (0, 0), (30, 110), (0, 0), (29, 111), (0, 0), (28, 112), (0, 0), (27, 113), (0, 0), (26, 114), (0, 0), (25, 115), (0, 0), (24, 116), (0, 0), (23, 117), (0, 0), (22, 118), (0, 0), (21, 119), (0, 0), (20, 120), (0, 0), (19, 121), (0, 0), (18, 122), (0, 0), (17, 123), (0, 0), (16, 124), (0, 0), (15, 125), (0, 0), (14, 126), (0, 0), (13, 127), (0, 0), (12, 128), (0, 0), (11, 129), (0, 0), (10, 130), (0, 0), (9, 131), (0, 0), (8, 132), (0, 0), (7, 133), (0, 0), (6, 134), (0, 0), (5, 135), (0, 0), (4, 136), (0, 0), (3, 137), (0, 0), (2, 138), (0, 0), (1, 139)];

    for n in (1..141).rev().step_by(2) {
        let lem = &i[n];
        println!("{n}: {}", lem.iter().filter(|x| **x == b'^').count());
        // assert_eq!((start, end), LUT[n]);
        // assert_eq!(LUT[n], (start, end));
        // println!("({start}, {end}),");
    }
    // println!("{LUT:?}");
    assert_unchecked(i.len() == 142);
    let mut v = [1u64; 141];
    let split = Simd::splat(b'^');
    let mut f = #[inline(always)]
    |(star, end): (usize, usize), line: &[u8; 142]| {
        let mut f = #[inline(always)]
        |mut mask: u64, o: usize| {
            while mask != 0 {
                let x = mask.trailing_zeros() as usize + o;
                mask ^= mask & mask.wrapping_neg();
                *v.get_unchecked_mut(x) = *v.get_unchecked(x - 1) + *v.get_unchecked(x + 1);
            }
        };
        // let (star, end) = LUT[ix];
        if (end - star) < 64 {
            let mask = u8x64::load_or_default(&line[star..])
                .simd_eq(split)
                .to_bitmask();
            f(mask, star);
        } else if (end - star) < 128 {
            let mask = u8x64::load_or_default(&line[star..])
                .simd_eq(split)
                .to_bitmask();
            f(mask, star);
            let mask = u8x64::load_or_default(&line[star + 64..])
                .simd_eq(split)
                .to_bitmask();
            f(mask, star + 64);
        } else {
            let mask = u8x64::load_or_default(&line[star..])
                .simd_eq(split)
                .to_bitmask();
            f(mask, star);
            let mask = u8x64::load_or_default(&line[star + 64..])
                .simd_eq(split)
                .to_bitmask();
            f(mask, star + 64);
            let mask = u8x64::load_or_default(&line[star + 128..])
                .simd_eq(split)
                .to_bitmask();
            f(mask, star + 128);
        }
        // if ix > 65 {
        //     let mask = u8x64::load_or_default(&line[..39])
        //         .simd_eq(split)
        //         .to_bitmask();
        //     f(mask, 0);
        // }
        // let middle = u8x64::load_or_default(&line[39..])
        //     .simd_eq(split)
        //     .to_bitmask();
        // f(middle, 39);
        // if ix > 65 {
        //     let end = u8x64::load_or_default(&line[39 + 64..])
        //         .simd_eq(split)
        //         .to_bitmask();
        //     f(end, 39 + 64);
        // }
    };

    f(const { LUT[140] }, &i[140]);
    f(const { LUT[138] }, &i[138]);
    f(const { LUT[136] }, &i[136]);
    f(const { LUT[134] }, &i[134]);
    f(const { LUT[132] }, &i[132]);
    f(const { LUT[130] }, &i[130]);
    f(const { LUT[128] }, &i[128]);
    f(const { LUT[126] }, &i[126]);
    f(const { LUT[124] }, &i[124]);
    f(const { LUT[122] }, &i[122]);
    f(const { LUT[120] }, &i[120]);
    f(const { LUT[118] }, &i[118]);
    f(const { LUT[116] }, &i[116]);
    f(const { LUT[114] }, &i[114]);
    f(const { LUT[112] }, &i[112]);
    f(const { LUT[110] }, &i[110]);
    f(const { LUT[108] }, &i[108]);
    f(const { LUT[106] }, &i[106]);
    f(const { LUT[104] }, &i[104]);
    f(const { LUT[102] }, &i[102]);
    f(const { LUT[100] }, &i[100]);
    f(const { LUT[98] }, &i[98]);
    f(const { LUT[96] }, &i[96]);
    f(const { LUT[94] }, &i[94]);
    f(const { LUT[92] }, &i[92]);
    f(const { LUT[90] }, &i[90]);
    f(const { LUT[88] }, &i[88]);
    f(const { LUT[86] }, &i[86]);
    f(const { LUT[84] }, &i[84]);
    f(const { LUT[82] }, &i[82]);
    f(const { LUT[80] }, &i[80]);
    f(const { LUT[78] }, &i[78]);
    f(const { LUT[76] }, &i[76]);
    f(const { LUT[74] }, &i[74]);
    f(const { LUT[72] }, &i[72]);
    f(const { LUT[70] }, &i[70]);
    f(const { LUT[68] }, &i[68]);
    f(const { LUT[66] }, &i[66]);
    f(const { LUT[64] }, &i[64]);
    f(const { LUT[62] }, &i[62]);
    f(const { LUT[60] }, &i[60]);
    f(const { LUT[58] }, &i[58]);
    f(const { LUT[56] }, &i[56]);
    f(const { LUT[54] }, &i[54]);
    f(const { LUT[52] }, &i[52]);
    f(const { LUT[50] }, &i[50]);
    f(const { LUT[48] }, &i[48]);
    f(const { LUT[46] }, &i[46]);
    f(const { LUT[44] }, &i[44]);
    f(const { LUT[42] }, &i[42]);
    f(const { LUT[40] }, &i[40]);
    f(const { LUT[38] }, &i[38]);
    f(const { LUT[36] }, &i[36]);
    f(const { LUT[34] }, &i[34]);
    f(const { LUT[32] }, &i[32]);
    f(const { LUT[30] }, &i[30]);
    f(const { LUT[28] }, &i[28]);
    f(const { LUT[26] }, &i[26]);
    f(const { LUT[24] }, &i[24]);
    f(const { LUT[22] }, &i[22]);
    f(const { LUT[20] }, &i[20]);
    f(const { LUT[18] }, &i[18]);
    f(const { LUT[16] }, &i[16]);
    f(const { LUT[14] }, &i[14]);
    f(const { LUT[12] }, &i[12]);
    f(const { LUT[10] }, &i[10]);
    f(const { LUT[8] }, &i[8]);
    f(const { LUT[6] }, &i[6]);
    f(const { LUT[4] }, &i[4]);
    f(const { LUT[2] }, &i[2]);
    // for n in (1..141).rev().step_by(2) {
    //     println!("f({n}, &i[{n}]);");
    //     f(n, &i[n]);
    // }

    *v.get_unchecked(70)
}
