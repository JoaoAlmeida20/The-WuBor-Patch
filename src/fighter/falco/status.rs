use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*}
};

#[status_script(agent = "falco", status = FIGHTER_STATUS_KIND_APPEAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn falco_appeal_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = fighter.status_Appeal();
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_lw_l")
    || MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_lw_r") {
        VarModule::on_flag(fighter.battle_object, falco::instance::flag::KAA);
    }
    else {
        VarModule::off_flag(fighter.battle_object, falco::instance::flag::KAA);
    }
    ret
}

#[status_script(agent = "falco", status = FIGHTER_STATUS_KIND_APPEAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn falco_appeal_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ATTACK_LW4_START {
        VarModule::off_flag(fighter.battle_object, falco::instance::flag::KAA);
    }
    fighter.status_end_Appeal()
}

#[status_script(agent = "falco", status = FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn falco_attacks4hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, falco::instance::flag::KAA);
    fighter.status_end_AttackLw4Hold()
}

#[status_script(agent = "falco", status = FIGHTER_STATUS_KIND_ATTACK_LW4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn falco_attacks4_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, falco::instance::flag::KAA);
    fighter.status_end_AttackLw4()
}

pub fn install() {
    install_status_scripts!(
        falco_appeal_main,
        falco_appeal_end,
        falco_attacks4hold_end,
        falco_attacks4_end
    );
}