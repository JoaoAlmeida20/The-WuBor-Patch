use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*},
    super::vl
};

#[status_script(agent = "chrom", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn chrom_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_1 as u32,
        0
    );
    0.into()
}

#[status_script(agent = "chrom", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn chrom_attacks3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32,
        0
    );
    0.into()
}

#[status_script(agent = "chrom", status = FIGHTER_STATUS_KIND_ATTACK_HI3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn chrom_attackhi3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32,
        0
    );
    0.into()
}

#[status_script(agent = "chrom", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn chrom_attacklw3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32,
        0
    );
    0.into()
}

#[status_script(agent = "chrom", status = FIGHTER_STATUS_KIND_ATTACK_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn chrom_attackdash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3);
    1.into()
}

#[status_script(agent = "chrom", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn chrom_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

#[status_script(agent = "chrom", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn chrom_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_lw"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH);
    fighter.sub_shift_status_main(L2CValue::Ptr(chrom_speciallw_loop as *const () as _))
}

unsafe extern "C" fn chrom_speciallw_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH)
        && GroundModule::can_entry_cliff(fighter.module_accessor) != 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE.into(), false.into());
        }
    }
    else {
        fighter.change_status(FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
    }
    0.into()
}

#[status_script(agent = "chrom", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn chrom_speciallw_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "chrom", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn chrom_speciallw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, chrom::status::flag::SPECIAL_LW_CHANGE_KINETIC) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_MOTION_RESET_TYPE_AIR_TRANS,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        macros::SET_SPEED_EX(
            fighter,
            vl::param_special_lw::dive_speed_x,
            vl::param_special_lw::dive_speed_y,
            *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN
        );
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        VarModule::off_flag(fighter.battle_object, chrom::status::flag::SPECIAL_LW_CHANGE_KINETIC);
    }
    0.into()
}

#[status_script(agent = "chrom", status = FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn chrom_speciallw_hit_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "chrom", status = FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn chrom_speciallw_hit_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    install_status_scripts!(
        chrom_attack_pre,
        chrom_attacks3_pre,
        chrom_attackhi3_pre,
        chrom_attacklw3_pre,
        chrom_attackdash_pre,
        chrom_speciallw_pre, chrom_speciallw_main, chrom_speciallw_init, chrom_speciallw_exec,
        chrom_speciallw_hit_init, chrom_speciallw_hit_exec
    );
}