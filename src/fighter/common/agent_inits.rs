use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*,
        table_const::*,
        fighter::{
            mario::agent_init::mario_speciallw_pre,
            daisy::agent_init::{
                daisy_speciallw_pre,
                daisy_itemtoss_pre
            },
            kirby::agent_init::kirby_specialn_pre,
            kamui::agent_init::kamui_escapeair_pre,
            ken::agent_init::ken_speciallw_pre,
            lucario::agent_init::lucario_specialhi_pre,
            lucina::agent_init::{
                yu_specialns_pre,
                yu_speciallw_pre,
                yu_check_special_command
            }
        }
    }
};

unsafe extern "C" fn specialn_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue {
    if DISABLE_SPECIAL_N[entry_id(fighter.module_accessor)] {
        return 0.into();
    }
    1.into()
}

unsafe extern "C" fn specials_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue {
    if DISABLE_SPECIAL_S[entry_id(fighter.module_accessor)] {
        return 0.into();
    }
    1.into()
}

unsafe extern "C" fn specialhi_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue {
    if DISABLE_SPECIAL_HI[entry_id(fighter.module_accessor)] {
        return 0.into();
    }
    1.into()
}

unsafe extern "C" fn speciallw_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue {
    if DISABLE_SPECIAL_LW[entry_id(fighter.module_accessor)] {
        return 0.into();
    }
    1.into()
}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind == *FIGHTER_KIND_MARIO {
            fighter.global_table[SPECIAL_LW_PRE].assign(&L2CValue::Ptr(mario_speciallw_pre as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_KIRBY {
            fighter.global_table[SPECIAL_N_PRE].assign(&L2CValue::Ptr(kirby_specialn_pre as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_DAISY {
            fighter.global_table[CHECK_AIR_SPECIAL_PRE].assign(&L2CValue::Bool(false));
            fighter.global_table[CHECK_GROUND_ATTACK_PRE].assign(&L2CValue::Ptr(daisy_itemtoss_pre as *const () as _));
            fighter.global_table[CHECK_AIR_ITEM_THROW_PRE].assign(&L2CValue::Bool(false));
            fighter.global_table[CHECK_AIR_JUMP_PRE].assign(&L2CValue::Bool(false));
            fighter.global_table[CHECK_AIR_JUMP_AERIAL_POST].assign(&L2CValue::Bool(false));
            fighter.global_table[SPECIAL_S_PRE].assign(&L2CValue::Ptr(specials_pre_generic as *const () as _));
            fighter.global_table[SPECIAL_LW_PRE].assign(&L2CValue::Ptr(daisy_speciallw_pre as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_PEACH {
            fighter.global_table[CHECK_GROUND_ATTACK_PRE].assign(&L2CValue::Ptr(daisy_itemtoss_pre as *const () as _));
            fighter.global_table[CHECK_AIR_ITEM_THROW_PRE].assign(&L2CValue::Ptr(daisy_itemtoss_pre as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_GANON {
            fighter.global_table[SPECIAL_N_PRE].assign(&L2CValue::Ptr(specialn_pre_generic as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_LUCINA {
            fighter.global_table[SPECIAL_N_PRE].assign(&L2CValue::Ptr(yu_specialns_pre as *const () as _));
            fighter.global_table[SPECIAL_S_PRE].assign(&L2CValue::Ptr(yu_specialns_pre as *const () as _));
            fighter.global_table[SPECIAL_LW_PRE].assign(&L2CValue::Ptr(yu_speciallw_pre as *const () as _));
            fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(yu_check_special_command as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_LUCARIO {
            fighter.global_table[SPECIAL_HI_PRE].assign(&L2CValue::Ptr(lucario_specialhi_pre as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_SHULK {
            fighter.global_table[SPECIAL_LW_PRE].assign(&L2CValue::Ptr(speciallw_pre_generic as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_KEN {
            fighter.global_table[SPECIAL_LW_PRE].assign(&L2CValue::Ptr(ken_speciallw_pre as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_KAMUI {
            fighter.global_table[CHECK_AIR_ESCAPE_PRE].assign(&L2CValue::Ptr(kamui_escapeair_pre as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_RICHTER {
            fighter.global_table[SPECIAL_HI_PRE].assign(&L2CValue::Ptr(specialhi_pre_generic as *const () as _));
        }
        // else if fighter_kind == *FIGHTER_KIND_RYU {
        //     fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Bool(false));
        // }
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}