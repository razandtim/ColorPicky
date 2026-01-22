//! Minimal LED blink test for RP2350/Pico 2

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

// Boot block for RP2350
#[link_section = ".start_block"]
#[used]
static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("BOOT");

    // Use default clock config (should work with XOSC)
    let p = embassy_rp::init(Default::default());

    info!("Init complete");

    // GP15 for external LED
    let mut led = Output::new(p.PIN_15, Level::Low);

    loop {
        info!("on");
        led.set_high();
        Timer::after_secs(1).await;

        info!("off");
        led.set_low();
        Timer::after_secs(1).await;
    }
}
