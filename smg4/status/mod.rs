use smash::app;
use super::*;

mod pingas;
mod special_lw;
mod decoy;

pub fn install_hook() {
    unsafe {
        let meteor_id = smashline::clone_weapon(DECOY_ORIGIN, *DECOY_KIND, "mariod","decoy",true);
        FIGHTER_MARIO_GENERATE_ARTICLE_DECOY = MARIOD_GENERATE_LAST + meteor_id;
        println!("[smashline_smg4::mario] (HOOK) Cloned to id {FIGHTER_MARIO_GENERATE_ARTICLE_DECOY}");
    }
    //param_config::disable_villager_pocket(*smash::lib::lua_const::FIGHTER_KIND_MARIO, slots.clone(), DECOY_KIND);
}

pub fn install() {
    #[cfg(feature = "dev")] {
        unsafe {
            FIGHTER_MARIO_GENERATE_ARTICLE_DECOY = MARIOD_GENERATE_LAST + 0;
        }
    }
    
    pingas::install();
    special_lw::install();
}