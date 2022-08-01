use crate::color::Color;

/// A hexidecimal value representing a color
pub struct Hex(str);

impl Color for Hex {
  fn to_hex(self: Self) -> Hex {
    self
  }
}
