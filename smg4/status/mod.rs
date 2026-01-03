use smash::app;
use super::*;

mod pingas;
mod special_lw;
mod decoy;

pub fn install() {    
    pingas::install();
    special_lw::install();
    decoy::install();
}