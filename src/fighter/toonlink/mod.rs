mod acmd;
mod frame;
mod status;
mod agent_init;
pub mod fgc;
pub mod vl;
pub mod vars;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    agent_init::install();
}