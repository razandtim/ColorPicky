use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::RgbColor;

pub struct NamedColor {
    pub name: &'static str,
    pub color: Rgb888,
}

// Comprehensive color palette for accurate color matching
pub const COLORS: &[NamedColor] = &[
    // === Reds ===
    NamedColor {
        name: "Red",
        color: Rgb888::new(255, 0, 0),
    },
    NamedColor {
        name: "Dark Red",
        color: Rgb888::new(139, 0, 0),
    },
    NamedColor {
        name: "Crimson",
        color: Rgb888::new(220, 20, 60),
    },
    NamedColor {
        name: "Maroon",
        color: Rgb888::new(128, 0, 0),
    },
    NamedColor {
        name: "Salmon",
        color: Rgb888::new(250, 128, 114),
    },
    NamedColor {
        name: "Coral",
        color: Rgb888::new(255, 127, 80),
    },
    NamedColor {
        name: "Tomato",
        color: Rgb888::new(255, 99, 71),
    },
    // === Oranges ===
    NamedColor {
        name: "Orange",
        color: Rgb888::new(255, 165, 0),
    },
    NamedColor {
        name: "Dark Orange",
        color: Rgb888::new(255, 140, 0),
    },
    NamedColor {
        name: "Orange Red",
        color: Rgb888::new(255, 69, 0),
    },
    NamedColor {
        name: "Peach",
        color: Rgb888::new(255, 218, 185),
    },
    // === Yellows ===
    NamedColor {
        name: "Yellow",
        color: Rgb888::new(255, 255, 0),
    },
    NamedColor {
        name: "Gold",
        color: Rgb888::new(255, 215, 0),
    },
    NamedColor {
        name: "Lemon",
        color: Rgb888::new(255, 247, 0),
    },
    NamedColor {
        name: "Khaki",
        color: Rgb888::new(240, 230, 140),
    },
    NamedColor {
        name: "Beige",
        color: Rgb888::new(245, 245, 220),
    },
    // === Greens ===
    NamedColor {
        name: "Green",
        color: Rgb888::new(0, 255, 0),
    },
    NamedColor {
        name: "Lime",
        color: Rgb888::new(50, 205, 50),
    },
    NamedColor {
        name: "Dark Green",
        color: Rgb888::new(0, 100, 0),
    },
    NamedColor {
        name: "Forest Green",
        color: Rgb888::new(34, 139, 34),
    },
    NamedColor {
        name: "Olive",
        color: Rgb888::new(128, 128, 0),
    },
    NamedColor {
        name: "Sea Green",
        color: Rgb888::new(46, 139, 87),
    },
    NamedColor {
        name: "Spring Green",
        color: Rgb888::new(0, 255, 127),
    },
    NamedColor {
        name: "Mint",
        color: Rgb888::new(152, 255, 152),
    },
    // === Cyans / Turquoise ===
    NamedColor {
        name: "Cyan",
        color: Rgb888::new(0, 255, 255),
    },
    NamedColor {
        name: "Aqua",
        color: Rgb888::new(0, 255, 255),
    },
    NamedColor {
        name: "Turquoise",
        color: Rgb888::new(64, 224, 208),
    },
    NamedColor {
        name: "Teal",
        color: Rgb888::new(0, 128, 128),
    },
    NamedColor {
        name: "Dark Cyan",
        color: Rgb888::new(0, 139, 139),
    },
    NamedColor {
        name: "Aquamarine",
        color: Rgb888::new(127, 255, 212),
    },
    NamedColor {
        name: "Light Cyan",
        color: Rgb888::new(224, 255, 255),
    },
    // === Blues ===
    NamedColor {
        name: "Blue",
        color: Rgb888::new(0, 0, 255),
    },
    NamedColor {
        name: "Navy",
        color: Rgb888::new(0, 0, 128),
    },
    NamedColor {
        name: "Royal Blue",
        color: Rgb888::new(65, 105, 225),
    },
    NamedColor {
        name: "Sky Blue",
        color: Rgb888::new(135, 206, 235),
    },
    NamedColor {
        name: "Light Blue",
        color: Rgb888::new(173, 216, 230),
    },
    NamedColor {
        name: "Steel Blue",
        color: Rgb888::new(70, 130, 180),
    },
    NamedColor {
        name: "Dodger Blue",
        color: Rgb888::new(30, 144, 255),
    },
    NamedColor {
        name: "Deep Sky Blue",
        color: Rgb888::new(0, 191, 255),
    },
    NamedColor {
        name: "Midnight Blue",
        color: Rgb888::new(25, 25, 112),
    },
    // === Purples / Violets ===
    NamedColor {
        name: "Purple",
        color: Rgb888::new(128, 0, 128),
    },
    NamedColor {
        name: "Violet",
        color: Rgb888::new(238, 130, 238),
    },
    NamedColor {
        name: "Indigo",
        color: Rgb888::new(75, 0, 130),
    },
    NamedColor {
        name: "Lavender",
        color: Rgb888::new(230, 230, 250),
    },
    NamedColor {
        name: "Plum",
        color: Rgb888::new(221, 160, 221),
    },
    NamedColor {
        name: "Orchid",
        color: Rgb888::new(218, 112, 214),
    },
    NamedColor {
        name: "Magenta",
        color: Rgb888::new(255, 0, 255),
    },
    NamedColor {
        name: "Fuchsia",
        color: Rgb888::new(255, 0, 255),
    },
    NamedColor {
        name: "Dark Violet",
        color: Rgb888::new(148, 0, 211),
    },
    NamedColor {
        name: "Blue Violet",
        color: Rgb888::new(138, 43, 226),
    },
    NamedColor {
        name: "Medium Purple",
        color: Rgb888::new(147, 112, 219),
    },
    // === Pinks ===
    NamedColor {
        name: "Pink",
        color: Rgb888::new(255, 192, 203),
    },
    NamedColor {
        name: "Hot Pink",
        color: Rgb888::new(255, 105, 180),
    },
    NamedColor {
        name: "Deep Pink",
        color: Rgb888::new(255, 20, 147),
    },
    NamedColor {
        name: "Light Pink",
        color: Rgb888::new(255, 182, 193),
    },
    NamedColor {
        name: "Rose",
        color: Rgb888::new(255, 0, 127),
    },
    // === Browns / Tans ===
    NamedColor {
        name: "Brown",
        color: Rgb888::new(139, 69, 19),
    },
    NamedColor {
        name: "Chocolate",
        color: Rgb888::new(210, 105, 30),
    },
    NamedColor {
        name: "Tan",
        color: Rgb888::new(210, 180, 140),
    },
    NamedColor {
        name: "Sienna",
        color: Rgb888::new(160, 82, 45),
    },
    NamedColor {
        name: "Sandy Brown",
        color: Rgb888::new(244, 164, 96),
    },
    NamedColor {
        name: "Peru",
        color: Rgb888::new(205, 133, 63),
    },
    NamedColor {
        name: "Saddle Brown",
        color: Rgb888::new(139, 69, 19),
    },
    // === Whites / Grays / Blacks ===
    NamedColor {
        name: "White",
        color: Rgb888::new(255, 255, 255),
    },
    NamedColor {
        name: "Snow",
        color: Rgb888::new(255, 250, 250),
    },
    NamedColor {
        name: "Ivory",
        color: Rgb888::new(255, 255, 240),
    },
    NamedColor {
        name: "Light Gray",
        color: Rgb888::new(211, 211, 211),
    },
    NamedColor {
        name: "Silver",
        color: Rgb888::new(192, 192, 192),
    },
    NamedColor {
        name: "Gray",
        color: Rgb888::new(128, 128, 128),
    },
    NamedColor {
        name: "Dark Gray",
        color: Rgb888::new(64, 64, 64),
    },
    NamedColor {
        name: "Charcoal",
        color: Rgb888::new(54, 69, 79),
    },
    NamedColor {
        name: "Black",
        color: Rgb888::new(0, 0, 0),
    },
    NamedColor {
        name: "Slate Gray",
        color: Rgb888::new(112, 128, 144),
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
