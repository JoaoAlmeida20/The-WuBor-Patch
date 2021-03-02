#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;

pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;
// static mut INT_OFFSET : usize = 0x4ded80;
static mut FLOAT_OFFSET : usize = 0x4dedc0;

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr();
    };
}

mod custom;
mod daisy;
// mod samusd;
mod lucina;
use crate::lucina::{LUCINA_SPECIAL_AIR_S, LUCINA_SPECIAL_LW};
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
// mod edge;
mod koopajr;

#[skyline::hook(replace=smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let ret = original!()(module_accessor,term);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter_kind == *FIGHTER_KIND_LUCINA {
        if LUCINA_SPECIAL_AIR_S[entry_id] {
            if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S {
                return false;
            }
            else {
                return ret;
            }
        }
        if LUCINA_SPECIAL_LW[entry_id] {
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
    else if fighter_kind == *FIGHTER_KIND_SHULK {
        if SHULK_SPECIAL_LW[entry_id] {
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
    if fighter_kind == *WEAPON_KIND_LUCARIO_AURABALL {
        if param_hash == smash::hash40("max_speed") {
            if IS_SPIRIT_BOMB[entry_id] {
                return 0.7;
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

// #[skyline::hook(offset = INT_OFFSET)]
// pub unsafe fn get_param_int_replace(boma: u64, param_type: u64, param_hash: u64) -> i32 {
//     let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
//     let ret = original!()(boma, param_type, param_hash);
//     let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
//     let fighter_kind = smash::app::utility::get_kind(module_accessor);
//     if fighter_kind == *FIGHTER_KIND_LUCARIO {
//         println!("Is Lucario!");
//         if param_type == smash::hash40("param_auraball") {
//             println!("In the auraball Param!");
//             if param_hash == smash::hash40("life") {
//                 println!("In the Life Param!");
//                 if IS_SPIRIT_BOMB[entry_id] {
//                     return 500;
//                 }
//                 else {
//                     return ret;
//                 }
//             }
//             else {
//                 return ret;
//             }
//         }
//         else {
//             return ret;
//         }
//     }
//     else {
//         return ret;
//     }
// }

#[skyline::main(name = "the_bor_patch")]
pub fn main() {
    unsafe{
        skyline::nn::ro::LookupSymbol(&mut FIGHTER_CUTIN_MANAGER_ADDR, c_str!("_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E"));
    }
    custom::install();
    daisy::install();
    // samusd::install();
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
    // edge::install();
    koopajr::install();
    skyline::install_hook!(is_enable_transition_term_replace);
    skyline::install_hook!(get_param_float_replace);
    // skyline::install_hook!(get_param_int_replace);
}