use {
    smash::{
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::vars::*
};

pub struct GenericModule {
    _vtable: *const u64,
    owner: *mut BattleObjectModuleAccessor,
    // ...
}

#[skyline::hook(offset = 0x971230)]
pub unsafe extern "C" fn dolly_check_super_special(work: &mut GenericModule, _damage: &mut GenericModule) -> u64 {
    let module_accessor = work.owner;
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    if smashball::is_training_mode() {
        return 1;
    }
    let object_id = (*module_accessor).battle_object_id;
    let object = sv_system::battle_object(object_id as u64);
    let go_meter = VarModule::get_float(object, dolly::instance::float::GO_METER);
    // println!("go_meter: {}", go_meter);
    if go_meter >= 100.0 {
        return 1;
    }
    0
}

#[skyline::hook(offset = 0x970fd0)]
pub unsafe extern "C" fn dolly_check_super_special_pre(module_accessor: *mut BattleObjectModuleAccessor, param_2: u8) {
    original!()(module_accessor, param_2)
}

pub fn install() {
    skyline::install_hooks!(
        dolly_check_super_special,
        dolly_check_super_special_pre
    );
}