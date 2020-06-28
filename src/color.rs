use blinkrs::Color;
use serde::{Deserialize, Serialize};

/// Represents the color of the Led.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Led {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Pink,
    Cyan,
    White,
    Blank,
}

impl From<&Led> for Color {
    fn from(led_color: &Led) -> Color {
        from(led_color)
    }
}

fn from(led_color: &Led) -> Color {
    match *led_color {
        Led::Red => Color::Red,
        Led::Green => Color::Green,
        Led::Blue => Color::Blue,
        Led::Yellow => Color::Three(255, 255, 0),
        Led::Orange => Color::Three(255, 165, 0),
        Led::Pink => Color::Three(255, 192, 203),
        Led::Cyan => Color::Three(0, 255, 255),
        Led::White => Color::Three(255, 255, 255),
        Led::Blank => Color::Three(0, 0, 00),
    }
}

impl From<Led> for Color {
    fn from(led_color: Led) -> Color {
        from(&led_color)
    }
}
