use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    wubor_utils::table_const::*,
    super::status::*
};

unsafe fn set_move_customizer(fighter: &mut L2CFighterCommon, customizer: unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue) {
    if fighter.global_table["move_customizer_set"].get_bool() {
        return;
    }

    let clone = fighter.global_table[WAZA_CUSTOMIZE_CONTROL].clone();
    fighter.global_table["move_customizer_set"].assign(&true.into());
    fighter.global_table["move_customizer_original"].assign(&clone);
    fighter.global_table[WAZA_CUSTOMIZE_CONTROL].assign(&L2CValue::Ptr(customizer as *const () as _));
}

unsafe fn get_original_customizer(fighter: &mut L2CFighterCommon) -> Option<unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue> {
    let ptr = fighter.global_table["move_customizer_original"].get_ptr();
    if !ptr.is_null() {
        Some(std::mem::transmute(ptr))
    } else {
        None
    }
}

unsafe extern "C" fn jack_move_customizer(fighter: &mut L2CFighterCommon) -> L2CValue {
    let customize_to = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_1 {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_S.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), &mut *(jack_specials_main as *const () as *mut libc::c_void));
        0.into()
    } else if let Some(original) = get_original_customizer(fighter) {
        original(fighter)
    } else {
        0.into()
    }
}

#[fighter_reset]
fn fighter_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind == *FIGHTER_KIND_JACK {
            set_move_customizer(fighter, jack_move_customizer);
        }
    }
}

pub fn install() {
    install_agent_resets!(
        fighter_reset
    );
}
