use std::convert::From;
use substring::Substring;

use crate::cmyk::CMYKColor;
use crate::color::Color;
use crate::rgb::RGBColor;

/// A representation of a color in hexidecimal
pub struct HexColor(String);

impl From<String> for HexColor {
  fn from(value: String) -> HexColor {
    HexColor(value)
  }
}

fn to_number(letter: &str) -> u8 {
  let number: u8 = match letter {
    "A" => 10,
    "B" => 11,
    "C" => 12,
    "D" => 13,
    "E" => 14,
    "F" => 15,
    _ => letter.parse().unwrap(),
  };

  number
}

fn color_value(val: &str) -> u8 {
  let items: Vec<&str> = val.split("").collect();

  let first = to_number(items[0]) * 16;
  let second = to_number(items[1]);

  first + second
}

impl Color for HexColor {
  fn to_hex(self: Self) -> HexColor {
    self
  }

  fn to_rgb(self: Self) -> RGBColor {
    let hex = self.0;
    let first = hex.substring(0, 2);
    let second = hex.substring(2, 4);
    let third = hex.substring(4, 6);

    let r = color_value(first);
    let g = color_value(second);
    let b = color_value(third);

    RGBColor::from(vec![r, g, b])
  }

  fn to_cmyk(self: Self) -> CMYKColor {
    self.to_rgb().to_cmyk()
  }
}
