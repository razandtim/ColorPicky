#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{
    block::ImageDef,
    gpio::{Level, Output},
    spi::{Config as SpiConfig, Spi},
};
use embassy_time::{Delay, Duration, Timer};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
    text::Text,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use st7735_lcd::{Orientation, ST7735};

use {defmt_rtt as _, panic_probe as _};

#[link_section = ".start_block"]
#[used]
static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("boot: LCD display test");

    let p = embassy_rp::init(Default::default());

    // SPI pins for display
    let clk = p.PIN_18; // SCK
    let mosi = p.PIN_19; // SDA/MOSI
    let miso = p.PIN_16; // Not used but needed for SPI (can be any unused pin)

    // Control pins
    let cs = Output::new(p.PIN_17, Level::High); // Chip Select
    let dc = Output::new(p.PIN_20, Level::Low); // Data/Command (A0)
    let rst = Output::new(p.PIN_21, Level::High); // Reset

    // Configure SPI
    let mut spi_config = SpiConfig::default();
    spi_config.frequency = 32_000_000; // 32 MHz

    let spi = Spi::new_blocking(p.SPI0, clk, mosi, miso, spi_config);
    let spi_device = ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    // Create display driver
    let mut display = ST7735::new(spi_device, dc, rst, true, false, 130, 130);

    // Initialize display
    display.init(&mut Delay).unwrap();
    display.set_orientation(&Orientation::Landscape).unwrap();
    info!("Display initialized!");

    // Clear screen to black
    display.clear(Rgb565::BLACK).unwrap();
    info!("Screen cleared to black");

    // Draw a red rectangle
    Rectangle::new(Point::new(10, 10), Size::new(50, 50))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
        .draw(&mut display)
        .unwrap();

    // Draw a green circle
    Circle::new(Point::new(70, 10), 50)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN))
        .draw(&mut display)
        .unwrap();

    // Draw text
    let style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    Text::new("Hello!", Point::new(30, 100), style)
        .draw(&mut display)
        .unwrap();

    info!("Drawing complete!");

    // Keep running
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
