use crate::color::Color;

/// A hexidecimal value representing a color
pub struct RGB(u8, u8, u8);

impl Color for RGB {
  fn to_rgb(self: Self) -> RGB {
    self
  }
}
