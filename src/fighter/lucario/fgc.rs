use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    wubor_utils::{
        wua_bind::*,
        vars::*
    }
};

pub unsafe extern "C" fn lucario_fgc(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
        let status = StatusModule::status_kind(fighter.module_accessor);
        let mut special_cancels : Vec<i32> = [].to_vec();
        let mut normal_cancels : Vec<i32> = [].to_vec();
        let mut jump_cancel = 0;
        MiscModule::set_hp(fighter, 116.0);
        
        if [
            *FIGHTER_STATUS_KIND_ATTACK
        ].contains(&status) {
            special_cancels = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
            ].to_vec();
            normal_cancels = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3
            ].to_vec();
        }
        else if [
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_ATTACK_AIR
        ].contains(&status) {
            if status == *FIGHTER_STATUS_KIND_ATTACK_S3 {
                if FGCModule::cancel_exceptions(fighter, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3, true).get_bool() {
                    return;
                }
            }
            special_cancels = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
            ].to_vec();
            normal_cancels = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
            ].to_vec();
        }
        else if [
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4
        ].contains(&status) {
            special_cancels = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
            ].to_vec();
        }
        else if status == *FIGHTER_STATUS_KIND_ATTACK_DASH {
            jump_cancel = 1;
        }
        else if status == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH
        && (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_IS_SUPER_DASH_CANCEL);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END, true);
        }
        if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI) {
            special_cancels.append(&mut [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI].to_vec());
        }
        FGCModule::cancel_system(fighter, normal_cancels, special_cancels, false, jump_cancel);
    }
}