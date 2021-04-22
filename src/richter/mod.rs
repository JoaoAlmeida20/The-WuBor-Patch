use smash::phx::Hash40;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;

pub static mut RICHTER_SPECIAL_HI : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_RICHTER )]
fn richter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH {
            RICHTER_SPECIAL_HI[entry_id] = false;
        }
        if sv_information::is_ready_go() == false {
            RICHTER_SPECIAL_HI[entry_id] = false;
        }

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_HI && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_LANDING {
            RICHTER_SPECIAL_HI[entry_id] = true;
        }
        else if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_AIR
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_THROWN
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CAPTURE_WAIT
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY 
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR 
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U 
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FALL
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_FINAL
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SLEEP
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ESCAPE_B
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ESCAPE_F
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SWALLOWED
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_AIR_LASSO
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CATCHED_REFLET
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CATCHED_RIDLEY
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_AIR
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_MISS_FOOT
        || WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI)
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DEAD
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_BURY {
            RICHTER_SPECIAL_HI[entry_id] = false;
        }
    }
}

#[acmd_script( agent = "richter", scripts = ["game_specialn", "game_specialairn"], category = ACMD_GAME, low_priority )]
unsafe fn richter_nspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    macros::FT_MOTION_RATE(fighter, 1.3);
    if macros::is_excute(fighter){
        ArticleModule::generate_article(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, false, 0);
    }
    sv_animcmd::frame(lua_state, 30.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
}

#[acmd_script( agent = "richter", scripts = ["game_specials1", "game_specialairs1"], category = ACMD_GAME, low_priority )]
unsafe fn richter_sspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.56);
    if macros::is_excute(fighter){
        ArticleModule::generate_article(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_CROSS, false, 0);
    }
    sv_animcmd::frame(lua_state, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL);
    }
    sv_animcmd::frame(lua_state, 16.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_CROSS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    sv_animcmd::frame(lua_state, 26.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL);
    }
}

#[acmd_script( agent = "richter", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME, low_priority )]
unsafe fn richter_dspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    macros::FT_MOTION_RATE(fighter, 1.3);
    if macros::is_excute(fighter){
        WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_GENERATE_HOLYWATER);
    }
    sv_animcmd::frame(lua_state, 18.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_SHOOT_HOLYWATER);
    }
}

#[acmd_script( agent = "richter", scripts = ["game_specialhi", "game_specialairhi"], category = ACMD_GAME, low_priority )]
unsafe fn richter_uspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    sv_animcmd::frame(lua_state, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 95, 100, 155, 0, 8.5, 0.0, 9.5, 10.5, None, None, None, 1.4, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 80, 0, 6.0, 0.0, 26.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    sv_animcmd::frame(lua_state, 20.0);
    if macros::is_excute(fighter) {
        smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    sv_animcmd::frame(lua_state, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
        smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 6.0, 61, 86, 0, 85, 6.0, 0.0, 26.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
    }
    sv_animcmd::frame(lua_state, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
    }
    sv_animcmd::frame(lua_state, 47.0);
    if macros::is_excute(fighter) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        richter_frame
    );
    smashline::install_acmd_scripts!(
        richter_nspecial,
        richter_sspecial,
        richter_dspecial,
        richter_uspecial
    );
}