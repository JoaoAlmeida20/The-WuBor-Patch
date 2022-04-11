use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    wubor_utils::vars::*
};

#[acmd_script( agent = "edge", scripts = [ "game_appealsl", "game_appealsr" ], category = ACMD_GAME, low_priority )]
unsafe fn edge_appeals(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 39.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L | *CONTROL_PAD_BUTTON_APPEAL_S_R) {
        if macros::is_excute(fighter) {
            WorkModule::set_int(
                fighter.module_accessor,
                *CONTROL_PAD_BUTTON_APPEAL_S_L | *CONTROL_PAD_BUTTON_APPEAL_S_R,
                FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_HELD_BUTTON
            );
            WorkModule::set_int(fighter.module_accessor, 48, FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_RESTART_FRAME);
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_HOLD);
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("appeal_s_loop"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
    }
}

#[acmd_script( agent = "edge", script = "effect_appealsloop", category = ACMD_EFFECT, low_priority )]
unsafe fn edge_appealsloop_eff(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(fighter.lua_state_agent, 120.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_light"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        }
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            frame(fighter.lua_state_agent, 121.0);
            if macros::is_excute(fighter) {
                macros::EFFECT(fighter, Hash40::new("edge_sword_flash"), Hash40::new("swordl2"), 12, 0, -0.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                macros::LAST_EFFECT_SET_RATE(fighter, 0.85);
            }
        }
        else {
            frame(fighter.lua_state_agent, 124.0);
            if macros::is_excute(fighter) {
                macros::EFFECT(fighter, Hash40::new("edge_sword_flash"), Hash40::new("swordl2"), 12, 0, -0.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
        }
        frame(fighter.lua_state_agent, 135.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_EDGE_STATUS_APPEAL_WORK_FLAG_APPEAL_S_JUST);
        }
        frame(fighter.lua_state_agent, 138.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_EDGE_STATUS_APPEAL_WORK_FLAG_APPEAL_S_JUST);
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

#[acmd_script( agent = "edge", script = "sound_appealsloop", category = ACMD_SOUND, low_priority )]
unsafe fn edge_appealsloop_snd(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(fighter.lua_state_agent, 116.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_edge_appeal_s02"));
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

#[acmd_script( agent = "edge", script = "expression_appealsloop", category = ACMD_EXPRESSION, low_priority )]
unsafe fn edge_appealsloop_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    for _ in 0..i32::MAX {
        frame(fighter.lua_state_agent, 125.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(
                fighter.module_accessor,
                Hash40::new("rbkind_nohiht11"),
                0,
                false,
                *BATTLE_OBJECT_ID_INVALID as u32
            );
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

#[acmd_script( agent = "edge", script = "game_appealsattack", category = ACMD_GAME, low_priority )]
unsafe fn edge_appealsattack(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_EDGE_STATUS_APPEAL_WORK_FLAG_APPEAL_S_JUST) {
        if macros::is_excute(fighter) {
            damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
            macros::SLOW_OPPONENT(fighter, 20.0, 30.0);
        }
        macros::FT_MOTION_RATE(fighter, 30.0 / 21.0);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 8.0, 3.0);
    }
    frame(fighter.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        let damage;
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_EDGE_STATUS_APPEAL_WORK_FLAG_APPEAL_S_JUST) {
            damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            damage = 700.0;
        }
        else {
            damage = 20.0;
        }
        macros::ATTACK(fighter, 0, 0, Hash40::new("swordl1"), damage, 361, 81, 0, 54, 4.0, 0.0, -1.0, 4.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("swordl1"), damage, 361, 78, 0, 57, 4.0, 7.0, -1.0, 4.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("swordl1"), damage, 361, 78, 0, 57, 4.0, 15.0, -1.0, 4.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 4, 0, Hash40::new("swordl1"), damage, 361, 81, 0, 54, 5.8, 21.5, -1.0, 4.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), damage, 361, 81, 0, 54, 5.8, 0.0, 8.5, 6.5, Some(0.0), Some(8.5), Some(6.5), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 5, 0, Hash40::new("top"), damage, 361, 78, 0, 57, 5.5, 0.0, 9.0, 16.5, Some(0.0), Some(9.0), Some(21.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.0, 3.0);
    }
}

#[acmd_script( agent = "edge", script = "effect_appealsattack", category = ACMD_EFFECT, low_priority )]
unsafe fn edge_appealsattack_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, 13, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_EDGE_STATUS_APPEAL_WORK_FLAG_APPEAL_S_JUST) {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("edge_aura"), Hash40::new("hip"), -2, -2, 0, 80, 90, 0, 1, true);
        }
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_smash_flash"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordl2"), -4.0, 0.0, -0.7, Hash40::new("swordl2"), 29.2, 0.0, 1.5, true, Hash40::new("null"), Hash40::new("swordl2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_flare"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 3.0);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 10, 0, 4, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("edge_sword_flare"), false, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("edge_sword"), Hash40::new("top"), -1, 13.2, 3.5, 0, -50, 24, 0.95, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("edge_slash_light"), Hash40::new("top"), -1, 13.2, 3.5, 0, 40, 24, 1.05, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 2);
    }
}

#[acmd_script( agent = "edge", script = "sound_appealsattack", category = ACMD_SOUND, low_priority )]
unsafe fn edge_appealsattack_snd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_edge_smash_s01"));
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_EDGE_STATUS_APPEAL_WORK_FLAG_APPEAL_S_JUST) {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("vc_edge_win03"));
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_edge_smash_s02"));
    }
    frame(fighter.lua_state_agent, 17.0);
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_EDGE_STATUS_APPEAL_WORK_FLAG_APPEAL_S_JUST) {
        if macros::is_excute(fighter) {
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_edge_rnd_attack_smash"));
        }
    }
    frame(fighter.lua_state_agent, 18.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_edge_winged_smash_s01"));
        }
    }
    frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_edge_smash_s03"));
    }
    frame(fighter.lua_state_agent, 58.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_edge_step_right_s"));
    }
    frame(fighter.lua_state_agent, 69.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_edge_step_left_s"));
    }
}

#[acmd_script( agent = "edge", script = "expression_appealsattack", category = ACMD_EXPRESSION, low_priority )]
unsafe fn edge_appealsattack_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(
            fighter.module_accessor,
            Hash40::new("top"),
            AttackDirectionAxis(*ATTACK_DIRECTION_Z),
            AttackDirectionAxis(*ATTACK_DIRECTION_X_MINUS),
            AttackDirectionAxis(*ATTACK_DIRECTION_Y)
        );
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_78_smash"), 30);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohitl_l"),
            9,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

#[acmd_script( agent = "edge", scripts = [ "game_appeallwl", "game_appeallwr" ], category = ACMD_GAME, low_priority )]
unsafe fn edge_appeallw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 39.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
        if macros::is_excute(fighter) {
            WorkModule::set_int(
                fighter.module_accessor,
                *CONTROL_PAD_BUTTON_APPEAL_LW,
                FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_HELD_BUTTON
            );
            WorkModule::set_int(fighter.module_accessor, 60, FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_RESTART_FRAME);
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_HOLD);
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("appeal_lw_loop"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
    }
}

#[acmd_script( agent = "edge", script = "effect_appeallwloop", category = ACMD_EFFECT, low_priority )]
unsafe fn edge_appeallwloop_eff(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("edge_aura"), Hash40::new("hip"), -2, -2, 0, 80, 90, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 60.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("edge_aura"), Hash40::new("hip"), -2, -2, 0, 80, 90, 0, 1, true);
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

#[acmd_script( agent = "edge", script = "expression_appeallwloop", category = ACMD_EXPRESSION, low_priority )]
unsafe fn edge_appeallwloop_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    for _ in 0..i32::MAX {
        frame(fighter.lua_state_agent, 70.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(
                fighter.module_accessor,
                Hash40::new("rbkind_elecattack"),
                100,
                true,
                *BATTLE_OBJECT_ID_INVALID as u32
            );
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

pub fn install() {
    install_acmd_scripts!(
        edge_appeals,
        edge_appealsloop_eff, edge_appealsloop_snd, edge_appealsloop_exp,
        edge_appealsattack, edge_appealsattack_eff, edge_appealsattack_snd, edge_appealsattack_exp,
        edge_appeallw,
        edge_appeallwloop_eff, edge_appeallwloop_exp
    );
}