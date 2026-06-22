#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    block::ImageDef,
    gpio::{Level, Output},
};
use embassy_time::Timer;
use panic_probe as _;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"TMC2209 controller"),
    embassy_rp::binary_info::rp_program_description!(c"some stuff"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let _en = Output::new(p.PIN_2, Level::Low);
    let _dir = Output::new(p.PIN_3, Level::High);
    let mut step = Output::new(p.PIN_26, Level::High);

    loop {
        step.set_low();
        Timer::after_micros(1).await;
        step.set_high();
        Timer::after_micros(625 * 3).await;
    }
}
