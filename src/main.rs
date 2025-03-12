#![allow(
    confusable_idents,
    uncommon_codepoints,
    non_upper_case_globals,
    internal_features,
    mixed_script_confusables,
    static_mut_refs,
    incomplete_features,
    redundant_semicolons
)]
#![feature(
    stdarch_x86_avx512,
    impl_trait_in_bindings,
    iter_partition_in_place,
    iter_chain,
    slice_swap_unchecked,
    generic_const_exprs,
    iter_array_chunks,
    slice_from_ptr_range,
    if_let_guard,
    once_cell_get_mut,
    iter_collect_into,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    vec_into_raw_parts,
    try_blocks,
    portable_simd,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    import_trait_associated_functions,
    core_intrinsics
)]
extern crate test;
pub mod util;

use atools::prelude::*;
use lower::apply;
use memchr::memmem;
use regex::bytes::Regex;
use std::simd::prelude::*;
pub use util::prelude::*;

#[allow(warnings)]
type u32x3 = Simd<u32, 3>;

#[no_mangle]
pub fn p1(x: &'static str) -> impl Display {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Effect {
        Shield,
        Poison,
        Recharge,
    }
    #[derive(Clone, Debug)]
    struct Game {
        effects: Vec<(Effect, u32)>,
        player: [u32; 3],
        boss: [u32; 2],
    }
    use Effect::*;
    static mut min: u64 = !0;
    fn mini(x: u64) {
        unsafe {
            if min > x {
                println!("{x}");
                min = x;
            }
        }
    }
    #[apply(saturating)]
    fn go(mut game: Game, player: bool, spent: u64) {
        if spent > unsafe { min } {
            return;
        }

        for (e, _) in &mut game.effects {
            match e {
                Effect::Shield => game.player[1] = 7,
                Effect::Poison => game.boss[0] -= 3,
                Effect::Recharge => game.player[2] += 101,
            }
        }
        game.effects
            .iter_mut()
            .for_each(|(_, x)| *x = x.saturating_sub(1));
        game.effects
            .extract_if(.., |x| x.1 == 0)
            .for_each(|(e, _)| match e {
                Effect::Shield => game.player[1] = 0,
                _ => (),
            });

        if game.boss[0] == 0 {
            // win
            return mini(spent);
        }

        // play boss turn
        if !player {
            // player[hp] -= boss[damage] - player[armor]
            game.player[0] = game.player[0] - (game.boss[1] - game.player[1]);
            if game.player[0] == 0 {
                // ded
                return;
            }
            return go(game, true, spent);
        }

        // game.player[0] -= 1;
        // if game.player[0] == 0 {
        //     // ded
        //     return;
        // }

        if game.player[2] < 53 {
            // ded
            return;
        }

        if game.player[2] >= 53 {
            // missile
            let mut game = game.clone();
            game.player[2] -= 53;
            game.boss[0] -= 4;
            if game.boss[0] == 0 {
                return mini(spent + 53);
            }
            go(game, false, spent + 53);
        }

        if game.player[2] >= 73 {
            // drain
            let mut game = game.clone();
            game.player[2] -= 73;
            game.player[0] += 2;
            game.boss[0] -= 2;
            if game.boss[0] == 0 {
                return mini(spent + 73);
            }
            go(game, false, spent + 73);
        }

        if game.player[2] >= 113 && !game.effects.iter().l().contains(&Shield) {
            // shield
            let mut game = game.clone();
            game.effects.push((Shield, 6));
            game.player[2] -= 113;
            go(game, false, spent + 113);
        }

        if game.player[2] >= 173 && !game.effects.iter().l().contains(&Poison) {
            // poison
            let mut game = game.clone();
            game.effects.push((Poison, 6));
            game.player[2] -= 173;
            go(game, false, spent + 173);
        }

        if game.player[2] >= 229 && !game.effects.iter().l().contains(&Recharge) {
            // recharge
            let mut game = game.clone();
            game.effects.push((Recharge, 5));
            game.player[2] -= 229;
            go(game, false, spent + 229);
        }

        // Magic Missile costs 53 mana. It instantly does 4 damage.
        // Drain costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.
        // Shield costs 113 mana. It starts an effect that lasts for 6 turns. While it is active, your armor is increased by 7.
        // Poison costs 173 mana. It starts an effect that lasts for 6 turns. At the start of each turn while it is active, it deals the boss 3 damage.
        // Recharge costs 229 mana. It starts an effect that lasts for 5 turns. At the start of each turn while it is active, it gives you 101 new mana.
    }
    // go(
    //     Game {
    //         effects: vec![],
    //         player: [10, 0, 250],
    //         boss: [14, 8],
    //     },
    //     true,
    //     0,
    // );
    go(
        Game {
            effects: vec![],
            player: [50, 0, 500],
            boss: [55, 8],
        },
        true,
        0,
    );
    unsafe { min }
}

fn main() {
    unsafe { println!("{}", p1(include_str!("inp.txt"))) };
}

#[bench]
fn benc(b: &mut test::Bencher) {
    let i = boxd(include_str!("inp.txt"));
    b.iter(|| unsafe { p1(i) });
}
