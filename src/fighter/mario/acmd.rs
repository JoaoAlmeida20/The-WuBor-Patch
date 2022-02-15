use {
    smash::{
        lua2cpp::L2CAgentBase,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    super::{
        helper::*,
        vl
    },
    wubor_utils::vars::*,
};

#[acmd_script( agent = "mario", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn mario_attacks4(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::remove(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, Hash40::new("start"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.7, 361, 105, 0, 25, 2.0, 0.0, 5.0, 6.0, Some(0.0), Some(6.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 17.8, 361, 99, 0, 25, 3.0, 0.0, 6.0, 16.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "mario", script = "effect_attacks4", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_attacks4_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 1, 6, 3, 0, -39, 10, 1.2, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.196, 0.196, 0.216);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "mario", script = "sound_attacks4", category = ACMD_SOUND, low_priority )]
unsafe fn mario_attacks4_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
	    macros::PLAY_SE(fighter, Hash40::new("vc_mario_attack05"));
    }
}

#[acmd_script( agent = "mario", script = "game_attacks4hi", category = ACMD_GAME, low_priority )]
unsafe fn mario_attacks4hi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::remove(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, Hash40::new("start"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.7, 361, 105, 0, 25, 2.0, 0.0, 5.0, 6.0, Some(0.0), Some(9.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 17.8, 361, 99, 0, 25, 3.0, 0.0, 11.0, 15.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "mario", script = "effect_attacks4hi", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_attacks4hi_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 1, 8, 3, -24, -39, 30, 1.2, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.196, 0.196, 0.216);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "mario", script = "sound_attacks4hi", category = ACMD_SOUND, low_priority )]
unsafe fn mario_attacks4hi_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
	    macros::PLAY_SE(fighter, Hash40::new("vc_mario_attack05"));
    }
}

#[acmd_script( agent = "mario", script = "game_attacks4lw", category = ACMD_GAME, low_priority )]
unsafe fn mario_attacks4lw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::remove(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, Hash40::new("start"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.7, 361, 105, 0, 25, 2.0, 0.0, 5.0, 4.0, Some(0.0), Some(4.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 17.8, 361, 99, 0, 25, 3.0, 0.0, 3.0, 15.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "mario", script = "effect_attacks4lw", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_attacks4lw_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 1, 6, 3, 12, -39, 0, 1.2, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.196, 0.196, 0.216);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "mario", script = "sound_attacks4lw", category = ACMD_SOUND, low_priority )]
unsafe fn mario_attacks4lw_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
	    macros::PLAY_SE(fighter, Hash40::new("vc_mario_attack05"));
    }
}

#[acmd_script( agent = "mario", script = "game_attackairn", category = ACMD_GAME, low_priority )]
unsafe fn mario_attackairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 5.0, 361, 90, 0, 45, 4.0, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 5.0, 361, 90, 0, 45, 4.0, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 8.0, 361, 100, 0, 20, 4.0, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 8.0, 361, 100, 0, 20, 4.0, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 5.0, 361, 90, 0, 13, 4.0, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 5.0, 361, 90, 0, 13, 4.0, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "mario", script = "game_attackairf", category = ACMD_GAME, low_priority )]
unsafe fn mario_attackairf(fighter: &mut L2CAgentBase) {
    let spike;
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, Hash40::new("light"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        spike = true;
        macros::FT_MOTION_RATE(fighter, 17.0 / 3.0);
    }
    else {
        spike = false;
        macros::FT_MOTION_RATE(fighter, 1.0 / 3.0);
    }
    frame(fighter.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 15.0);
    let angle : u64;
    let sound;
    if spike {
        angle = 290;
        sound = *ATTACK_SOUND_LEVEL_L;
    }
    else {
        angle = 361;
        sound = *ATTACK_SOUND_LEVEL_M;
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, angle, 80, 0, 30, 4.0, 0.0, 13.0, 8.0, Some(0.0), Some(3.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sound, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, angle, 80, 0, 30, 4.0, 0.0, 3.0, 13.0, Some(0.0), Some(1.5), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sound, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, Hash40::new("start"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "mario", script = "effect_attackairf", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_attackairf_eff(fighter: &mut L2CAgentBase) {
    let spike;
    frame(fighter.lua_state_agent, 10.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        spike = true;
    }
    else {
        spike = false;
    }
    frame(fighter.lua_state_agent, 12.0);
    if spike {
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 7, -1, -3, -11, -140, 1.5, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.196, 0.196, 0.216);
    }
}

#[acmd_script( agent = "mario", script = "expression_attackairf", category = ACMD_EXPRESSION, low_priority )]
unsafe fn mario_attackairf_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "mario", script = "game_landingairf", category = ACMD_GAME, low_priority )]
unsafe fn mario_landingairf(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, Hash40::new("start"), false, -1.0);
    }
}

#[acmd_script( agent = "mario", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe fn mario_attackairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 7.0, 72, 135, 0, 10, 4.4, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.0, 72, 135, 0, 10, 5.5, 3.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "mario", script = "game_attackairlw", category = ACMD_GAME, low_priority )]
unsafe fn mario_attackairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 6.0/9.0);
    frame(fighter.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 65, 90, 0, 3.0, 0.0, 0.0, -2.0, Some(0.0), Some(0.0), Some(2.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 45, 65, 90, 0, 3.0, 0.0, 0.0, -2.0, Some(0.0), Some(0.0), Some(2.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 40, 90, 0, 60, 4.0, 0.0, 0.0, -2.0, Some(0.0), Some(0.0), Some(2.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 56.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 63.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "mario", script = "effect_attackairlw", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_attackairlw_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3.0, 0, 0, 0, 0, 0.55, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3.0, 0, 0, 0, 0, 0.55, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3.0, 0, 0, 0, 0, 0.55, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 47.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3.0, 0, 0, 0, 0, 0.55, true, *EF_FLIP_YZ);
    }
}

#[acmd_script( agent = "mario", script = "sound_attackairlw", category = ACMD_SOUND, low_priority )]
unsafe fn mario_attackairlw_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_mario_002"));
        macros::PLAY_SE(fighter, Hash40::new("se_mario_attackair_l01"));
    } 
}

#[acmd_script( agent = "mario", script = "expression_attackairlw", category = ACMD_EXPRESSION, low_priority )]
unsafe fn mario_attackairlw_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackss"), 5);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 5);
    }
}

#[acmd_script( agent = "mario", script = "game_landingairlw", category = ACMD_GAME, low_priority )]
unsafe fn mario_landingairlw(_fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "mario", scripts = [ "game_specialn", "game_specialairn" ], category = ACMD_GAME, low_priority )]
unsafe fn mario_specialn(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_N_FLAG_FGC_CANCEL);
    }
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
        macros::FT_MOTION_RATE(fighter, 1.15);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, -1);
    }
}

#[acmd_script( agent = "mario", scripts = [ "game_specials", "game_specialairs" ], category = ACMD_GAME, low_priority )]
unsafe fn mario_specials(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::set_float(fighter.module_accessor, 9.0, *FIGHTER_MARIO_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP)
        && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            macros::SET_SPEED_EX(
                fighter,
                0.0,
                vl::param_special_s::spin_hop_speed_y,
                *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 65, 0, 65, 4.0, 0.0, 9.0, 10.0, Some(0.0), Some(5.0), Some(10.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 361, 50, 0, 65, 3.0, 0.0, 9.0, 7.0, Some(0.0), Some(9.0), Some(-10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 361, 50, 0, 65, 3.0, 0.0, 5.0, 7.0, Some(0.0), Some(5.0), Some(-10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "mario", script = "effect_specials", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_specials_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mario_supermant_wind_r"), Hash40::new("mario_supermant_wind_l"), Hash40::new("top"), 2.5, 5, -2.0, 22, 0, 0, 1.2, true, *EF_FLIP_NONE);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
    }
}

#[acmd_script( agent = "mario", script = "effect_specialairs", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_specialairs_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mario_supermant_wind_r"), Hash40::new("mario_supermant_wind_l"), Hash40::new("top"), 2.5, 5, -2.0, 22, 0, 0, 1.2, true, *EF_FLIP_NONE);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
    }
}

#[acmd_script( agent = "mario", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn mario_specialhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 3.0);
    frame(fighter.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    let mut effect = Hash40::new("collision_attr_coin");
    let mut sound = *COLLISION_SOUND_ATTR_COIN;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY) {
        effect.hash = hash40("collision_attr_mario_local_coin");
        sound = *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN;
    }
    if macros::is_excute(fighter) {
        macros::SA_SET(fighter, *SITUATION_KIND_AIR);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 100, 160, 0, 2.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 86, 100, 150, 0, 4.0, 0.0, 6.5, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 100, 100, 150, 0, 4.0, 0.0, 6.3, 9.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 60, 100, 180, 0, 3.0, 0.0, 6.5, 2.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.6, 92, 100, 170, 0, 3.8, 0.0, 6.5, 8.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.6, 60, 100, 110, 0, 3.0, 0.0, 11.5, 2.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.6, 92, 100, 110, 0, 3.0, 0.0, 11.5, 8.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 2, true, false);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 3, true, false);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 17.0);
    if sound == *COLLISION_SOUND_ATTR_COIN {
        sound = *COLLISION_SOUND_ATTR_MARIO_COIN_LAST;
    }
    else {
        sound = *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST;
    }
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 145, 0, 50, 9.0, 0.0, 11.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 60, 145, 0, 50, 9.0, 0.0, 11.5, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "mario", script = "sound_speciallwstart", category = ACMD_SOUND, low_priority )]
unsafe fn mario_longjumpstart_snd(_fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "mario", script = "game_speciallwlight", category = ACMD_GAME, low_priority )]
unsafe fn mario_longjump(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_LW_INT_LONG_JUMP_KIND) == FIGHTER_MARIO_LONG_JUMP_B {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_LW_FLAG_LANDING);
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_LW_FLAG_LANDING);
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_LW_INT_LONG_JUMP_KIND) == FIGHTER_MARIO_LONG_JUMP_M
        || WorkModule::get_int(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_LW_INT_LONG_JUMP_KIND) == FIGHTER_MARIO_LONG_JUMP_B {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_LW_INT_LONG_JUMP_KIND) == FIGHTER_MARIO_LONG_JUMP_W {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_LW_INT_LONG_JUMP_KIND) == FIGHTER_MARIO_LONG_JUMP_S {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

#[acmd_script( agent = "mario", script = "sound_speciallwlight", category = ACMD_SOUND, low_priority )]
unsafe fn mario_longjump_snd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_mario_009"));
    }
}

#[acmd_script( agent = "mario", script = "expression_speciallwlight", category = ACMD_EXPRESSION, low_priority )]
unsafe fn mario_longjump_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "mario", script = "sound_speciallwhold", category = ACMD_SOUND, low_priority )]
unsafe fn mario_longjumpland_snd(_fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "mario", script = "effect_speciallwhold", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_longjumpland_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "mario", script = "expression_speciallwhold", category = ACMD_EXPRESSION, low_priority )]
unsafe fn mario_longjumpland_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
    	ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "mario", script = "game_specialairlwstart", category = ACMD_GAME, low_priority )]
unsafe fn mario_groundpoundstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        KineticModule::unable_energy_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "mario", script = "sound_specialairlwstart", category = ACMD_SOUND, low_priority )]
unsafe fn mario_groundpoundstart_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_mario_special_l04"));
    }
}

#[acmd_script( agent = "mario", script = "game_specialairlwlight", category = ACMD_GAME, low_priority )]
unsafe fn mario_groundpoundfall(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 30, 50, 0, 50, 5.0, 0.0, 2.8, -2.0, Some(0.0), Some(2.8), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
    }
}

#[acmd_script( agent = "mario", script = "sound_specialairlwlight", category = ACMD_SOUND, low_priority )]
unsafe fn mario_groundpoundfall_snd(_fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "mario", script = "expression_specialairlwlight", category = ACMD_EXPRESSION, low_priority )]
unsafe fn mario_groundpoundfall_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "mario", script = "game_specialairlwheavy", category = ACMD_GAME, low_priority )]
unsafe fn mario_groundpoundcancel(_fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "mario", script = "sound_specialairlwheavy", category = ACMD_SOUND, low_priority )]
unsafe fn mario_groundpoundcancel_snd(_fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "mario", script = "game_specialairlwhold", category = ACMD_GAME, low_priority )]
unsafe fn mario_groundpoundland(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 30.0 / 25.0);
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 30, 50, 0, 60, 5.0, 0.0, 2.8, -3.0, Some(0.0), Some(2.8), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "mario", script = "sound_specialairlwhold", category = ACMD_SOUND, low_priority )]
unsafe fn mario_groundpoundland_snd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_mario_special_l03"));
    }
}

#[acmd_script( agent = "mario", script = "effect_specialairlwhold", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_groundpoundland_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "mario", script = "expression_specialairlwhold", category = ACMD_EXPRESSION, low_priority )]
unsafe fn mario_groundpoundland_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

#[acmd_script( agent = "mario_fireball", script = "game_regular", category = ACMD_GAME, low_priority )]
unsafe fn mario_fireball_regular(weapon: &mut L2CAgentBase) {
    let angle : u64;
    let bkb : i32;
    let kbg : i32;
    let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    if WorkModule::is_flag(oboma, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
        angle = 80;
        bkb = 90;
        kbg = 0;
    }
    else {
        angle = 361;
        bkb = 35;
        kbg = 20;
    }
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 5.0, angle, kbg, 0, bkb, 2.4, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MARIO_FIREBALL, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(weapon.module_accessor);
    }
    frame(weapon.lua_state_agent, 5.0);
    if macros::is_excute(weapon) {
        if !WorkModule::is_flag(oboma, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 5.0, 361, 15, 0, 28, 2.2, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MARIO_FIREBALL, *ATTACK_REGION_NONE);
        }
    }
    frame(weapon.lua_state_agent, 30.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 4.0, 361, 10, 0, 22, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MARIO_FIREBALL, *ATTACK_REGION_NONE);
    }
}

#[acmd_script( agent = "mario_pump", script = "effect_start", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_pump_start_eff(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        bonker_vis(weapon.module_accessor);
        LinkModule::set_model_constraint_pos_ort(weapon.module_accessor, *LINK_NO_ARTICLE, Hash40::new("have"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, false);
    }
}

#[acmd_script( agent = "mario_pump", script = "effect_light", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_pump_light_eff(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        bonker_vis(weapon.module_accessor);
        LinkModule::set_model_constraint_pos_ort(weapon.module_accessor, *LINK_NO_ARTICLE, Hash40::new("have"), Hash40::new("havel"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        mario_attacks4, mario_attacks4_eff, mario_attacks4_snd,
        mario_attacks4hi, mario_attacks4hi_eff, mario_attacks4hi_snd,
        mario_attacks4lw, mario_attacks4lw_eff, mario_attacks4lw_snd,
        mario_attackairn,
        mario_attackairf, mario_attackairf_eff, mario_attackairf_exp,
        mario_landingairf,
        mario_attackairhi,
        mario_attackairlw, mario_attackairlw_eff, mario_attackairlw_snd, mario_attackairlw_exp,
        mario_landingairlw,
        mario_specialn,
        mario_specials, mario_specials_eff, mario_specialairs_eff,
        mario_specialhi,
        mario_longjumpstart_snd,
        mario_longjump, mario_longjump_snd, mario_longjump_exp,
        mario_longjumpland_snd, mario_longjumpland_eff, mario_longjumpland_exp,
        mario_groundpoundstart, mario_groundpoundstart_snd,
        mario_groundpoundfall, mario_groundpoundfall_snd, mario_groundpoundfall_exp,
        mario_groundpoundcancel, mario_groundpoundcancel_snd,
        mario_groundpoundland, mario_groundpoundland_snd, mario_groundpoundland_eff, mario_groundpoundland_exp,
        mario_fireball_regular,
        mario_pump_start_eff,
        mario_pump_light_eff
    );
}