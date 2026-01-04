use crate::imports::imports_agent::*;
use crate::imports::imports_acmd::*;

unsafe extern "C" fn special_lw_update_motion(fighter: &mut L2CFighterCommon, is_loop: bool) {
    let mot_g =  WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    let mot_a = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    if !is_loop {
        let situation = fighter.global_table[0x16].get_i32();
        let motion = if situation == *SITUATION_KIND_GROUND {mot_g} else {mot_a};
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        fighter.sub_change_motion_by_situation(Hash40::new_raw(mot_g).into(), Hash40::new_raw(mot_a).into(), true.into());
    }
}

unsafe extern "C" fn special_lw_kinetic_correct(fighter: &mut L2CFighterCommon) {
    let situation = fighter.global_table[0x16].get_i32();
    if situation == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        //KineticModule::clear_speed_all(fighter.module_accessor);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    }
}

unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_blank = ArticleModule::is_exist(fighter.module_accessor, crate::smg4::FIGHTER_MARIO_GENERATE_ARTICLE_DECOY);

    let mot_g = if !is_blank {hash40("special_lw")} else {hash40("special_lw_blank")};
    let mot_a = if !is_blank {hash40("special_air_lw")} else {hash40("special_air_lw_blank")};
    WorkModule::set_int64(fighter.module_accessor, mot_g as i64,*FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, mot_a as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    special_lw_update_motion(fighter,false);
    special_lw_kinetic_correct(fighter);
    
	fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = fighter.global_table[0x16].get_i32();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let next_status = if situation == *SITUATION_KIND_GROUND {FIGHTER_STATUS_KIND_WAIT} else {FIGHTER_STATUS_KIND_FALL};
        fighter.change_status(next_status.into(), false.into());
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_set_ground_correct_by_situation(false.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
        let mot_g =  WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
        let mot_a = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
        special_lw_update_motion(fighter,true);
    }

    0.into()
}

pub fn install() {
    Agent::new("mariod")
    .set_costume([112, 113, 114, 115, 116, 117, 118, 119].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_pre)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main)
    .install();
}