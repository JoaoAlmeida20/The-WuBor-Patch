mod acmd;
mod status;
mod frame;
mod vtable_hook;
pub mod vl;

pub fn install() {
    acmd::install();
    status::install();
    frame::install();
    vtable_hook::install();
}