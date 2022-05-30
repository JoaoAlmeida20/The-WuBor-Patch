use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_status::*,
    wubor_utils::table_const::*,
    super::{status::helper::*, vars::*}
};

pub unsafe extern "C" fn marth_check_ground_special_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)
    && WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE) {
        if marth_stance_special_cancel_helper(fighter).get_bool()
        || marth_stance_ground_cancel_helper(fighter).get_bool() {
            return true.into();
        }
        let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ENTER);
        let clear_buffer = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0;
        fighter.change_status(status.into(), clear_buffer.into());
        return true.into();
    }
    false.into()
}

pub unsafe extern "C" fn marth_check_air_special_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)
    && WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE) {
        if marth_stance_special_cancel_helper(fighter).get_bool() {
            return true.into();
        }
        let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ENTER);
        fighter.change_status(status.into(), false.into());
        return true.into();
    }
    false.into()
}

pub unsafe extern "C" fn marth_speciallw_pre(_fighter: &mut L2CFighterCommon) -> L2CValue {
    false.into()
}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_MARTH {
            return;
        }
        fighter.global_table[CHECK_GROUND_SPECIAL_UNIQ].assign(&L2CValue::Ptr(marth_check_ground_special_pre as *const () as _));
        fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].assign(&L2CValue::Ptr(marth_check_air_special_pre as *const () as _));
        fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(marth_speciallw_pre as *const () as _));
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
