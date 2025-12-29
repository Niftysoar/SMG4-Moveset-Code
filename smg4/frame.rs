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

unsafe extern "C" fn mariod_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let motion = MotionModule::motion_kind(fighter.module_accessor);
        
        if crate::MARKED_COLORS[color as usize] {

            // Trigger only when entering nair (frame < 1.0 prevents loops)
            if motion == hash40("attack_air_n") && MotionModule::frame(boma) < 1.0 {
                // Pick one of 3 animations: 0,1,2
                let roll = sv_math::rand(hash40("mario_random_nair"), 3);

                let chosen = match roll {
                    0 => hash40("attack_air_n_alt1"),
                    1 => hash40("attack_air_n_alt2"),
                    _ => hash40("attack_air_n_alt3"),
                };

                // Change the animation BEFORE hitboxes spawn
                MotionModule::change_motion(boma, smash::phx::Hash40::new_raw(chosen), 0.0, 1.0, false, 0.0, false, false);
            }
   
            if motion == hash40("throw_f") || motion == hash40("attack_air_b") {
                if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
                    macros::PLAY_SE(fighter, Hash40::new("se_mariod_appeal_s03"));
                }
            }

            let beeg_smg4_backthrow = motion == smash::hash40("throw_b") && MotionModule::frame(boma) >= 1.0 && MotionModule::frame(boma) <= 24.0;
            let mmmmmmmmmmmmmmmmm = (motion == hash40("special_air_hi") || motion == hash40("special_hi")) && MotionModule::frame(boma) >= 1.0 && MotionModule::frame(boma) <= 60.0;
            if beeg_smg4_backthrow {
                // Show BEEG
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("beegsmg4"), true);
            } else {
                // Hide BEEG
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("beegsmg4"), false);
            }

            if mmmmmmmmmmmmmmmmm {
                // Show MarioTT
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("mariott"), true);
            } else {
                // Hide MarioTT
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("mariott"), false);
            }

            if GroundModule::can_entry_cliff(boma) == 1 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT) < 7 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME) < 1 && ControlModule::get_stick_y(boma) > -0.5{
                fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE.into(),false.into());
            };
            if motion == hash40("special_air_hi") || motion == hash40("special_hi") {
                if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
                };
            }
        }
    }
}

pub fn install() {
    Agent::new("mariod")
    .set_costume([112, 113, 114, 115, 116, 117, 118, 119].to_vec())

        .on_line(Main, mariod_frame)

        .install();
}