use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*}
};

#[fighter_frame( agent = FIGHTER_KIND_TOONLINK )]
fn toonlink_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // Down Air Bounce

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw"){
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                VarModule::on_flag(fighter.battle_object, toonlink::attack_air_lw::flag::BOUNCE);
            }
            if VarModule::is_flag(fighter.battle_object, toonlink::attack_air_lw::flag::BOUNCE) {
                macros::SET_SPEED_EX(fighter, 0.0, 0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                KineticModule::suspend_energy_all(fighter.module_accessor);
                if !fighter.global_table[IS_STOP].get_bool()
                && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
                && MotionModule::frame(fighter.module_accessor) < 65.0 {
                    MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 65.0, true, true, false);
                }
                else if MotionModule::frame(fighter.module_accessor) > 66.0 {
                    KineticModule::resume_energy_all(fighter.module_accessor);
                    VarModule::off_flag(fighter.battle_object, toonlink::attack_air_lw::flag::BOUNCE);
                    MotionModule::set_rate(fighter.module_accessor, 0.4);
                }
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        toonlink_frame
    );
}