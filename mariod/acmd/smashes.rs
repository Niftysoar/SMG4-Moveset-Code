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

// ********************
// UP SMASH
// ********************
unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;

    frame(lua_state, 5.0);
    if macros::is_excute(agent) {
        // Hold / Charge phase
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }

    frame(lua_state, 6.0);
    if macros::is_excute(agent) {
        // Strong circular hitbox near the head
        macros::ATTACK(agent, 0, 0, Hash40::new("top"),
            14.0, 88, 100, 0, 40,   // Damage, Angle, KBG, BKB, FKB
            5.5, 0.0, 8.0, 0.0,    // Size + position (centered above)
            None, None, None,
            1.2, 1.0,
            *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS,
            false, 0, 0.0, 0,
            true, false, false, false, true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_HEAD
        );

        // Secondary arc hitbox (covers circular sweep)
        macros::ATTACK(agent, 1, 0, Hash40::new("top"),
            12.0, 90, 95, 0, 40,
            4.5, 0.0, 5.0, -3.0,   // behind offset
            None, None, None,
            1.1, 1.0,
            *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS,
            false, 0, 0.0, 0,
            true, false, false, false, true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_HEAD
        );
    }

    frame(lua_state, 20.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("head"), 6, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -3, 3, 0, -90, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.5);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 20, 0, 0, 0, 0, 1, false, 1);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
}

unsafe extern "C" fn sound_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start"));
        macros::PLAY_SE(agent, Hash40::new("vc_mariod_attack06"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_smashswing_03"));
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_l"));
    }
}

// ********************
// DOWN SMASH
// ********************
unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;

    frame(lua_state, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }

    // Spin multihits from frame 6 to 20
    for i in (6..40).step_by(3) {
        frame(lua_state, i as f32);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"),
                1.0, 120, 15, 0, 25,
                4.2, 0.0, 3.5, 12.0,   // right side
                Some(0.0), Some(3.5), Some(-12.0), // left side
                0.5, 1.0,
                *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS,
                false, 0, 0.0, 0,
                true, false, false, false, true,
                *COLLISION_SITUATION_MASK_GA,
                *COLLISION_CATEGORY_MASK_ALL,
                *COLLISION_PART_MASK_ALL,
                false,
                Hash40::new("collision_attr_normal"),
                *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH,
                *ATTACK_REGION_KICK
            );
        }
        wait(lua_state, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }

    // Final strong hit (frame 20)
    frame(lua_state, 40.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"),
            5.0, 30, 95, 0, 50,
            5.5, 0.0, 3.5, 13.0,   // right side
            Some(0.0), Some(3.5), Some(-13.0), // left side
            1.5, 1.0,
            *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS,
            false, 0, 0.0, 0,
            true, false, false, false, true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK
        );
    }

    frame(lua_state, 45.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("toel"), 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start"));
        macros::PLAY_SE(agent, Hash40::new("vc_mariod_attack07"));
    }
}

pub fn install() {
    Agent::new("mariod")
        .game_acmd("game_attackhi4smg4", game_attackhi4, High)
        .effect_acmd("effect_attackhi4smg4", effect_attackhi4, High)
        .sound_acmd("sound_attackhi4smg4", sound_attackhi4, High)
    
        .game_acmd("game_attacklw4smg4", game_attacklw4, High)
        .effect_acmd("effect_attacklw4smg4", effect_attacklw4, High)
        .sound_acmd("sound_attacklw4smg4", sound_attacklw4, High)

        .install();
}