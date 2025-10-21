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

//Forward Smash Charge Effect
unsafe extern "C" fn effect_attacks4charge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    for _ in 0..12 {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 6, 6, -3.0, 12, 90, -90, 0.6, false, 0.5);
            macros::LAST_EFFECT_SET_RATE(agent, 2.5);
        }
        wait(agent.lua_state_agent, 4.0);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("handr"), 2.0, 3, 0, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, true);
        wait(agent.lua_state_agent, 1.0);
    }
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 4.0);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("handr"), 2.0, 3, 0, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, true);
        wait(agent.lua_state_agent, 1.0);
    }
}

//Forward Smash Charge Sound
unsafe extern "C" fn sound_attacks4charge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_smash_start"));
    }
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
        }
        wait(agent.lua_state_agent, 18.0);
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
        wait(agent.lua_state_agent, 1.0);
    }
}

unsafe extern "C" fn game_attacks4s(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 15.0, 361, 110, 0, 22, 4.0, 1.4, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("bust"), 16.5, 361, 105, 0, 22, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 18.0, 361, 105, 0, 22, 5.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_attacks4s(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 6, 6, -3.0, 12, 90, -90, 0.6, false, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 2.5);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 6, 6, -3.0, 12, 90, -90, 0.6, false, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 2.5);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 6, 6, -3.0, 12, 90, -90, 0.6, false, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 2.5);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7, -1, 0, -2, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_attacks4s(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start"));
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_mario_attack05"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("se_common_swing_09"));
        macros::PLAY_SE_REMAIN(agent, Hash40::new("se_common_punch_kick_swing_l"));
    }
}

//Forward Smash Hi ACMD
unsafe extern "C" fn game_attacks4hi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 15.0, 361, 110, 0, 25, 3.0, 1.4, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("bust"), 16.5, 361, 105, 0, 25, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 18.0, 361, 105, 0, 25, 5.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Hi Effect
unsafe extern "C" fn effect_attacks4hi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7, -1, -15, -2, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Forward Smash Lw ACMD
unsafe extern "C" fn game_attacks4lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 15.0, 361, 110, 0, 25, 3.0, 1.4, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("bust"), 16.5, 361, 105, 0, 25, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 18.0, 361, 105, 0, 25, 5.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Lw Effect
unsafe extern "C" fn effect_attacks4lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7, -1, 18, -2, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

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
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 88, 100, 0, 40, 5.5, 0.0, 8.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);

        // Secondary arc hitbox (covers circular sweep)
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 90, 95, 0, 40, 4.5, 0.0, 5.0, -3.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
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
                4.2, 0.0, 3.5, 8.0,   // right side
                Some(0.0), Some(3.5), Some(0.0), // left side
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
            5.5, 0.0, 3.5, 9.0,   // right side
            Some(0.0), Some(3.5), Some(0.0), // left side
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
        .effect_acmd("effect_attacks4chargesmg4", effect_attacks4charge, High) 
        .sound_acmd("sound_attacks4chargesmg4", sound_attacks4charge, High) 

        .game_acmd("game_attacks4smg4", game_attacks4s, High)  
        .effect_acmd("effect_attacks4smg4", effect_attacks4s, High) 
        .sound_acmd("sound_attacks4smg4", sound_attacks4s, High) 
        .game_acmd("game_attacks4hismg4", game_attacks4hi, High)  
        .effect_acmd("effect_attacks4hismg4", effect_attacks4hi, High)  
        .sound_acmd("sound_attacks4hismg4", sound_attacks4s, High) 
        .game_acmd("game_attacks4lwsmg4", game_attacks4lw, High)  
        .effect_acmd("effect_attacks4lwsmg4", effect_attacks4lw, High)  
        .sound_acmd("sound_attacks4lwsmg4", sound_attacks4s, High) 

        .game_acmd("game_attackhi4smg4", game_attackhi4, High)
        .effect_acmd("effect_attackhi4smg4", effect_attackhi4, High)
        .sound_acmd("sound_attackhi4smg4", sound_attackhi4, High)
    
        .game_acmd("game_attacklw4smg4", game_attacklw4, High)
        .effect_acmd("effect_attacklw4smg4", effect_attacklw4, High)
        .sound_acmd("sound_attacklw4smg4", sound_attacklw4, High)

        .install();
}