use super::*;

unsafe extern "C" fn mariod_drcapsule_regular_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn mariod_drcapsule_regular_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_drcapsule"), hash40("speed"));
    let gravity_acl_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_drcapsule"), hash40("gravity_acl_max"));
    let gravity_accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_drcapsule"), hash40("gravity_accel"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let angle: f32 = 45.0;
    let speed_x = angle.to_radians().sin()*speed*lr;
    let speed_y = angle.to_radians().cos()*speed;
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -gravity_accel);
    sv_kinetic_energy!(set_limit_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed, gravity_acl_max);
    0.into()
}

pub fn install() {
    Agent::new("mariod_drcapsule")
        .set_costume([112, 113, 114, 115, 116, 117, 118, 119].to_vec())
        .status(Pre, *WEAPON_MARIOD_DRCAPSULE_STATUS_KIND_REGULAR, mariod_drcapsule_regular_pre_status)
        .status(Init, *WEAPON_MARIOD_DRCAPSULE_STATUS_KIND_REGULAR, mariod_drcapsule_regular_init_status)
        .install()
    ;
}