use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    wubor_utils::table_const::*
};

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_JUMP_AERIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn sonic_jumpaerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ROCKETBELT) {
        let energy = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_ROCKETBELT_BURNER_ENERGY_VALUE);
        if 0.0 < energy {
            ItemModule::set_attach_item_action(fighter.module_accessor, ItemKind(*ITEM_KIND_ROCKETBELT), *ITEM_ROCKETBELT_ACTION_JUMP_JET_FIRE, 1.0);
        }
    }
    fighter.status_JumpAerialSub(false.into(), false.into());
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH {
        sonic_set_jumps(fighter);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_JumpAerial_Main as *const () as _))
}

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP_AERIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn sonic_screwjumpaerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_ItemScrewJumpAerialSub();
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH {
        sonic_set_jumps(fighter);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_ItemScrewJumpAerial_Main as *const () as _))
}

unsafe extern "C" fn sonic_set_jumps(fighter: &mut L2CFighterCommon) {
    let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    let jump_count_max = WorkModule::get_param_int(fighter.module_accessor, hash40("jump_count_max"), 0);
    if jump_count < jump_count_max {
        WorkModule::set_int(fighter.module_accessor, jump_count_max, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SCREW_JUMP_COUNT);
    if jump_count < *FIGHTER_STATUS_SCREW_JUMP_COUNT_MAX {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_SCREW_JUMP_COUNT_MAX, *FIGHTER_INSTANCE_WORK_ID_INT_SCREW_JUMP_COUNT);
    }
}

pub fn install() {
    install_status_scripts!(
        sonic_jumpaerial_main,
        sonic_screwjumpaerial_main
    );
}