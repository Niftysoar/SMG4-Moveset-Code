#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

use std::collections::HashMap;
use smash::hash40;
use csk::*;
use param_config::*;
use smash::lib::lua_const::{FIGHTER_KIND_MARIOD};
use smash::lib::lua_const::WEAPON_KIND_KOOPAJR_CANNONBALL;

mod smg4;

pub static mut MARKED_COLORS: [bool; 256] = [false; 256];

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    let lowest_color: i32 = 112;
    let color_num: i32 = 8;
    let marked_slots: Vec<i32> = (112..=119).collect();

    unsafe {
        for slot in &marked_slots {
            MARKED_COLORS[*slot as usize] = true;
        }
    }

    // Param Edits

    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("walk_accel_mul"), 0, 0.09));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("walk_accel_add"), 0, 0.09));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("walk_accel_max"), 0, 1.05));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("walk_slow_speed_mul"), 0, 0.2));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("walk_middle_ratio"), 0, 0.38));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("walk_fast_ratio"), 0, 0.75));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("ground_brake"), 0, 0.138));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("run_accel_mul"), 0, 0.16));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("run_accel_add"), 0, 0.05));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("run_speed_max"), 0, 1.7));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("dash_speed"), 0, 1.9));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("jump_inital_y"), 0, 19.5));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("jump_y"), 0, 30.00));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("mini_jump_y"), 0, 16.5));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("jump_aerial_y"), 0, 34.0));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("jump_speed_x"), 0, 0.85));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("jump_speed_x_mul"), 0, 0.75));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("jump_speed_x_max"), 0, 1.2));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("jump_aerial_speed_x_max"), 0, 0.85));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("air_accel_x_mul"), 0, 0.06));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("wall_jump_type"), 0, 0.0));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("air_accel_x_add"), 0, 0.008));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("air_accel_x_stable"), 0, 1.1));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("air_brake_x"), 0, 0.013));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("air_accel_y"), 0, 0.087));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("air_accel_y_stable"), 0, 1.5));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("damage_fly_top_air_accel_y"), 0, 0.074928));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("damage_fly_top_air_accel_y_stable"), 0, 1.8));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("air_break_y"), 0, 0.015));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("dive_speed_y"), 0, 2.4));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("jump_count_max"), 0, 2.0));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("scale"), 0, 1.0));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("weight"), 0, 104.0));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("height"), 0, 14.0));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("expand_height"), 0, 18.0));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("shield_radius"), 0, 9.7));
    update_float_2(*FIGHTER_KIND_MARIOD, marked_slots.clone(), (hash40("shield_break_y"), 0, 44.1));


    // CSK Stuff

    add_narration_characall_entry("vc_narration_characall_smg4");

    add_chara_db_entry_info(CharacterDatabaseEntry{
        ui_chara_id: hash40("ui_chara_smg4"),
        clone_from_ui_chara_id: Some(hash40("ui_chara_mariod")),
        name_id: StringType::Overwrite(CStrCSK::new("smg4")),
        ui_series_id: Hash40Type::Overwrite(hash40("ui_series_smg4")),
        // disp_order: SignedByteType::Overwrite(63),
        is_dlc:BoolType::Overwrite(false),
        is_patch: BoolType::Overwrite(false),
        color_num: UnsignedByteType::Overwrite(color_num as u8),
        extra_index_maps: UnsignedByteMap::Overwrite(HashMap::from([
            (hash40("color_start_index"), UnsignedByteType::Overwrite(lowest_color as u8))
        ])),
        extra_hash_maps: Hash40Map::Overwrite(HashMap::from([
            (hash40("characall_label_c00"), Hash40Type::Overwrite(hash40("vc_narration_characall_smg4"))),
            (hash40("original_ui_chara_hash"), Hash40Type::Overwrite(hash40("ui_chara_mariod")))
        ])),
        ..Default::default()
    });

    add_chara_layout_db_entry_info(CharacterLayoutDatabaseEntry {
        ui_layout_id: hash40("ui_chara_smg4_00"),
        clone_from_ui_layout_id: Some(hash40("ui_chara_mariod_00")),
        ui_chara_id: Hash40Type::Overwrite(hash40("ui_chara_smg4")),
        ..Default::default()
    });

    add_series_db_entry_info(SeriesDatabaseEntry {
        ui_series_id: hash40("ui_series_smg4"),
        name_id: StringType::Overwrite(CStrCSK::new("smg4")),
        disp_order: SignedByteType::Overwrite(0),
        disp_order_sound: SignedByteType::Overwrite(0),
        save_no: SignedByteType::Overwrite(0),
        shown_as_series_in_directory: BoolType::Overwrite(false),
        is_dlc: BoolType::Overwrite(false),
        is_patch: BoolType::Overwrite(false),
        dlc_chara_id: Hash40Type::Overwrite(0),
        is_use_amiibo_bg: BoolType::Overwrite(false),
        ..Default::default()
    });

    // add_tips_db_entry_info(&TipsDatabaseEntry {
    //     ui_tips_id: hash40("ui_tips_smg4_00"),
    //     clone_from_ui_tips_id: Some(hash40("ui_tips_mariod_00")),
    //     // save_no: UnsignedIntType::Optional(Some(id)),
    //     // level: Hash40Type::Overwrite(hash40(level)),
    //     // topic: Hash40Type::Overwrite(hash40(topic)),
    //     // skill_kind: Hash40Type::Overwrite(hash40(skill_kind)),
    //     ui_tips_unlock_id: Hash40Type::Overwrite(smash::hash40("")),
    //     // disp_order: UnsignedIntType::Optional(Some(order)),
    //     type_0: Hash40Type::Overwrite(smash::hash40("relation_chara")),
    //     key_0: Hash40Type::Overwrite(smash::hash40("ui_chara_smg4")), //chara_hash?
    //     type_1: Hash40Type::Overwrite(smash::hash40("relation_series")), //relation_series
    //     key_1: Hash40Type::Overwrite(smash::hash40("ui_series_smg4")), //ui_series_mario
    //     type_2: Hash40Type::Overwrite(smash::hash40("")),
    //     key_2: Hash40Type::Overwrite(smash::hash40("")),
    //     type_3: Hash40Type::Overwrite(smash::hash40("")),
    //     key_3: Hash40Type::Overwrite(smash::hash40("")),
    //     type_4: Hash40Type::Overwrite(smash::hash40("")),
    //     key_4: Hash40Type::Overwrite(smash::hash40("")),
    //     type_5: Hash40Type::Overwrite(smash::hash40("")),
    //     key_5: Hash40Type::Overwrite(smash::hash40("")),
    //     type_6: Hash40Type::Overwrite(smash::hash40("")),
    //     key_6: Hash40Type::Overwrite(smash::hash40("")),
    //     type_7: Hash40Type::Overwrite(smash::hash40("")),
    //     key_7: Hash40Type::Overwrite(smash::hash40("")),
    //     type_8: Hash40Type::Overwrite(smash::hash40("")),
    //     key_8: Hash40Type::Overwrite(smash::hash40("")),
    //     ..Default::default()
    // });

}

#[skyline::main(name = "smashline_test")]
pub fn main() {

    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }

    smg4::install();
    // smashline::clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "mariod", "waluauncher", true);
}