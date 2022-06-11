use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::lua_const::*
    },
    wubor_utils::{wua_bind::*, table_const::*}
};

pub unsafe extern "C" fn chrom_fgc(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    let mut special_cancels : Vec<i32> = [].to_vec();
    let mut normal_cancels : Vec<i32> = [].to_vec();
    if [
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
    ].contains(&status) {
        special_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
        ].to_vec();
        normal_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
        ].to_vec();
    }
    else if [
        *FIGHTER_STATUS_KIND_ATTACK_AIR
    ].contains(&status) {
        if FGCModule::air_dash_cancel_check(fighter, false).get_bool() {
            return;
        }
    }
    FGCModule::cancel_system(fighter, normal_cancels, special_cancels, false, 0);
}
