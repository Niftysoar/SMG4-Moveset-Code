use crate::imports::imports_agent::*;
use crate::imports::imports_acmd::*;

const MOVE_SPEED : f32 = 2.0;
const LIFE : i32 = 60;

unsafe extern "C" fn empty_status(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::disable_tip(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.5, 62, 64, 0, 77, 7.5, 0.0, 5.0, 0.0, Some(0.0), Some(-5.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}
unsafe extern "C" fn effect_fly(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn game_burst(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ModelModule::set_visibility(agent.module_accessor, false);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_furafura"), 9, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosionm"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}
unsafe extern "C" fn effect_burst(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn sound_burst(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_m"));
    }
}

pub unsafe extern "C" fn haved_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_RESET,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}
pub unsafe extern "C" fn haved_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    weapon.fastshift(L2CValue::Ptr(haved_main_loop as *const () as _))
}

unsafe extern "C" fn haved_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub unsafe extern "C" fn fly_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

pub unsafe extern "C" fn fly_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let prev_status = StatusModule::prev_status_kind(weapon.module_accessor, 0);
    if prev_status == crate::smg4::DECOY_STATUS_FLY {
    if prev_status == crate::smg4::DECOY_STATUS_FLY {
        return 0.into();
    }

    LinkModule::remove_model_constraint(weapon.module_accessor, true);
    let owner = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner);
    let lr = PostureModule::lr(owner_boma);
    
    /*
    SNAP TO BONE
    */
    let owner_bone = Hash40::new("haver");
    let mut offset = Vector3f{x:0.0,y:0.0,z:0.0};
    
    let mut owner_bone_pos = Vector3f{x:0.0,y:0.0,z:0.0};
    let owner_offset = ModelModule::joint_global_offset_from_top(owner_boma, owner_bone, &mut owner_bone_pos);
    let newPos = Vector3f{
        x: PostureModule::pos_x(owner_boma) + owner_bone_pos.x + (offset.x*lr), 
        y: PostureModule::pos_y(owner_boma) + owner_bone_pos.y + offset.y, 
        z: GroundModule::get_z(weapon.module_accessor)
    };
    PostureModule::set_pos(weapon.module_accessor, &newPos); 

    /*
    SET LR and Rot
    */
    PostureModule::set_lr(weapon.module_accessor, lr);
    PostureModule::update_rot_y_lr(weapon.module_accessor);
    let rot_y = PostureModule::rot_y(weapon.module_accessor, 0);
    let rot_z = if lr > 0.0 {-90.0} else {90.0};
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: 0.0, y: rot_y, z: rot_z}, 0);
    let rot_y = PostureModule::rot_y(weapon.module_accessor, 0);
    let rot_z = if lr > 0.0 {-90.0} else {90.0};
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: 0.0, y: rot_y, z: rot_z}, 0);

    /*
    SET SPEED
    */
    let move_speed = MOVE_SPEED;
    let speed_x = move_speed;
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        move_speed*lr,
        0.0
    );

    0.into()
}

pub unsafe extern "C" fn fly_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    //Life
    let life = LIFE;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    //Set Motion
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    
    weapon.fastshift(L2CValue::Ptr(fly_main_status_loop as *const () as _))
}
unsafe extern "C" fn fly_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    //Die on attack collision
    if AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_HIT) {
        WorkModule::set_int(weapon.module_accessor, 1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    //Die on wall hit
    if GroundModule::is_touch(weapon.module_accessor, (*GROUND_TOUCH_FLAG_ALL) as u32) {
       WorkModule::set_int(weapon.module_accessor, 1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    if WorkModule::count_down_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE, 0) {
        return fly_burst(weapon);
    }

    0.into()
}
unsafe extern "C" fn fly_burst(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    AttackModule::clear_all(weapon.module_accessor);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("burst"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(fly_burst_loop as *const () as _))
}
unsafe extern "C" fn fly_burst_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub fn install() {     
    let agent = &mut smashline::Agent::new("mariod_decoy");
    agent.set_costume([112, 113, 114, 115, 116, 117, 118, 119].to_vec());
    agent.set_costume([112, 113, 114, 115, 116, 117, 118, 119].to_vec());
    agent.acmd("game_fly", game_fly, Priority::Default);
    agent.acmd("effect_fly", effect_fly, Priority::Default);
    
    agent.acmd("game_burst", game_burst, Priority::Default);
    agent.acmd("effect_burst", effect_burst, Priority::Default);
    agent.acmd("sound_burst", sound_burst, Priority::Default);

    agent.status(Pre, crate::smg4::DECOY_STATUS_HAVED, haved_pre);
    agent.status(Init, crate::smg4::DECOY_STATUS_HAVED, empty_status);
    agent.status(Main, crate::smg4::DECOY_STATUS_HAVED, haved_main);
    agent.status(End, crate::smg4::DECOY_STATUS_HAVED, empty_status);
    agent.status(Pre, crate::smg4::DECOY_STATUS_HAVED, haved_pre);
    agent.status(Init, crate::smg4::DECOY_STATUS_HAVED, empty_status);
    agent.status(Main, crate::smg4::DECOY_STATUS_HAVED, haved_main);
    agent.status(End, crate::smg4::DECOY_STATUS_HAVED, empty_status);

    agent.status(Pre, crate::smg4::DECOY_STATUS_FLY, fly_pre);
    agent.status(Init, crate::smg4::DECOY_STATUS_FLY, fly_init);
    agent.status(Main, crate::smg4::DECOY_STATUS_FLY, fly_main);
    agent.status(End, crate::smg4::DECOY_STATUS_FLY, empty_status);
    agent.status(Pre, crate::smg4::DECOY_STATUS_FLY, fly_pre);
    agent.status(Init, crate::smg4::DECOY_STATUS_FLY, fly_init);
    agent.status(Main, crate::smg4::DECOY_STATUS_FLY, fly_main);
    agent.status(End, crate::smg4::DECOY_STATUS_FLY, empty_status);

    agent.install();
}
