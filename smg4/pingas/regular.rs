use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*,
        hash40
	},
    smash_script::*,
    smashline::*
};

use smashline::Priority::*;
use super::*;

unsafe extern "C" fn game_regular(agent: &mut L2CAgentBase) {

    // First roll for the super rare insta-kill (1/100 chance)
    let rare_roll = sv_math::rand(hash40("rng"), 100); // 0–99
    let rng = if rare_roll == 0 {
        99 // special code for insta kill
    } else {
        sv_math::rand(hash40("rng"), 6) // 0–5 (6 regular effects)
    };

    // store chosen attributes before the attacks
    let (dmg, angle, kbg, bkb, attr, sfx) = match rng {
        // Normal projectile
        0 => (5.0, 65, 40, 60, Hash40::new("collision_attr_normal"), *COLLISION_SOUND_ATTR_MARIOD_CAPSULE),
        // Nothing
        1 => (0.0, 361, 0, 0, Hash40::new("collision_attr_normal"), *COLLISION_SOUND_ATTR_NONE),
        // Fire
        2 => (12.0, 45, 90, 70, Hash40::new("collision_attr_fire"), *COLLISION_SOUND_ATTR_MARIOD_CAPSULE),
        // Ice
        3 => (8.0, 361, 60, 50, Hash40::new("collision_attr_ice"), *COLLISION_SOUND_ATTR_MARIOD_CAPSULE),
        // Dizzy
        4 => (5.0, 361, 60, 60, Hash40::new("collision_attr_paralyze"), *COLLISION_SOUND_ATTR_ELEC),
        // Flip
        5 => (1.0, 361, 0, 0, Hash40::new("collision_attr_turn"), *COLLISION_SOUND_ATTR_MARIOD_MANT),
        // Insta-kill (rare)
        99 => (999.0, 361, 999, 999, Hash40::new("collision_attr_normal"), *COLLISION_SOUND_ATTR_BAT),
        // Fallback
        _ => (5.0, 65, 40, 60, Hash40::new("collision_attr_normal"), *COLLISION_SOUND_ATTR_MARIOD_CAPSULE),
    };

    // Initial spawn
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), dmg, angle as u64, kbg as u64, 0, bkb,
            1.7, 0.0, 1.7, 0.0, Some(0.0), Some(-1.7), Some(0.0), 1.0, 1.0,
            *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0,
            true, false, false, false, false,
            *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL,
            false, attr, *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_OBJECT);
    }

    // Reapply hitbox, slightly weaker
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), dmg * 0.8, angle as u64, kbg as u64, 0, bkb - 5,
            1.7, 0.0, 1.7, 0.0, Some(0.0), Some(-1.7), Some(0.0), 1.0, 1.0,
            *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0,
            true, false, false, false, false,
            *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL,
            false, attr, *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_OBJECT);
    }

    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), dmg * 0.6, angle as u64, kbg as u64, 0, bkb - 10,
            1.7, 0.0, 1.7, 0.0, Some(0.0), Some(-1.7), Some(0.0), 1.0, 1.0,
            *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.0, 0.0, 0,
            true, false, false, false, false,
            *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL,
            false, attr, *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_OBJECT);
    }
}

pub fn install() {
    Agent::new("mariod_drcapsule")
    .set_costume([112, 113, 114, 115, 116, 117, 118, 119].to_vec())
        .acmd("game_regularsmg4", game_regular, High)

        .install();
}