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

mod mariod;

pub static mut MARKED_COLORS: [bool; 256] = [false; 256];
pub static mut LAST_COLOR: i32 = -1;

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    const FIGHTER_NAME: &str = "mariod";
    const MARKER_FILE: &str = "smg4.marker";
    let mut lowest_color: i32 = -1;
    let mut marked_slots: Vec<i32> = vec![];
    for x in 0..256 {
        if let Ok(_) = std::fs::read(format!(
            "mods:/fighter/{}/model/body/c{:02}/{}",
            FIGHTER_NAME, x, MARKER_FILE
        )) {
            unsafe {
                marked_slots.push(x as _);
                MARKED_COLORS[x as usize] = true;
                if lowest_color == -1 {
                    lowest_color = x as _ ;
                }
            }
        }
    }


    if lowest_color == -1 {
        // if no marker exist, leave
        return;
    }

    let color_num = {
        unsafe {
            let mut index = lowest_color;
            while index < 256 && MARKED_COLORS[index as usize] {
                index += 1;
            }
            LAST_COLOR = index - 1;
            index - lowest_color
        }
    };

    // Param Edits

    

    // CSK Stuff

    add_narration_characall_entry("vc_narration_characall_smg4");

    add_chara_db_entry_info(CharacterDatabaseEntry{
        ui_chara_id: hash40("ui_chara_smg4"),
        clone_from_ui_chara_id: Some(hash40("ui_chara_mariod")),
        name_id: StringType::Overwrite(CStrCSK::new("smg4")),
        ui_series_id: Hash40Type::Overwrite(hash40("ui_series_smg4")),
        disp_order: SignedByteType::Overwrite(63),
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

    mariod::install();
}