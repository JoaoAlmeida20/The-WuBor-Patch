//use smash::app::utility::get_kind;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
// use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::{L2CFighterCommon/*, L2CFighterBase*/};
// use smash_script::*;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::{IS_FUNNY, IS_FGC};
use skyline::nn::ro::LookupSymbol;
use smash::app::{self, /***/};

pub static mut TIME_SLOW_EFFECT_VECTOR: smash::phx::Vector3f = smash::phx::Vector3f {x:-3.0,y:3.0,z:0.0};
//pub const TIME_SLOW_EFFECT_HASH: u64 = smash::hash40("sys_sp_flash");

// Use this for general per-frame fighter-level hooks
unsafe fn global_fighter_frame(_fighter : &mut L2CFighterCommon) {
    let lua_state = _fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    LookupSymbol(
        &mut FIGHTER_CUTIN_MANAGER_ADDR,
        "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
    );
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP)
    && IS_FUNNY[entry_id] == false {
        IS_FUNNY[entry_id] = true;
    }
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP);
    }
    // if !ItemModule::is_attach_item(boma, app::ItemKind(*ITEM_KIND_USAGIHAT))
    if (StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DEAD || smash::app::sv_information::is_ready_go() == false)
    && IS_FUNNY[entry_id] {
        IS_FUNNY[entry_id] = false;
    }
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_REFLECTOR)
    && IS_FGC[entry_id] == false {
        IS_FGC[entry_id] = true;
        println!("FGC is on!");
    }
    if IS_FGC[entry_id] {
        if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_REFLECTOR) {
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_REFLECTOR);
            println!("Disabled Badge Reflector!");
        }
    }
    if !ItemModule::is_attach_item(boma, app::ItemKind(*ITEM_KIND_BADGE))
    && IS_FGC[entry_id] {
        IS_FGC[entry_id] = false;
        println!("FGC is off!");
    }
    if IS_FUNNY[entry_id] {
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            StatusModule::change_status_request_from_script(boma,*FIGHTER_STATUS_KIND_FALL_AERIAL,true);
        }
    }
}

// Use this for general per-frame weapon-level hooks
//pub fn once_per_weapon_frame(fighter_base : &mut L2CFighterBase) {
//    unsafe {
//        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
//        let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;
//
//        if frame % 10 == 0 {
//            println!("[Weapon Hook] Frame : {}", frame);
//        }
//    }
//}

pub fn install() {
    smash_script::add_fighter_frame_callbacks!(global_fighter_frame);
//    acmd::add_custom_weapon_hooks!(once_per_weapon_frame);
}