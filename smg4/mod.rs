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

pub const FIGHTER_SMG4_GENERATE_ARTICLE_WALUAUNCHER: i32 = 6;

pub fn install() {

    acmd::install();
    pingas::install();
    frame::install();
    status::install();

}