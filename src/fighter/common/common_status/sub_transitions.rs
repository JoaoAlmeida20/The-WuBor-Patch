#![allow(unused_must_use)]

use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    crate::{
        vars::*,
        table_const::*
    }
};

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_transition_group_check_ground_guard)]
pub unsafe fn sub_transition_group_check_ground_guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if fighter.global_table[CHECK_GROUND_GUARD_PRE].get_bool() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_GROUND_GUARD_PRE].get_ptr());
            if callable(fighter).get_bool() {
                return true.into();
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON) {
            if fighter.sub_check_command_guard().get_bool() {
                let common_guard_hold = ControlModule::get_command_life(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY2, 0x18) as i32;
                WorkModule::set_int(fighter.module_accessor, common_guard_hold, FIGHTER_INSTANCE_WORK_ID_INT_GUARD_HOLD_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), true.into());
                return true.into();
            }
        }
    }
    false.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_transition_group_check_special_command)]
pub unsafe fn sub_transition_group_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    // if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND) {
        if fighter.global_table[CHECK_SPECIAL_COMMAND].get_bool() == false {
            return false.into();
        }
        else {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_SPECIAL_COMMAND].get_ptr());
            if callable(fighter).get_bool() {
                return true.into();
            }
        }
    // }
    false.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_transition_group_check_air_tread_jump)]
pub unsafe fn sub_transition_group_check_air_tread_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cont;
    if fighter.global_table[CHECK_AIR_TREAD_JUMP_PRE].get_bool() == false {
        cont = false;
    }
    else {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_AIR_TREAD_JUMP_PRE].get_ptr());
        cont = callable(fighter).get_bool();
    }
    if cont == false {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0 {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP)
                && ControlModule::is_enable_flick_jump(fighter.module_accessor) {
                    let do_footstool;
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME) != 0 {
                        do_footstool = false;
                    }
                    else {
                        let tread_speed_y = fighter.FL_sub_fighter_float_next_tread_speed_y().get_f32();
                        let tread_jump_speed_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("tread_jump_speed_limit"));
                        if !(tread_jump_speed_limit <= tread_speed_y) {
                            do_footstool = false;
                        }
                        else {
                            fighter.clear_lua_stack();
                            lua_args!(fighter, 0x21bfbd3f83u64);
                            smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                            do_footstool = fighter.pop_lua_stack(1).get_bool();
                        }
                    }
                    if do_footstool {
                        fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), true.into());
                        return true.into();
                    }
                }
            }
            if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_JUMP_TRIGGER != 0
            || fighter.global_table[CMD_CAT2].get_i32() & (
                *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI |
                *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L |
                *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R |
                *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW
            ) != 0 /* this is the addition */ {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP_BUTTON) {
                    let do_footstool;
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME) != 0 {
                        do_footstool = false;
                    }
                    else {
                        let tread_speed_y = fighter.FL_sub_fighter_float_next_tread_speed_y().get_f32();
                        let tread_jump_speed_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("tread_jump_speed_limit"));
                        if !(tread_jump_speed_limit <= tread_speed_y) {
                            do_footstool = false;
                        }
                        else {
                            fighter.clear_lua_stack();
                            lua_args!(fighter, 0x21bfbd3f83u64);
                            smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                            do_footstool = fighter.pop_lua_stack(1).get_bool();
                        }
                    }
                    if do_footstool {
                        fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), true.into());
                        return true.into();
                    }
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP_NO_TRIGGER) {
                fighter.clear_lua_stack();
                lua_args!(fighter, 0x21bfbd3f83u64, true);
                smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                if fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), false.into());
                    return true.into();
                }
            }
        }
    }
    else {
        return true.into();
    }
    false.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_transition_group_check_ground_guard,
            sub_transition_group_check_special_command,
            sub_transition_group_check_air_tread_jump
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}