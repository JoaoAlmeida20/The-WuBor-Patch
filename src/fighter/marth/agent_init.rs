use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    custom_status::*,
    super::{status::helper::*, vars::*}
};

pub unsafe extern "C" fn marth_check_ground_special_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)
    && WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE) {
        if marth_stance_ground_cancel_helper(fighter).get_bool() {
            return true.into();
        }
        let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ENTER);
        fighter.change_status(status.into(), false.into());
        return true.into();
    }
    false.into()
}

pub unsafe extern "C" fn marth_check_air_special_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)
    && WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE) {
        let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ENTER);
        fighter.change_status(status.into(), false.into());
        return true.into();
    }
    false.into()
}

pub unsafe extern "C" fn marth_speciallw_pre(_fighter: &mut L2CFighterCommon) -> L2CValue {
    false.into()
}
