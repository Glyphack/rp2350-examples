#![no_std]
#![no_main]

use embassy_rp::{peripherals::USB, usb};

use defmt::*;
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

embassy_rp::bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<USB>;
});
#[embassy_executor::task]
async fn logger_task(usb: embassy_rp::Peri<'static, embassy_rp::peripherals::USB>) {
    let driver = embassy_rp::usb::Driver::new(usb, Irqs);

    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    spawner.spawn(logger_task(p.USB).unwrap());

    let _en = Output::new(p.PIN_2, Level::Low);
    let _dir = Output::new(p.PIN_3, Level::High);
    let mut step = Output::new(p.PIN_26, Level::High);

    loop {
        step.set_low();
        Timer::after_micros(1 * 10000).await;
        log::info!("hi step is high: {}", step.is_set_high());
        step.set_high();
        Timer::after_micros(625 * 1000).await;
        log::info!("hi step is high: {}", step.is_set_high());
    }
}
