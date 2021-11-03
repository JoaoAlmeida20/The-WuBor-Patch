use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::table_const::*
};

#[status_script(agent = "pitb", status = FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn pitb_specialncharge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_DIR_S) {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_hold_hi") as i64, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_hold_hi") as i64, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION_AIR);
        WorkModule::set_int64(fighter.module_accessor, 0x7cabbcbb5, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION);
        WorkModule::set_int64(fighter.module_accessor, 0xbd2abc95c, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION_AIR);
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_hold_s") as i64, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_hold_s") as i64, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION_AIR);
        WorkModule::set_int64(fighter.module_accessor, 0x684068652, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION);
        WorkModule::set_int64(fighter.module_accessor, 0xa23431885, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION_AIR);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(pitb_specialncharge_charge as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(pitb_specialncharge_loop as *const () as _))
}

unsafe extern "C" fn pitb_specialncharge_charge(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_CHARGE);
    0.into()
}

unsafe extern "C" fn pitb_specialncharge_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_PIT_SPECIAL_AIR_N);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_FIRST) {
            let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION_AIR);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot), -1.0, 1.0, 0.0, false, false);
            let bowmot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION_AIR);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, Hash40::new_raw(bowmot), true, -1.0);
        }
        else {
            let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION_AIR);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
            let bowmot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION_AIR);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, Hash40::new_raw(bowmot), false, -1.0);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_FIRST);
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_FIRST) {
            let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot), -1.0, 1.0, 0.0, false, false);
            let bowmot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, Hash40::new_raw(bowmot), true, -1.0);
        }
        else {
            let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
            let bowmot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, Hash40::new_raw(bowmot), false, -1.0);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_FIRST);
        }
    }
    let curr_charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_CHARGE);
    let max_charge = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("charge_frame"));
    if !ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    && curr_charge < max_charge {
        let sticky = fighter.global_table[STICK_Y].get_f32();
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_DIR_S) {
            let upsticky = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("up_stick_y"));
            if sticky < upsticky {
                fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR.into(), true.into());
            }
        }
        else {
            let upsticky = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("up_stick_y"));
            if sticky >= upsticky {
                fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR.into(), true.into());
            }
        }
    }
    else {
        if curr_charge >= max_charge {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX);
            fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
        }
        else {
            // let stickx = fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor);
            // let turnstickx = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("turn_stick_x"));
            // if stickx <= turnstickx {
            //     fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN.into(), true.into());
            // }
            // else {
                fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT.into(), true.into());
            // }
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "pitb", status = FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn pitb_specialnturn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::reverse_lr(fighter.module_accessor);
    fighter.sub_shift_status_main(L2CValue::Ptr(pitb_specialnturn_loop as *const () as _))
}

unsafe extern "C" fn pitb_specialnturn_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor)
    || !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_SHOOT_FLAG_FIRST) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_PIT_SPECIAL_AIR_N);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_SHOOT_FLAG_FIRST) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_s_to_s"), 0.0, 1.0, false, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, Hash40::new_raw(0xa728bc2ca), false, -1.0);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_SHOOT_FLAG_FIRST);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_s_to_s"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, Hash40::new_raw(0xa728bc2ca), true, -1.0);
            }
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_SHOOT_FLAG_FIRST) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_s_to_s"), 0.0, 1.0, false, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, Hash40::new_raw(0x6d5ce5c1d), false, -1.0);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_SHOOT_FLAG_FIRST);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_s_to_s"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, Hash40::new_raw(0x6d5ce5c1d), true, -1.0);
            }
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "pitb", status = FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn pitb_specialnshoot_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    pitb_specialn_endremove(fighter)
}

unsafe extern "C" fn pitb_specialn_endremove(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE
    && status != *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR
    && status != *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN
    && status != *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT {
        ArticleModule::remove(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_normal") as i64);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    L2CValue::I32(0)
}

pub fn install() {
    install_status_scripts!(
        pitb_specialncharge_main,
        pitb_specialnturn_main,
        pitb_specialnshoot_end
    );
}