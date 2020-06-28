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
        Led::Blank => Color::Three(0x00, 0x00, 0x00),
    }
}

impl From<Led> for Color {
    fn from(led_color: Led) -> Color {
        from(&led_color)
    }
}
