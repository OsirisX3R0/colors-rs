use std::convert::From;

use crate::color::Color;
use crate::hex::HexColor;
use crate::rgb::RGBColor;

/// A representation of a colors cyan, magenta, yellow and black values
pub struct CMYKColor(u8, u8, u8, u8);

impl From<Vec<u8>> for CMYKColor {
  fn from(values: Vec<u8>) -> CMYKColor {
    CMYKColor(values[0], values[1], values[2], values[3])
  }
}

impl Color for CMYKColor {
  fn to_cmyk(self: Self) -> CMYKColor {
    self
  }

  fn to_rgb(self: Self) -> RGBColor {
    let CMYKColor(c, m, y, k) = self;

    let r = 255 * (1 - c) * (1 - k);
    let g = 255 * (1 - m) * (1 - k);
    let b = 255 * (1 - y) * (1 - k);

    RGBColor::from(vec![r, g, b])
  }

  fn to_hex(self: Self) -> HexColor {
    self.to_rgb().to_hex()
  }
}
