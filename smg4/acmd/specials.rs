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

use crate::imports::imports_agent::*;
use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        if true{
            let x_speed = PostureModule::lr(agent.module_accessor) * ControlModule::get_stick_x(agent.module_accessor) * 0.75;
            macros::SET_SPEED_EX(agent, x_speed, 3.35, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		}
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }

    for i in (5..25).step_by(3) {
        frame(agent.lua_state_agent, i as f32);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 100, 60, 0, 6.0, 0.0, 8.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }

    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 89, 85, 0, 86, 6.0, 0.0, 14.0, 0.0, Some(0.0), Some(20.0), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }

    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specialhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mariod_special_h03"));
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        println!("[smashline_smg4::special_lw] spawn {}",crate::smg4::FIGHTER_MARIO_GENERATE_ARTICLE_DECOY);
        ArticleModule::generate_article(agent.module_accessor, crate::smg4::FIGHTER_MARIO_GENERATE_ARTICLE_DECOY, false, -1);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        ArticleModule::change_status_exist(agent.module_accessor, crate::smg4::FIGHTER_MARIO_GENERATE_ARTICLE_DECOY, crate::smg4::DECOY_STATUS_FLY);
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let situation_kind = StatusModule::situation_kind(agent.module_accessor);
    let facing_right = PostureModule::lr(agent.module_accessor) > 0.0;

    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 6, 11, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        // fire burst direction
        let yaw = if facing_right { 45.0 } else { -45.0 };

        if situation_kind == *SITUATION_KIND_GROUND {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.2, 0.2, 0.2);
        }
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flame"), Hash40::new("havel"), 1.0, 0, 0, 0, 0, 0, 0.5, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_bomb_a"), Hash40::new("handl"), 1.0, 0, 0, 0, 0, 0, 0.25, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mariod_special_l01"));
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_mariod_009"));
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mariod_special_l02"));
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
    }
}

unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

unsafe extern "C" fn game_speciallwblank(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        println!("[smashline_smg4::special_lw] nothing to spawn");
    }
}

unsafe extern "C" fn effect_speciallwblank(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn sound_speciallwblank(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn expression_speciallwblank(agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("mariod")
    .set_costume([112, 113, 114, 115, 116, 117, 118, 119].to_vec())
        .game_acmd("game_specialhi", game_specialhi, High)
        .effect_acmd("effect_specialhi", effect_specialhi, High)
        .sound_acmd("sound_specialhi", sound_specialhi, High)
        .game_acmd("game_specialairhi", game_specialhi, High)
        .effect_acmd("effect_specialairhi", effect_specialhi, High)
        .sound_acmd("sound_specialairhi", sound_specialhi, High)

        .game_acmd("game_speciallw", game_speciallw, High)
        .game_acmd("game_specialairlw", game_speciallw, High)
        .effect_acmd("effect_speciallw", effect_speciallw, High)
        .effect_acmd("effect_specialairlw", effect_speciallw, High)
        .sound_acmd("sound_speciallw", sound_speciallw, High)
        .sound_acmd("sound_specialairlw", sound_speciallw, High)
        .expression_acmd("expression_speciallw", expression_speciallw, High)
        .expression_acmd("expression_specialairlw", expression_speciallw, High)
        .game_acmd("game_speciallwblank", game_speciallwblank, High)
        .game_acmd("game_specialairlwblank", game_speciallwblank, High)
        .effect_acmd("effect_speciallwblank", effect_speciallwblank, High)
        .effect_acmd("effect_specialairlwblank", effect_speciallwblank, High)
        .sound_acmd("sound_speciallwblank", sound_speciallwblank, High)
        .sound_acmd("sound_specialairlwblank", sound_speciallwblank, High)
        .expression_acmd("expression_speciallwblank", expression_speciallwblank, High)
        .expression_acmd("expression_specialairlwblank", expression_speciallwblank, High)

        .install();
}