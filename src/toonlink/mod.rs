use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smash::app::sv_animcmd;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smash_script::macros;
//use smash::phx::Vector3f;
//use smash::app::BattleObjectModuleAccessor;
//use smash::app::lua_bind::EffectModule;

#[fighter_frame( agent = FIGHTER_KIND_TOONLINK )]
unsafe fn toonlink_frame(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if MotionModule::motion_kind(boma) == smash::hash40("special_hi") {
        if MotionModule::frame(boma) > 6.0 && MotionModule::frame(boma) < 46.0 {
            let facing_dirn = PostureModule::lr(boma);
            if facing_dirn > 0.0 {
                macros::SET_SPEED_EX(fighter, 1.76 * ControlModule::get_stick_x(boma), 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            else{
                macros::SET_SPEED_EX(fighter, -1.76 * ControlModule::get_stick_x(boma), 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        }
    }
}

#[script( agent = "toonlink", script = "game_attackdash", category = ACMD_GAME )]
unsafe fn toonlink_dashattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    macros::FT_MOTION_RATE(fighter, 0.7);
    sv_animcmd::frame(lua_state, 8.0);
    macros::FT_MOTION_RATE(fighter, 1.1);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("sword2"), 8.0, 82, 70, 0, 55, 4.2, 5.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 8.0, 82, 70, 0, 55, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 8.0, 82, 70, 0, 55, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(lua_state, 2.0);
    macros::FT_MOTION_RATE(fighter, 0.6186);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 43.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

pub fn install() {
    smash_script::replace_fighter_frames!(
        toonlink_frame
    );
    smash_script::replace_scripts!(
        toonlink_dashattack
    );
}