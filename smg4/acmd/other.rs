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

unsafe extern "C" fn sound_entryr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mariod_appear01"));
    }
    frame(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mariod_appear02"));
    }
}

unsafe extern "C" fn sound_appealsr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mariod_appeal_s01"));
    }
}

unsafe extern "C" fn sound_appeallwr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mariod_appeal_l01"));
    }
}

unsafe extern "C" fn game_win1(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn effect_win1(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn sound_win1(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn game_win1wait(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn game_win2(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn effect_win2(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn sound_win2(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn game_win2wait(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn expression_win2wait(agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("mariod")
    .set_costume([112, 113, 114, 115, 116, 117, 118, 119].to_vec())
        .sound_acmd("sound_entryr", sound_entryr, Priority::Low)  
        .sound_acmd("sound_entryl", sound_entryr, Priority::Low) 
        
        .sound_acmd("sound_appealsr", sound_appealsr, Priority::Low)
        .sound_acmd("sound_appealsl", sound_appealsr, Priority::Low)

        .sound_acmd("sound_appeallwr", sound_appeallwr, Priority::Low)
        .sound_acmd("sound_appeallwl", sound_appeallwr, Priority::Low)

        .game_acmd("game_win1", game_win1, Priority::Low)
        .effect_acmd("effect_win1", effect_win1, Priority::Low)
        .sound_acmd("sound_win1", sound_win1, Priority::Low)

        .game_acmd("game_win1wait", game_win1wait, Priority::Low)

        .game_acmd("game_win2", game_win2, Priority::Low)
        .effect_acmd("effect_win2", effect_win2, Priority::Low)
        .sound_acmd("sound_win2", sound_win2, Priority::Low)

        .game_acmd("game_win2wait", game_win2wait, Priority::Low)
        .expression_acmd("expression_win2wait", expression_win2wait, Priority::Low)

        .install();
}