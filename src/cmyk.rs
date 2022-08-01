use crate::color::Color;

/// A hexidecimal value representing a color
pub struct CMYK(u8, u8, u8, u8);

impl Color for CMYK {
  fn to_cmyk(self: Self) -> CMYK {
    self
  }
}
