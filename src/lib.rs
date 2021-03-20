#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use skyline::hooks::{getRegionAddress, Region};

pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;
pub static mut IS_FUNNY : [bool; 8] = [false; 8];
pub static mut IS_FGC : [bool; 8] = [false; 8];
static mut INT_OFFSET : usize = 0x4E19D0;
// static mut INT64_OFFSET : usize = 0x4E19F0;
static mut FLOAT_OFFSET : usize = 0x4E19D0;
static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];

static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];

// static INT64_SEARCH_CODE: &[u8] = &[
//     0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x15, 0x40, 0xf9,
// ];

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr();
    };
}

mod custom;
mod daisy;
mod samusd;
mod lucina;
use crate::lucina::LUCINA_SPECIAL_AIR_S;
mod littlemac;
mod gaogaen;
mod dedede;
mod lucas;
mod jack;
mod kirby;
mod cloud;
mod lucario;
use crate::lucario::IS_SPIRIT_BOMB;
// mod bayonetta;
// mod dolly;
mod shulk;
use crate::shulk::SHULK_SPECIAL_LW;
mod pikachu;
mod robot;
mod snake;
mod palutena;
mod master;
mod ryu;
mod toonlink;
mod zelda;
mod buddy;
mod ridley;
use crate::ridley::FUNNY_RIDLEY;
mod koopajr;
mod gamewatch;
mod donkey;
mod richter;
use crate::richter::RICHTER_SPECIAL_HI;
mod eflame;
mod elight;
mod falco;
// mod brave;
mod purin;

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term )]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let ret = original!()(module_accessor,term);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
    // Fighter-Specific Param Edits
    
    if fighter_kind == *FIGHTER_KIND_LUCINA {
        if LUCINA_SPECIAL_AIR_S[entry_id] && IS_FUNNY[entry_id] == false {
            if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S { // Disable Lion's Leap if used once unless in Funny
                return false;
            }
            else {
                return ret;
            }
        }
        if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW { // No One More! Spam
            return false;
        }
        else {
            return ret;
        }
    }
    if fighter_kind == *FIGHTER_KIND_SHULK {
        if SHULK_SPECIAL_LW[entry_id] && IS_FUNNY[entry_id] == false { // Disable Vision if used Burst and not in Funny
            if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
                return false;
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    if fighter_kind == *FIGHTER_KIND_RICHTER {
        if RICHTER_SPECIAL_HI[entry_id] && IS_FUNNY[entry_id] == false { // Disable Multiple Up-Bs unless in Funny
            if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
                return false;
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    else {
        return ret;
    }
}

#[skyline::hook(offset = INT_OFFSET)]
pub unsafe fn get_param_int_replace(boma: u64, param_type: u64, param_hash: u64) -> i32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let fighter_kind = smash::app::utility::get_kind(module_accessor);

    // Global Param Edits
    
    if param_hash == smash::hash40("guard_off_cancel_frame") { // Shield Drop Cancel Frame
        if IS_FUNNY[entry_id] {
            return 5;
        }
        else {
            return ret;
        }
    }

    if param_hash == smash::hash40("invalid_capture_frame") { // The Chain Grab Param
        if IS_FUNNY[entry_id] {
            return 1;
        }
        else {
            return ret;
        }
    }
    
    // Fighter-Specific Param Edits

    if fighter_kind == *WEAPON_KIND_LUCARIO_AURABALL { // Funny Mode Spirit Bomb Params
        let oboma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let o_entry_id = WorkModule::get_int(oboma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if param_hash == smash::hash40("life") {
            if IS_SPIRIT_BOMB[o_entry_id] {
                return 6000;
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    if fighter_kind == *WEAPON_KIND_SAMUSD_CSHOT { // Phazon Orb Life
        let oboma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let o_entry_id = WorkModule::get_int(oboma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if param_hash == smash::hash40("life") {
            if IS_FUNNY[o_entry_id] {
                return 6000;
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    if fighter_kind == *FIGHTER_KIND_RIDLEY { // Funny Ridley
        if FUNNY_RIDLEY[entry_id] {
            if param_hash == smash::hash40("max_charge_frame") {// Funny Plasma Breath
                return 300;
            }
            if param_hash == smash::hash40("max_fire_num") {
                return 40;
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    if fighter_kind == *FIGHTER_KIND_DAISY {
        if IS_FUNNY[entry_id] {
            if param_hash == smash::hash40("uniq_float_float_frame") { // Give Daisy Float back if Funny
                return 300;
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    if fighter_kind == *FIGHTER_KIND_CLOUD { // Limit Shenanigans
        if IS_FUNNY[entry_id] {
            if param_hash == smash::hash40("limit_break_clear_frame") {
                return 6000;
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    else {
        return ret;
    }
}

#[skyline::hook(offset = FLOAT_OFFSET)]
pub unsafe fn get_param_float_replace(boma: u64, param_type: u64, param_hash: u64) -> f32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    
    // Fighter-Specific Param Edits
    
    if fighter_kind == *WEAPON_KIND_LUCARIO_AURABALL { // Funny Mode Spirit Bomb Params
        let oboma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let o_entry_id = WorkModule::get_int(oboma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if param_hash == smash::hash40("max_speed") {
            if IS_SPIRIT_BOMB[o_entry_id] {
                return 0.4;
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    if fighter_kind == *FIGHTER_KIND_CLOUD { // Limit Shenanigans
        if IS_FUNNY[entry_id] {
            if param_hash == smash::hash40("lw2_spd_x_mul") {
                return 0.6;
            }
            if param_hash == smash::hash40("lw2_spd_y_mul") {
                return 0.6;
            }
            if param_hash == smash::hash40("lw2_accel_y") {
                return 0.06;
            }
            if param_hash == smash::hash40("lw2_speed_max_y") {
                return 1.2;
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    if fighter_kind == *FIGHTER_KIND_SHULK {
        if IS_FUNNY[entry_id] {
            if param_hash == smash::hash40("active_time_jump") {
                return 100.0;
            }
            if param_hash == smash::hash40("active_time_speed") {
                return 100.0;
            }
            if param_hash == smash::hash40("active_time_shield") {
                return 100.0;
            }
            if param_hash == smash::hash40("active_time_buster") {
                return 100.0;
            }
            if param_hash == smash::hash40("active_time_smash") {
                return 100.0;
            }
            if param_hash == smash::hash40("unavailable_time_jump") {
                return 0.1;
            }
            if param_hash == smash::hash40("unavailable_time_speed") {
                return 0.1;
            }
            if param_hash == smash::hash40("unavailable_time_shield") {
                return 0.1;
            }
            if param_hash == smash::hash40("unavailable_time_buster") {
                return 0.1;
            }
            if param_hash == smash::hash40("unavailable_time_smash") {
                return 0.1;
            }
            if param_hash == smash::hash40("shield_endure_mul") {
                return 100.0;
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    else {
        return ret;
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

#[skyline::main(name = "the_bor_patch")]
pub fn main() {
    unsafe{
        skyline::nn::ro::LookupSymbol(&mut FIGHTER_CUTIN_MANAGER_ADDR, c_str!("_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E"));
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
        // if let Some(offset) = find_subsequence(text, INT64_SEARCH_CODE) {
        //     INT64_OFFSET = offset;
        // }
    }
    custom::install();
    daisy::install();
    samusd::install();
    lucina::install();
    littlemac::install();
    gaogaen::install();
    dedede::install();
    lucas::install();
    jack::install();
    kirby::install();
    cloud::install();
    lucario::install();
    // bayonetta::install();
    // dolly::install();
    shulk::install();
    pikachu::install();
    robot::install();
    snake::install();
    palutena::install();
    master::install();
    ryu::install();
    toonlink::install();
    zelda::install();
    buddy::install();
    ridley::install();
    koopajr::install();
    gamewatch::install();
    donkey::install();
    richter::install();
    eflame::install();
    elight::install();
    falco::install();
    // brave::install();
    purin::install();
    skyline::install_hook!(is_enable_transition_term_replace);
    skyline::install_hook!(get_param_float_replace);
    skyline::install_hook!(get_param_int_replace);
}