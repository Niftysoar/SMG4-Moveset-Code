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

//Fighter Frame

unsafe extern "C" fn mariod_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let motion = MotionModule::motion_kind(fighter.module_accessor);
        
        if crate::MARKED_COLORS[color as usize] {
   
            if motion == hash40("throw_f") {
                if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
                    macros::PLAY_SE(fighter, Hash40::new("se_mariod_appeal_s03"));
                }
            }
        }
    }
}

pub fn install() {

    acmd::install();

    Agent::new("mariod")
        //fighter frame
        .on_line(Main, mariod_frame)

        .install();
}