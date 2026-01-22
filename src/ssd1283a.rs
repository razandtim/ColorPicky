// src/ssd1283a.rs
// SSD1283A driver based on ZinggJM/SSD1283A reference
#![allow(dead_code)]

use embedded_graphics::{pixelcolor::Rgb565, prelude::*, primitives::Rectangle};

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiBus;

pub const WIDTH: u16 = 130;
pub const HEIGHT: u16 = 130;

#[derive(Debug)]
pub enum Error<SpiE, PinE> {
    Spi(SpiE),
    Pin(PinE),
}

pub struct Ssd1283a<SPI, DC, RST, CS> {
    spi: SPI,
    dc: DC,
    rst: RST,
    cs: Option<CS>,
    width: u16,
    height: u16,
}

impl<SPI, DC, RST, CS> Ssd1283a<SPI, DC, RST, CS>
where
    SPI: SpiBus<u8>,
    DC: OutputPin,
    RST: OutputPin,
    CS: OutputPin,
{
    pub fn new(spi: SPI, dc: DC, rst: RST, cs: Option<CS>) -> Self {
        Self {
            spi,
            dc,
            rst,
            cs,
            width: WIDTH,
            height: HEIGHT,
        }
    }

    fn start_transaction(&mut self) {
        if let Some(cs) = self.cs.as_mut() {
            let _ = cs.set_low();
        }
    }

    fn end_transaction(&mut self) {
        if let Some(cs) = self.cs.as_mut() {
            let _ = cs.set_high();
        }
    }

    // Write command + 16-bit data as one transaction
    fn write_reg(&mut self, cmd: u8, data: u16) -> Result<(), Error<SPI::Error, CS::Error>> {
        self.start_transaction();
        let _ = self.dc.set_low();
        self.spi.write(&[cmd]).map_err(Error::Spi)?;
        let _ = self.dc.set_high();
        self.spi
            .write(&[(data >> 8) as u8, data as u8])
            .map_err(Error::Spi)?;
        self.end_transaction();
        Ok(())
    }

    pub fn reset<D: DelayNs>(&mut self, delay: &mut D) {
        self.end_transaction();
        let _ = self.rst.set_high();
        delay.delay_ms(5);
        let _ = self.rst.set_low();
        delay.delay_ms(2);
        let _ = self.rst.set_high();
        delay.delay_ms(200);
    }

    // Init sequence from ZinggJM/SSD1283A reference
    pub fn init<D: DelayNs>(&mut self, delay: &mut D) -> Result<(), Error<SPI::Error, CS::Error>> {
        self.reset(delay);

        self.write_reg(0x10, 0x2F8E)?;
        self.write_reg(0x11, 0x000C)?;
        self.write_reg(0x07, 0x0021)?;
        self.write_reg(0x28, 0x0006)?;
        self.write_reg(0x28, 0x0005)?;
        self.write_reg(0x27, 0x057F)?;
        self.write_reg(0x29, 0x89A1)?;
        self.write_reg(0x00, 0x0001)?;
        delay.delay_ms(100);

        self.write_reg(0x29, 0x80B0)?;
        delay.delay_ms(30);

        self.write_reg(0x29, 0xFFFE)?;
        self.write_reg(0x07, 0x0223)?;
        delay.delay_ms(30);

        self.write_reg(0x07, 0x0233)?;
        self.write_reg(0x01, 0x2183)?;
        self.write_reg(0x03, 0x6030)?; // Entry mode: RGB (not BGR)
        self.write_reg(0x2F, 0xFFFF)?;
        self.write_reg(0x2C, 0x8000)?;
        self.write_reg(0x27, 0x0570)?;
        self.write_reg(0x02, 0x0300)?;
        self.write_reg(0x0B, 0x580C)?;
        self.write_reg(0x12, 0x0609)?;
        self.write_reg(0x13, 0x3100)?;

        delay.delay_ms(50);
        Ok(())
    }

    // Set window address with +2 offset (rotation 0) and start GRAM write
    fn set_window_and_write_start(
        &mut self,
        x1: u16,
        y1: u16,
        x2: u16,
        y2: u16,
    ) -> Result<(), Error<SPI::Error, CS::Error>> {
        // Reg 0x44: horizontal RAM (x2+2, x1+2)
        self.write_reg(0x44, (((x2 + 2) as u16) << 8) | ((x1 + 2) as u16))?;

        // Reg 0x45: vertical RAM (y2+2, y1+2)
        self.write_reg(0x45, (((y2 + 2) as u16) << 8) | ((y1 + 2) as u16))?;

        // Reg 0x21: RAM counter (y1+2, x1+2)
        self.write_reg(0x21, (((y1 + 2) as u16) << 8) | ((x1 + 2) as u16))?;

        // Start GRAM write - CS low, send 0x22 command, then DC high for data
        self.start_transaction();
        let _ = self.dc.set_low();
        self.spi.write(&[0x22]).map_err(Error::Spi)?;
        let _ = self.dc.set_high();
        // Leave transaction open for pixel data
        Ok(())
    }

    pub fn fill_screen(&mut self, color: Rgb565) -> Result<(), Error<SPI::Error, CS::Error>> {
        self.set_window_and_write_start(0, 0, self.width - 1, self.height - 1)?;

        // Little-endian pixel data (matches what worked before)
        let raw = color.into_storage();
        let px = [raw as u8, (raw >> 8) as u8];
        let total = (self.width as u32) * (self.height as u32);

        let mut buf = [0u8; 512];
        for chunk in buf.chunks_exact_mut(2) {
            chunk[0] = px[0];
            chunk[1] = px[1];
        }

        let mut remaining = total;
        while remaining > 0 {
            let pixels_this = core::cmp::min(remaining, (buf.len() / 2) as u32);
            let bytes_this = (pixels_this as usize) * 2;
            self.spi.write(&buf[..bytes_this]).map_err(Error::Spi)?;
            remaining -= pixels_this;
        }

        self.end_transaction();
        Ok(())
    }

    pub fn fill_rect(
        &mut self,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        color: Rgb565,
    ) -> Result<(), Error<SPI::Error, CS::Error>> {
        if w == 0 || h == 0 {
            return Ok(());
        }
        let x2 = x + w - 1;
        let y2 = y + h - 1;

        self.set_window_and_write_start(x, y, x2, y2)?;

        let raw = color.into_storage();
        let px = [raw as u8, (raw >> 8) as u8];
        let total = (w as u32) * (h as u32);

        let mut buf = [0u8; 512];
        for chunk in buf.chunks_exact_mut(2) {
            chunk[0] = px[0];
            chunk[1] = px[1];
        }

        let mut remaining = total;
        while remaining > 0 {
            let pixels_this = core::cmp::min(remaining, (buf.len() / 2) as u32);
            let bytes_this = (pixels_this as usize) * 2;
            self.spi.write(&buf[..bytes_this]).map_err(Error::Spi)?;
            remaining -= pixels_this;
        }

        self.end_transaction();
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        x: u16,
        y: u16,
        color: Rgb565,
    ) -> Result<(), Error<SPI::Error, CS::Error>> {
        self.set_window_and_write_start(x, y, x, y)?;
        let raw = color.into_storage();
        self.spi
            .write(&[raw as u8, (raw >> 8) as u8])
            .map_err(Error::Spi)?;
        self.end_transaction();
        Ok(())
    }
}

impl<SPI, DC, RST, CS> OriginDimensions for Ssd1283a<SPI, DC, RST, CS> {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}

impl<SPI, DC, RST, CS> DrawTarget for Ssd1283a<SPI, DC, RST, CS>
where
    SPI: SpiBus<u8>,
    DC: OutputPin,
    RST: OutputPin,
    CS: OutputPin,
{
    type Color = Rgb565;
    type Error = Error<SPI::Error, CS::Error>;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(p, c) in pixels {
            if p.x < 0 || p.y < 0 {
                continue;
            }
            let x = p.x as u16;
            let y = p.y as u16;
            if x >= self.width || y >= self.height {
                continue;
            }
            self.draw_pixel(x, y, c)?;
        }
        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let display_size = Size::new(self.width as u32, self.height as u32);
        let area = area.intersection(&Rectangle::new(Point::zero(), display_size));
        if area.size.width == 0 || area.size.height == 0 {
            return Ok(());
        }
        self.fill_rect(
            area.top_left.x as u16,
            area.top_left.y as u16,
            area.size.width as u16,
            area.size.height as u16,
            color,
        )
    }
}
