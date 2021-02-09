use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_BAYONETTA,
    animation = "attack_s3_s3",
    animcmd = "game_attacks33")]
pub fn bayonetta_ftilt3(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2d51fcdb09, FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, true, false, false, 10, 3, 10, 5, true)
        }
        FT_MOTION_RATE(FSM=0.6)
        frame(Frame=22)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=81, KBG=100, FKB=0, BKB=70, Size=8.0, X=0.0, Y=16.0, Z=6.3, X2=0.0, Y2=8.0, Z2=5.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.0, Angle=81, KBG=100, FKB=0, BKB=70, Size=8.0, X=0.0, Y=26.0, Z=7.0, X2=0.0, Y2=8.0, Z2=5.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        FT_START_ADJUST_MOTION_FRAME_arg1(0.85)
        frame(Frame=25)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION)
            WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP)
            WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END)
        }
    });
}


pub fn install() {
    acmd::add_hooks!(
        bayonetta_ftilt3
    );
}