mod acmd;
mod frame;
mod status;
pub mod helper;
pub mod fgc;
pub mod vars;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}