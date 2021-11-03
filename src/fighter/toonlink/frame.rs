use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::{lua_bind::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_TOONLINK )]
fn toonlink_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // Toon Link can now move during his grounded Spin Attack.

        if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_hi")
        && WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME) >= WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_hold_frame")) as f32 {
            if MotionModule::frame(fighter.module_accessor) > 46.0 {
                macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            else if MotionModule::frame(fighter.module_accessor) > 6.0 {
                let stickx = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
                SPIN_SPEED[entry_id(fighter.module_accessor)] += 0.1 * stickx;
                if IS_FUNNY[entry_id(fighter.module_accessor)]
                && SPIN_SPEED[entry_id(fighter.module_accessor)] > 3.0 {
                    SPIN_SPEED[entry_id(fighter.module_accessor)] = 3.0;
                }
                else if SPIN_SPEED[entry_id(fighter.module_accessor)] > 2.0 {
                    SPIN_SPEED[entry_id(fighter.module_accessor)] = 2.0;
                }
                let speed = SPIN_SPEED[entry_id(fighter.module_accessor)];
                macros::SET_SPEED_EX(fighter, speed, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            else {
                if IS_FUNNY[entry_id(fighter.module_accessor)] {
                    SPIN_SPEED[entry_id(fighter.module_accessor)] = 3.0;
                }
                else {
                    SPIN_SPEED[entry_id(fighter.module_accessor)] = 2.0;
                }
            }
        }

        // Down Air Bounce

        if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("attack_air_lw"){
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                BOUNCE[entry_id(fighter.module_accessor)] = true;
            }
            if BOUNCE[entry_id(fighter.module_accessor)] {
                macros::SET_SPEED_EX(fighter, 0.0, 0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                KineticModule::suspend_energy_all(fighter.module_accessor);
                if MotionModule::frame(fighter.module_accessor) > HIT_FRAME[entry_id(fighter.module_accessor)] + 1.0
                && MotionModule::frame(fighter.module_accessor) < 65.0 {
                    MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 65.0, true, false, false);
                }
                else if MotionModule::frame(fighter.module_accessor) > 66.0 {
                    KineticModule::resume_energy_all(fighter.module_accessor);
                    BOUNCE[entry_id(fighter.module_accessor)] = false;
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