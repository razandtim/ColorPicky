# ColorPicky - Alpha Version
**The Rust Real-Life Color Picker**

A real-time color picking tool powered by **Rust**, **Embassy**, and the **RP2350** (Raspberry Pi Pico 2W). It reads colors from real-world objects using a TCS34725 sensor, identifies them by name and hex code, and keeps a history of your findings.

![Project Status](https://img.shields.io/badge/status-alpha-orange)
![Rust](https://img.shields.io/badge/built_with-Rust-red)

## 🌟 Features

*   **Real-time Color Sensing**: Reads RGB and Clear channel data from a TCS34725 sensor.
*   **Color Naming**: Matches the sampled color to the nearest known color name (e.g., "Red", "Sky Blue", "Forest Green").
*   **Hex & RGB Display**: Shows the precise Hex code and RGB values on screen.
*   **History Mode**: Stores the last **10 saved colors** in a scrollable history list.
*   **Interactive UI**:
    *   **Main Screen**: Live color preview box, name, and hex.
    *   **History Screen**: List of previously saved colors.
*   **Controls**:
    *   **Hold Button**: Real-time color sampling mode - watch the color update live as you move the sensor!
    *   **Release Button**: Saves the picked color to history
    *   **Quick Tap**: Instantly save current color to history
    *   **Double Tap**: Toggle between Main and History screens
    *   **Long Press (in History)**: Clear all saved colors

## 🛠 Hardware Setup

### Components
*   **MCU**: Raspberry Pi Pico 2W (RP2350)
*   **Display**: SSD1283A 1.6" 130x130 SPI LCD (https://www.lcdwiki.com/1.6inch_SPI_Module_SSD1283A_SKU:MSP1601)
*   **Sensor**: TCS34725 RGB Color Sensor (I2C) (https://www.dfrobot.com/product-1546.html)
*   **Input**: Push button

### Pinout Configuration

| Component | Pin Function | Pico 2 Pin |
|-----------|--------------|------------|
| **Display** | | |
| SSD1283A | VCC / LED | 3V3 |
| | SCK | **GP18** |
| | SDA (MOSI)| **GP19** |
| | A0 (DC) | **GP20** |
| | RST | **GP16** |
| | CS | **GP17** |
| **Sensor** | | |
| TCS34725 | SDA | **GP6** |
| | SCL | **GP7** |
| | VCC | 3V3 |
| **Input** | | |
| Button | Signal | **GP15** (to GND) |

## 📦 Dependencies & Tech Stack

This project uses the bleeding-edge **Embassy** async ecosystem.

*   `embassy-rp` (RP2350 support)
*   `embassy-main` (Async executor)
*   `embedded-graphics` (UI rendering)
*   `defmt` (High-performance logging)

> **Note**: Due to rapid development in the Embassy ecosystem, this project currently points to `git` dependencies to ensure compatibility.

## 🚀 Building & Running

1.  **Install Prerequisites**:
    *   Rust (stable)
    *   `probe-rs` (`cargo install probe-rs --features cli`)

2.  **Run**:
    ```bash
    cargo run --release
    ```

## 📸 Photos

![20260123_030441 (2)](https://github.com/user-attachments/assets/650df984-5e4b-48c1-8758-c4408921fd63)
![IMG-20260123-WA0030](https://github.com/user-attachments/assets/2e46d976-adc2-4b95-980e-dcdda3059260)
![IMG-20260123-WA0028 (2)](https://github.com/user-attachments/assets/a464ceb3-0fd0-4ca4-a8a1-2ab2da3e47b5)
![20260123_031750](https://github.com/user-attachments/assets/2db3e5dd-3259-414a-86d0-fac11ea59f1a)
![20260123_031733](https://github.com/user-attachments/assets/94b2205e-c4bd-4459-9572-4d4a545c8bbe)


## 📜 License

MIT

---

**Made by:**  
Razvan Andrei Timofte  
Group 1241EC - IoT
