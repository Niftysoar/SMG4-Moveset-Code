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

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;

    // Rising movement + initial flags
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }

    // Multi-hits from frame 5 → 25
    for i in (5..25).step_by(3) {
        frame(lua_state, i as f32);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"),
                1.0, 367, 100, 60, 0,
                6.0, 0.0, 8.0, 0.0,
                Some(0.0), Some(14.0), Some(0.0),
                1.0, 0.8,
                *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS,
                false, 0, 0.0, 0,
                true, false, false, false, true,
                *COLLISION_SITUATION_MASK_GA,
                *COLLISION_CATEGORY_MASK_ALL,
                *COLLISION_PART_MASK_ALL,
                false,
                Hash40::new("collision_attr_normal"),
                *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH,
                *ATTACK_REGION_BODY
            );
        }
        wait(lua_state, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }

    // Final strong hit at frame 28
    frame(lua_state, 28.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 89, 85, 0, 86, 6.0, 0.0, 14.0, 0.0, Some(0.0), Some(20.0), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }

    frame(lua_state, 33.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mariod_special_h03"));
    }
}

pub fn install() {
    Agent::new("mariod")
        .game_acmd("game_specialhismg4", game_specialhi, High)
        // .effect_acmd("effect_specialhismg4", effect_attackhi4, High)
        .sound_acmd("sound_specialhismg4", sound_specialhi, High)
        .game_acmd("game_specialairhismg4", game_specialhi, High)
        // .effect_acmd("effect_specialairhismg4", effect_attackhi4, High)
        .sound_acmd("sound_specialairhismg4", sound_specialhi, High)

        .install();
}