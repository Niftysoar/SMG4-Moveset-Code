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

pub fn install() {
    Agent::new("mariod")
        .sound_acmd("sound_entryrsmg4", sound_entryr, Priority::Low)  
        .sound_acmd("sound_entrylsmg4", sound_entryr, Priority::Low) 
        
        .sound_acmd("sound_appealsrsmg4", sound_appealsr, Priority::Low)
        .sound_acmd("sound_appealslsmg4", sound_appealsr, Priority::Low)

        .sound_acmd("sound_appeallwrsmg4", sound_appeallwr, Priority::Low)
        .sound_acmd("sound_appeallwlsmg4", sound_appeallwr, Priority::Low)

        .install();
}