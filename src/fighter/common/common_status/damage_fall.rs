#![allow(unused_must_use)]

use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Vector3f,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::table_const::*
};

#[common_status_script(status = FIGHTER_STATUS_KIND_DAMAGE_FALL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_damagefall(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_DamageFall_common();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f {x: 1.0, y: 0.0, z: 0.0});
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(status_damagefall_main as *const () as _))
}

unsafe extern "C" fn status_damagefall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() == true
    || fighter.check_damage_fall_transition().get_bool() == true {
        return 0.into();
    }
    let tech : bool;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND) == false {
        tech = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD);
    }
    else {
        // let mut flame_choke_tech_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("ganon_special_s_passive_trigger_frame")) as f32;
        // let tech_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("passive_trigger_frame_mul"), 0 as u64);
        // flame_choke_tech_frame *= tech_mul;
        tech = fighter.sub_check_passive_button(L2CValue::I32(0x30)).get_bool();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB)
    && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
    && WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value")) <= fighter.global_table[STICK_X].get_f32().abs()
    && tech {
        fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_FB.into(), true.into());
        return L2CValue::Bool(true);
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE) {
        if FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            if FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) == false {
                if tech {
                    fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE.into(), true.into());
                    return true.into();
                }
            }
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN.into(), true.into());
        return 0.into();
    }
    fighter.sub_damage_fall_uniq_process_exec_fix_pos();
    0.into()
}

pub fn install() {
    install_status_scripts!(
        status_damagefall
    );
}