use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::RgbColor;

pub struct NamedColor {
    pub name: &'static str,
    pub color: Rgb888,
}

pub const COLORS: &[NamedColor] = &[
    NamedColor {
        name: "Red",
        color: Rgb888::new(255, 0, 0),
    },
    NamedColor {
        name: "Green",
        color: Rgb888::new(0, 255, 0),
    },
    NamedColor {
        name: "Blue",
        color: Rgb888::new(0, 0, 255),
    },
    NamedColor {
        name: "Yellow",
        color: Rgb888::new(255, 255, 0),
    },
    NamedColor {
        name: "Cyan",
        color: Rgb888::new(0, 255, 255),
    },
    NamedColor {
        name: "Magenta",
        color: Rgb888::new(255, 0, 255),
    },
    NamedColor {
        name: "White",
        color: Rgb888::new(255, 255, 255),
    },
    NamedColor {
        name: "Black",
        color: Rgb888::new(0, 0, 0),
    },
    NamedColor {
        name: "Gray",
        color: Rgb888::new(128, 128, 128),
    },
    NamedColor {
        name: "Orange",
        color: Rgb888::new(255, 165, 0),
    },
    NamedColor {
        name: "Purple",
        color: Rgb888::new(128, 0, 128),
    },
    NamedColor {
        name: "Brown",
        color: Rgb888::new(165, 42, 42),
    },
    NamedColor {
        name: "Pink",
        color: Rgb888::new(255, 192, 203),
    },
    NamedColor {
        name: "Teal",
        color: Rgb888::new(0, 128, 128),
    },
    NamedColor {
        name: "Violet",
        color: Rgb888::new(238, 130, 238),
    },
];

pub fn match_color(r: u8, g: u8, b: u8) -> &'static str {
    let mut min_dist = u32::MAX;
    let mut best_match = "Unknown";

    for c in COLORS {
        let dr = (c.color.r() as i32 - r as i32).pow(2);
        let dg = (c.color.g() as i32 - g as i32).pow(2);
        let db = (c.color.b() as i32 - b as i32).pow(2);
        let dist = (dr + dg + db) as u32;

        if dist < min_dist {
            min_dist = dist;
            best_match = c.name;
        }
    }
    best_match
}
