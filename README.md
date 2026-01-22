# ColorPicky - SSD1283A Display on Raspberry Pi Pico 2

A Rust embedded project driving an SSD1283A 1.6" 130x130 LCD display using a Raspberry Pi Pico 2 (RP2350) with debugprobe.

## Hardware Setup

| LCD Pin | Pico 2 Pin |
|---------|------------|
| VCC | 3V3 (Pin 36) |
| LED | 3V3 (Pin 36) |
| SCK | GP18 |
| SDA (MOSI) | GP19 |
| A0 (DC) | GP20 |
| RST | GP16 |
| CS | GP17 |
| GND | GND (Pin 23) |

## Display Module

- **Module**: 1.6" SPI 130x130 LCD
- **Controller**: SSD1283A
- **Part Numbers**: 7290088N05, MSP1601
- **Level Shifter**: LVC245A (onboard)
- **Reference**: [LCDWiki MSP1601](https://www.lcdwiki.com/1.6inch_SPI_Module_SSD1283A_SKU:MSP1601)

## Features

- ✅ Custom SSD1283A Rust driver with `embedded-graphics` support
- ✅ Full 130x130 pixel RGB565 display
- ✅ Filled rectangles and circles
- ✅ Text rendering via `embedded-graphics` mono_font
- ✅ SPI communication at 2MHz (Mode 0)

## Dependencies

```toml
[dependencies]
embassy-rp = { version = "0.4", features = ["rp235xa", "critical-section-impl"] }
embedded-hal = "1.0"
embedded-graphics = "0.8"
cortex-m = "0.7"
cortex-m-rt = "0.7"
defmt = "0.3"
defmt-rtt = "0.4"
panic-probe = { version = "0.3", features = ["print-defmt"] }
```

## Building & Flashing

```bash
# Build release
cargo build --release

# Flash and run (with debugprobe)
cargo run --release
```

## Key Technical Details

### SSD1283A Driver Notes

1. **RAM Address Offset**: The SSD1283A requires a +2 pixel offset for window/RAM addresses
2. **Pixel Format**: Little-endian RGB565 (low byte first)
3. **Entry Mode**: 0x6030 for RGB color order
4. **Init Sequence**: Based on ZinggJM/SSD1283A Arduino reference

### Known Working Configuration

- SPI Mode 0 (CPOL=0, CPHA=0)
- SPI Frequency: 2MHz
- DC pin toggles between command (low) and data (high)
- CS stays low during complete register write (command + data)

## Screenshots

![ColorPicky Display Working](screenshot.jpg)

## License

MIT