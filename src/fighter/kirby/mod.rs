mod acmd;
mod frame;
pub mod agent_init;

pub fn install() {
    acmd::install();
    frame::install();
}