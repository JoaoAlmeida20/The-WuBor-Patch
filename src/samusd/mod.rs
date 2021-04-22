use smash::phx::Hash40;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon, L2CFighterBase};
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
use smash::phx::Vector3f;
use smash::phx::Vector2f;
use crate::IS_FUNNY;
use crate::commonfuncs::*;

static mut BOUNCE : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_SAMUSD )]
fn samusd_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let boma = sv_system::battle_object_module_accessor(lua_state);

        if get_player_number(boma) < 8 {

            if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH {
                BOUNCE[get_player_number(boma)] = false;
            }
            if sv_information::is_ready_go() == false {
                BOUNCE[get_player_number(boma)] = false;
            }
        
            // Morph Ball Drop Bounce
            if MotionModule::motion_kind(boma) == smash::hash40("special_lw")
            || MotionModule::motion_kind(boma) == smash::hash40("special_air_lw") {
                if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
                || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD))
                && BOUNCE[get_player_number(boma)] == false {
                    MotionModule::set_frame(boma, 44.0, true);
                    WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
                    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                    macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                    KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                    KineticModule::add_speed(boma, &Vector3f{x: 0.0,y: 0.5,z: 0.0});
                    BOUNCE[get_player_number(boma)] = true;
                }
            }
            else {
                if BOUNCE[get_player_number(boma)] {
                    KineticModule::add_speed(boma, &Vector3f{x: 0.0,y: 0.25,z: 0.0});
                    BOUNCE[get_player_number(boma)] = false;
                }
            }
        }
    }
}

#[weapon_frame( agent = WEAPON_KIND_SAMUSD_CSHOT )]
fn samusd_cshot_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(weapon.lua_state_agent);
        if MotionModule::motion_kind(boma) == smash::hash40("shoot") {
            let slowdownvec : Vector3f = Vector3f{x: 0.9,y: 0.0,z: 0.0};
            KineticModule::mul_speed(boma, &slowdownvec, *WEAPON_KINETIC_TYPE_NONE);
        }
    }
}

#[acmd_script( agent = "samusd", script = "game_attack11", category = ACMD_GAME, low_priority )]
unsafe fn samusd_jab1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
    sv_animcmd::frame(lua_state, 3.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 10, 0, 25, 2.0, 0.0, 10.0, 6.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 361, 10, 0, 25, 2.3, 0.0, 10.0, 8.8, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 361, 10, 0, 25, 2.5, 0.0, 10.0, 12.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 4.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 4.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 4.0, false);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

#[acmd_script( agent = "samusd", script = "game_attack12", category = ACMD_GAME, low_priority )]
unsafe fn samusd_jab2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 5.0);
    macros::FT_MOTION_RATE(fighter, 1.2);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 0, 75, 0, 50, 6.5, 0.0, 8.0, 14.0, None, None, None, 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, -7.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "samusd", script = "effect_attack12", category = ACMD_EFFECT, low_priority )]
unsafe fn samusd_jab2effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new_raw(0x15368a2a9c), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 1.3, true, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, 3, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(fighter, Hash40::new_raw(0x0f4b6cda63), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    sv_animcmd::frame(lua_state, 35.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x15368a2a9c), false, true);
    }
}

#[acmd_script( agent = "samusd", script = "expression_attack12", category = ACMD_EXPRESSION, low_priority )]
unsafe fn samusd_jab2ex(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(boma, smash::hash40("body") as i64, smash::hash40("body_hide_gun") as i64);
        ArticleModule::remove_exist(boma, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(boma, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN, Hash40::new_raw(0x03dbd0dc6a), false, 0.0);
    }
    macros::FT_MOTION_RATE(fighter, 2.0);
    sv_animcmd::frame(lua_state, 4.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackll"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
    }
    sv_animcmd::frame(lua_state, 35.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	    VisibilityModule::set_int64(boma, smash::hash40("body") as i64, smash::hash40("body_normal") as i64);
    }
}

#[acmd_script( agent = "samusd", script = "sound_attack12", category = ACMD_SOUND, low_priority )]
unsafe fn samusd_jab2sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_smash_s01"));
    }
    sv_animcmd::wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_smash_s02"));
    }
}

#[acmd_script( agent = "samusd", script = "game_attackdash", category = ACMD_GAME, low_priority )]
unsafe fn samusd_dashattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 8.0);
    for _ in 0..4 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.8, 10, 0, 50, 38, 4.3, 0.0, 9.0, 3.0, Some(0.0), Some(0.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
        sv_animcmd::wait(lua_state, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        sv_animcmd::wait(lua_state, 1.0);
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 72, 40, 0, 120, 5.5, 0.0, 11.0, 3.0, Some(0.0), Some(0.0), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "samusd", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn samusd_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 8.0, 67, 80, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 9.0, 67, 100, 0, 30, 3.0, 0.6, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 10.0, 67, 100, 0, 30, 3.0, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "samusd", script = "game_attacks3hi", category = ACMD_GAME, low_priority )]
unsafe fn samusd_ftilthi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 9.0, 67, 80, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 10.0, 67, 100, 0, 30, 3.0, 0.6, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 11.0, 67, 100, 0, 30, 3.0, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    sv_animcmd::wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "samusd", script = "game_attacks3lw", category = ACMD_GAME, low_priority )]
unsafe fn samusd_ftiltlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 9.0, 67, 80, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 10.0, 67, 100, 0, 30, 3.0, 0.6, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 11.0, 67, 100, 0, 30, 3.0, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "samusd", script = "game_attackhi3", category = ACMD_GAME, low_priority )]
unsafe fn samusd_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    sv_animcmd::frame(lua_state, 5.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(lua_state, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 9.0, 78, 110, 0, 50, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 9.0, 78, 110, 0, 50, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    sv_animcmd::wait(lua_state, 7.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "samusd", script = "effect_attackhi3", category = ACMD_EFFECT, low_priority )]
unsafe fn samusd_utilteffect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x10c8b4f661), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x10c8b4f661), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2.0, true);
        macros::BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
    }
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -6, 20, -2, 1.7, -39, 92, 1.65, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.3);
    }
    sv_animcmd::wait(lua_state, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x10c8b4f661), false, true);
        macros::BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
        macros::BURN_COLOR_NORMAL(fighter);
    }
}

#[acmd_script( agent = "samusd", script = "sound_attackhi3", category = ACMD_SOUND, low_priority )]
unsafe fn samusd_utiltsound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_attackhard_h01"));
    }
}

#[acmd_script( agent = "samusd", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn samusd_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.0, 40, 50, 0, 80, 3.8, 0.0, 0.0, 0.0, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 40, 50, 0, 80, 7.2, 0.0, 1.6, 14.4, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
    }
    sv_animcmd::wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "samusd", script = "game_attackhi4", category = ACMD_GAME, low_priority )]
unsafe fn samusd_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(lua_state, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 3.0, 100, 30, 0, 60, 4.5, -3.5, 0.0, 0.0, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.0, 368, 30, 0, 0, 5.0, 6.5, 0.0, 0.0, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 3.0, 368, 30, 0, 0, 5.0, 6.5, 0.0, 0.0, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 4.0, y: 22.0} as *const Vector2f, 4, false);
        AttackModule::set_vec_target_pos(boma, 2, Hash40::new("top"), &Vector2f{x: 4.0, y: 22.0} as *const Vector2f, 4, false);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 3.0, 130, 30, 0, 30, 4.5, -3.5, 0.0, 0.0, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.0, 368, 30, 0, 0, 5.0, 6.5, 0.0, 0.0, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 3.0, 160, 30, 0, 30, 5.0, 6.5, 0.0, 0.0, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: -1.0, y: 23.0} as *const Vector2f, 10, false);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 3.0, 130, 30, 0, 30, 4.5, -3.5, 0.0, 0.0, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.0, 368, 30, 0, 0, 5.0, 6.5, 0.0, 0.0, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 3.0, 160, 30, 0, 30, 5.0, 6.5, 0.0, 0.0, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: -6.0, y: 22.0} as *const Vector2f, 10, false);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 3.0, 120, 30, 0, 30, 4.5, -3.5, 0.0, 0.0, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.0, 368, 30, 0, 0, 5.0, 6.5, 0.0, 0.0, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 3.0, 170, 30, 0, 30, 5.0, 6.5, 0.0, 0.0, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: -10.0, y: 20.0} as *const Vector2f, 10, false);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0, 145, 162, 0, 50, 4.5, -3.5, 0.0, 0.0, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 6.0, 145, 162, 0, 50, 8.5, 5.5, 0.0, 0.0, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "samusd", script = "game_attacklw4", category = ACMD_GAME, low_priority )]
unsafe fn samusd_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 10.0, 80, 70, 0, 70, 4.2, 8.0, -0.5, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 17.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 12.0, 80, 70, 0, 70, 4.6, 8.0, -0.5, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "samusd", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe fn samusd_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 10.0, 66, 96, 0, 20, 4.5, 3.2, 2.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 9.0, 66, 96, 0, 20, 5.0, 6.2, 0.9, -0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(lua_state, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 9.0, 30, 80, 0, 12, 4.5, 3.2, 2.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 8.0, 30, 80, 0, 12, 5.0, 6.2, 0.9, -0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "samusd", script = "effect_attackairhi", category = ACMD_EFFECT, low_priority )]
unsafe fn samusd_uaireffect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x10c8b4f661), Hash40::new("legl"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x10c8b4f661), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 2.0, true);
        macros::BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
    }
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 14, 3, 1.7, 100, 90, 1.2, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    sv_animcmd::wait(lua_state, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x10c8b4f661), false, true);
        macros::BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
        macros::BURN_COLOR_NORMAL(fighter);
    }
}

#[acmd_script( agent = "samusd", script = "game_attackairn", category = ACMD_GAME, low_priority )]
unsafe fn samusd_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 7.0);
    for _ in 0..5 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 185, 100, 30, 20, 2.5, 0.0, 11.0, 4.1, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.6, 185, 100, 30, 20, 2.5, 0.0, 11.0, -6.1, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.6, 367, 100, 60, 20, 2.5, 0.0, 2.0, 4.1, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.6, 367, 100, 60, 20, 2.5, 0.0, 2.0, -6.1, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        }
        sv_animcmd::wait(lua_state, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        sv_animcmd::wait(lua_state, 2.0);
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 135, 0, 40, 13.5, 0.0, 7.0, -2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 46.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "samusd", script = "game_attackairf", category = ACMD_GAME, low_priority )]
unsafe fn samusd_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.0, 53, 100, 0, 40, 3.3, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.0, 53, 100, 0, 40, 3.3, -4.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 12.0, 53, 100, 0, 40, 4.5, 7.5, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "samusd", script = "game_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn samusd_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 10.0, 67, 90, 0, 42, 4.5, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 12.0, 67, 90, 0, 42, 4.5, 6.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 7.0, 67, 90, 0, 42, 4.5, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.0, 67, 90, 0, 42, 4.5, 6.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(lua_state, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 42.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "samusd", script = "game_special", category = ACMD_GAME, low_priority )]
unsafe fn samusd_special(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.667);
    sv_animcmd::frame(lua_state, 18.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
    }
}

#[acmd_script( agent = "samusd", script = "game_specialair", category = ACMD_GAME, low_priority )]
unsafe fn samusd_specialair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.667);
    sv_animcmd::frame(lua_state, 18.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
    }
    sv_animcmd::frame(lua_state, 27.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
    }
}

#[acmd_script( agent = "samusd", script = "game_specials", category = ACMD_GAME, low_priority )]
unsafe fn samusd_sspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.667);
    sv_animcmd::frame(lua_state, 21.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
    }
}

#[acmd_script( agent = "samusd", script = "game_specialairs", category = ACMD_GAME, low_priority )]
unsafe fn samusd_sspecialair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.667);
    sv_animcmd::frame(lua_state, 21.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
    }
    sv_animcmd::frame(lua_state, 27.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
    }
}

#[acmd_script( agent = "samusd", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn samusd_dspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    sv_animcmd::frame(lua_state, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    sv_animcmd::frame(lua_state, 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_special_l01"));
        if IS_FUNNY[get_player_number(boma)] { 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 45, 30, 0, 80, 5.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, 0, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        VisibilityModule::set_int64(boma, smash::hash40("body") as i64, smash::hash40("body_sphere") as i64);
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    sv_animcmd::frame(lua_state, 33.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
    if BOUNCE[get_player_number(boma)] == false {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 30, 0, 70, 5.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::SET_SPEED_EX(fighter, 0, -3.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        }
    }
    sv_animcmd::frame(lua_state, 45.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
        VisibilityModule::set_int64(boma, smash::hash40("body") as i64, smash::hash40("body_normal") as i64);
    }
    macros::FT_MOTION_RATE(fighter, 0.6);
}

#[acmd_script( agent = "samusd", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn samusd_dspecialair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_special_l01"));
        if IS_FUNNY[get_player_number(boma)] { 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 45, 30, 0, 80, 5.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, 0, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        VisibilityModule::set_int64(boma, smash::hash40("body") as i64, smash::hash40("body_sphere") as i64);
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    sv_animcmd::frame(lua_state, 33.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
    if BOUNCE[get_player_number(boma)] == false {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 30, 0, 80, 5.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::SET_SPEED_EX(fighter, 0, -3.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        }
    }
    sv_animcmd::frame(lua_state, 45.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
        VisibilityModule::set_int64(boma, smash::hash40("body") as i64, smash::hash40("body_normal") as i64);
    }
    macros::FT_MOTION_RATE(fighter, 0.6);
}

#[acmd_script( agent = "samusd_cshot", script = "game_shoot", category = ACMD_GAME, low_priority )]
unsafe fn samusd_cshot_shoot(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, -60, 0, -150, 1.9, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 78, 20, 0, 50, 5.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        smash_script::attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
    }
}

#[acmd_script( agent = "samusd_missile", script = "game_homing", category = ACMD_GAME, low_priority )]
unsafe fn samusd_missile_homing(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 25, 0, 26, 2.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
}

#[acmd_script( agent = "samusd_supermissile", script = "game_ready", category = ACMD_GAME, low_priority )]
unsafe fn samusd_supermissile_ready(_fighter: &mut L2CAgentBase) {
}

pub fn install() {
    smashline::install_agent_frames!(
        samusd_frame,
        samusd_cshot_frame
    );
    smashline::install_acmd_scripts!(
        samusd_jab1,
        samusd_jab2,
        samusd_jab2effect,
        samusd_jab2ex,
        samusd_jab2sound,
        samusd_dashattack,
        samusd_ftilt,
        samusd_ftilthi,
        samusd_ftiltlw,
        samusd_utilt,
        samusd_utilteffect,
        samusd_utiltsound,
        samusd_dtilt,
        samusd_usmash,
        samusd_dsmash,
        samusd_uair,
        samusd_uaireffect,
        samusd_nair,
        samusd_fair,
        samusd_bair,
        samusd_special,
        samusd_specialair,
        samusd_sspecial,
        samusd_sspecialair,
        samusd_dspecial,
        samusd_dspecialair,
        samusd_cshot_shoot,
        samusd_missile_homing,
        samusd_supermissile_ready
    );
}