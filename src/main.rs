#![no_std]
#![no_main]

mod colors;
mod input;
mod ssd1283a;
mod tcs34725;

use defmt::{error, info, Debug2Format};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::i2c::{Config as I2cConfig, I2c};
use embassy_rp::spi::{Config as SpiConfig, Spi};
use embassy_time::{Delay, Timer};
use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;
use panic_probe as _;

use colors::{match_color, NamedColor};
use input::{ButtonEvent, ButtonInput};
use ssd1283a::Ssd1283a;
use tcs34725::{Rgbc, Tcs34725};

use embassy_rp::block::ImageDef;
#[link_section = ".start_block"]
#[used]
static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[derive(PartialEq, Clone, Copy)]
enum AppMode {
    Measuring,
    History,
}

struct AppState {
    mode: AppMode,
    history: [Option<NamedColor>; 10],
    current_reading: Option<NamedColor>,
    current_rgbc: Rgbc,
}

impl AppState {
    fn new() -> Self {
        Self {
            mode: AppMode::Measuring,
            history: [None; 10], // Default value
            current_reading: None,
            current_rgbc: Rgbc::default(),
        }
    }

    fn push_history(&mut self) {
        if let Some(color) = &self.current_reading {
            // Check if it's the same as the last saved one to avoid dupes?
            // User requested "Save current color", maybe dupes are allowed or shifts.
            // "Save current color to next slot (1-10)" implies a ring buffer or list.

            // For now, ring buffer logic or fill logic.
            // Let's do a shift down? Or just ring buffer.
            // User UI shows 1-10.

            // Shift everything down to make room at top? Or append?
            // "history list" usually newest on top.
            for i in (1..10).rev() {
                self.history[i] = self.history[i - 1].clone();
            }
            // Add new at 0

            // Wait, NamedColor has a &'static str name, but Rgb888 color.
            // We need to clone the struct (needs Clone derive in colors.rs? No, manual clone or copy fields)
            let new_c = NamedColor {
                name: color.name,
                color: color.color,
            };
            self.history[0] = Some(new_c);

            info!("Saved color: {}", color.name);
        }
    }

    fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            AppMode::Measuring => AppMode::History,
            AppMode::History => AppMode::Measuring,
        };
        info!("Mode switched");
    }

    fn clear_history(&mut self) {
        self.history = [None; 10];
        info!("History cleared");
    }
}

// Implement Clone/Copy for NamedColor manually since we didn't derive it
impl Clone for NamedColor {
    fn clone(&self) -> Self {
        NamedColor {
            name: self.name,
            color: self.color,
        }
    }
}
impl Copy for NamedColor {}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("ColorPicky Phase 2 Starting...");

    let p = embassy_rp::init(Default::default());

    // ==================
    // LCD Setup
    // ==================
    let clk = p.PIN_18;
    let mosi = p.PIN_19;
    let cs = Output::new(p.PIN_17, Level::High);
    let rst = Output::new(p.PIN_16, Level::High);
    let dc = Output::new(p.PIN_20, Level::High);

    let mut spi_config = SpiConfig::default();
    spi_config.frequency = 4_000_000;
    let spi = Spi::new_blocking_txonly(p.SPI0, clk, mosi, spi_config);

    let mut display = Ssd1283a::new(spi, dc, rst, Some(cs));
    display.init(&mut Delay).expect("Display init failed");

    // Clear screen black
    display.fill_screen(Rgb565::BLACK).unwrap();

    // ==================
    // Sensor Setup (I2C1)
    // ==================
    // GP6 = I2C1 SDA, GP7 = I2C1 SCL
    let sda = p.PIN_6;
    let scl = p.PIN_7;
    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = 400_000;

    let i2c = I2c::new_blocking(p.I2C1, scl, sda, i2c_config);
    let mut sensor = Tcs34725::new(i2c);

    if let Err(e) = sensor.init() {
        error!("Sensor init failed: {:?}", Debug2Format(&e));
    } else {
        info!("Sensor initialized");
    }
    // Enable PON/AEN
    let _ = sensor.enable();

    // ==================
    // Button Setup
    // ==================
    let btn_pin = Input::new(p.PIN_15, Pull::Up);
    let mut button = ButtonInput::new(btn_pin);

    // ==================
    // State & Loop
    // ==================
    let mut state = AppState::new();
    // let mut last_draw_mode = AppMode::Measuring; // Force draw first time (unused)
    let mut needs_redraw = true;

    // Font styles
    let style_title = MonoTextStyle::new(&FONT_6X10, Rgb565::YELLOW);
    let style_text = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
    let style_small = MonoTextStyle::new(&FONT_6X10, Rgb565::CSS_GRAY);

    loop {
        // 1. Poll Button
        if let Some(event) = button.poll().await {
            match event {
                ButtonEvent::SingleClick => {
                    if state.mode == AppMode::Measuring {
                        state.push_history(); // Capture current
                                              // Visual feedback? maybe
                    }
                }
                ButtonEvent::DoubleClick => {
                    state.toggle_mode();
                    needs_redraw = true; // Full screen redraw
                }
                ButtonEvent::LongPress => {
                    if state.mode == AppMode::History {
                        state.clear_history();
                        needs_redraw = true;
                    }
                }
            }
        }

        // 2. Sensor Read (Only in Measuring mode or periodically)
        if state.mode == AppMode::Measuring {
            // Read every 100ms?
            // We can check if time passed, but for now lets just read.
            // sensor.read_all() takes time (I2C blocking), so it acts as delay.
            match sensor.read_all() {
                Ok(rgbc) => {
                    state.current_rgbc = rgbc;
                    // Simple white balance / scaling
                    // TCS34725 clear channel C is brightness.
                    // If C is 0, everything 0.
                    // Normalize: r = r/c * 255

                    if rgbc.c > 0 {
                        let r8 = (rgbc.r as u32 * 255 / rgbc.c as u32) as u8;
                        let g8 = (rgbc.g as u32 * 255 / rgbc.c as u32) as u8;
                        let b8 = (rgbc.b as u32 * 255 / rgbc.c as u32) as u8;

                        let name = match_color(r8, g8, b8);
                        let matched_color = NamedColor {
                            name,
                            color: embedded_graphics::pixelcolor::Rgb888::new(r8, g8, b8),
                        };

                        // Update state if changed significantly?
                        // For now just update always, UI drawing will handle flicker check
                        state.current_reading = Some(matched_color);
                    }
                }
                Err(_) => {} // Ignore i2c fail
            }
        }

        // 3. Draw UI
        if needs_redraw || state.mode == AppMode::Measuring {
            if needs_redraw {
                display.fill_screen(Rgb565::BLACK).unwrap();
                needs_redraw = false;
            }

            match state.mode {
                AppMode::Measuring => {
                    // Draw Main Screen
                    draw_main_screen(&mut display, &state, style_title, style_text, style_small);
                }
                AppMode::History => {
                    // Draw History Screen (static unless updated)
                    // If we just entered handled by needs_redraw
                    draw_history_screen(&mut display, &state, style_title, style_text);
                }
            }
        }

        // Small delay to prevent busy loop if poll is fast
        Timer::after_millis(50).await;
    }
}

fn draw_main_screen<D>(
    display: &mut D,
    state: &AppState,
    style_title: MonoTextStyle<Rgb565>,
    style_text: MonoTextStyle<Rgb565>,
    _style_small: MonoTextStyle<Rgb565>,
) where
    D: DrawTarget<Color = Rgb565>,
{
    // Title
    Text::new("ColorPicky", Point::new(30, 10), style_title)
        .draw(display)
        .ok();

    // Separator
    // Rectangle::new(Point::new(0, 15), Size::new(130, 1)).into_styled(PrimitiveStyle::with_fill(Rgb565::CSS_GRAY)).draw(display).ok();

    if let Some(c) = &state.current_reading {
        // Color Box
        let box_color = Rgb565::from(c.color);
        Rectangle::new(Point::new(25, 25), Size::new(80, 80))
            .into_styled(PrimitiveStyle::with_fill(box_color))
            .draw(display)
            .ok();

        // Text inside box? user mockup shows "80x80px Color".
        // We'll skip text inside for cleanliness or render if contrast ok.

        // Name
        Text::new(c.name, Point::new(10, 120), style_text)
            .draw(display)
            .ok();

        // Hex / RGB (Maybe below? 130px height is tight)
        // Adjust layout:
        // Title: y=10
        // Box: y=20, h=80 -> y_end=100
        // Name: y=115
        // RGB: y=125

        // Hex / RGB
        use core::fmt::Write;
        let mut buf = heapless::String::<32>::new();
        let _ = buf.write_fmt(format_args!(
            "#{0:02X}{1:02X}{2:02X}",
            c.color.r(),
            c.color.g(),
            c.color.b()
        ));
        Text::new(&buf, Point::new(10, 125), style_text)
            .draw(display)
            .ok();
    } else {
        Text::new("Place on color...", Point::new(10, 60), style_text)
            .draw(display)
            .ok();
    }
}

fn draw_history_screen<D>(
    display: &mut D,
    state: &AppState,
    style_title: MonoTextStyle<Rgb565>,
    style_text: MonoTextStyle<Rgb565>,
) where
    D: DrawTarget<Color = Rgb565>,
{
    Text::new("History", Point::new(40, 10), style_title)
        .draw(display)
        .ok();

    let mut y = 25;
    for (i, item) in state.history.iter().enumerate() {
        if let Some(c) = item {
            // 1 #HEX Name
            use core::fmt::Write;
            let mut buf = heapless::String::<32>::new();
            let _ = buf.write_fmt(format_args!("{}. {} ", i + 1, c.name));
            // Hex is too long for line? "1. #FFFFFF Name"

            Text::new(&buf, Point::new(5, y), style_text)
                .draw(display)
                .ok();

            // Small color indicator?
            Rectangle::new(Point::new(110, y - 8), Size::new(10, 10))
                .into_styled(PrimitiveStyle::with_fill(Rgb565::from(c.color)))
                .draw(display)
                .ok();

            y += 12;
            if y > 125 {
                break;
            }
        }
    }
}
