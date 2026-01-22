#![no_std]
#![no_main]

mod ssd1283a;

use defmt::*;
use defmt_rtt as _;
use embedded_hal::delay::DelayNs;
use panic_probe as _;

use embassy_rp::block::ImageDef;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::spi::{Config as SpiConfig, Spi};
use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;

use ssd1283a::Ssd1283a;

// CRITICAL for RP2350: Boot block signature
#[link_section = ".start_block"]
#[used]
static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

/// Simple blocking delay (no async)
struct BlockingDelay;

impl embedded_hal::delay::DelayNs for BlockingDelay {
    fn delay_ns(&mut self, ns: u32) {
        // ~150MHz clock, each cycle ~6.67ns
        // delay_ns / 6.67 ≈ cycles, but asm::delay has overhead
        // Rough approximation: 1000ns = 150 cycles
        // For 150MHz: 1us = 150 cycles. 1ns is too fine, assume 1us commands min usually
        // asm::delay takes clock cycles
        let cycles = (ns as u64 * 150 / 1000) as u32;
        if cycles > 0 {
            cortex_m::asm::delay(cycles);
        }
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("SSD1283A LCD init...");

    // Use ROSC to avoid crystal issues, safe default for Pico 2
    let mut config = embassy_rp::config::Config::default();
    config.clocks = embassy_rp::clocks::ClockConfig::rosc();
    let p = embassy_rp::init(config);

    // ==========================================
    // LCD Wiring Configuration (User Provided)
    // ==========================================
    // VCC: 3V3
    // LED: 3V3
    // GND: GND
    //
    // SCK: GP18
    // SDA: GP19 (MOSI)
    // A0 : GP20 (Data/Command)
    // RST: GP16
    // CS : GP17
    // ==========================================

    let clk = p.PIN_18; // SCK
    let mosi = p.PIN_19; // SDA / MOSI

    // Control pins
    // Initial states: CS high (inactive), RST high (inactive), DC high (data)
    let cs = Output::new(p.PIN_17, Level::High);
    let rst = Output::new(p.PIN_16, Level::High);
    let dc = Output::new(p.PIN_20, Level::High); // A0

    // Configure SPI - Mode 0 (CPOL=0, CPHA=0)
    let mut spi_config = SpiConfig::default();
    spi_config.frequency = 2_000_000; // Slow for stability

    // Create SPI 0 instance (blocking mode, TX only is fine)
    let spi = Spi::new_blocking_txonly(p.SPI0, clk, mosi, spi_config);

    // Create display driver instance
    let mut display = Ssd1283a::new(spi, dc, rst, Some(cs));

    // Initialize display
    let mut delay = BlockingDelay;

    info!("Resetting and initializing display...");
    match display.init(&mut delay) {
        Ok(_) => info!("Display initialized successfully!"),
        Err(_) => error!("Display init failed! Check wiring."),
    }

    // 1. Clear screen with Blue
    let _ = Rectangle::new(Point::new(0, 0), Size::new(130, 130))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
        .draw(&mut display);

    // 2. Draw a Red Rectangle
    let _ = Rectangle::new(Point::new(10, 10), Size::new(110, 50))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
        .draw(&mut display);

    // 3. Draw a Green Circle
    let _ = Circle::new(Point::new(50, 75), 40)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN))
        .draw(&mut display);

    // 4. Draw "ColorPicky" text
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
    let _ = Text::new("ColorPicky", Point::new(35, 35), text_style).draw(&mut display);

    info!("Drawing done. Blinking LED now.");

    // LED on Pico 2W/Pico 2 is usually GP25 or wireless chip, but standard Pico 2 has GP25
    // User didn't specify LED pin, assuming standard onboard or user led.
    // If Pico 2W, LED is connected to wireless chip - more complex.
    // Assuming standard Pico 2 uses GP25 for onboard LED.
    let mut led = Output::new(p.PIN_25, Level::Low);

    loop {
        led.toggle();
        delay.delay_ns(500_000_000); // 500ms
    }
}
