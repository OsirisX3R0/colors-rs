use std::convert::From;

use crate::color::Color;
use crate::hex::Hex;
use crate::rgb::RGB;

/// A hexidecimal value representing a color
pub struct CMYK(u8, u8, u8, u8);

impl From<Vec<u8>> for CMYK {
  fn from(values: Vec<u8>) -> CMYK {
    CMYK(values[0], values[1], values[2], values[3])
  }
}

impl Color for CMYK {
  fn to_cmyk(self: Self) -> CMYK {
    self
  }

  fn to_rgb(self: Self) -> RGB {}

  fn to_hex(self: Self) -> Hex {
    self.to_rgb().to_hex()
  }
}
