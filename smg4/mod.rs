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

mod acmd;
mod pingas;
mod frame;
mod status;

const DECOY_KIND: smash::lib::LuaConst = WEAPON_KIND_MIISWORDSMAN_CHAKRAM;
const DECOY_ORIGIN: &str = "miiswordsman";
const MARIOD_GENERATE_LAST: i32 = 5;

pub static mut FIGHTER_MARIO_GENERATE_ARTICLE_DECOY: i32 = 0;
pub const DECOY_STATUS_HAVED: i32 = 0;
pub const DECOY_STATUS_FLY: i32 = 2;

pub fn install() {

    acmd::install();
    pingas::install();
    frame::install();
    status::install();

}