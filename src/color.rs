use crate::cmyk::CMYKColor;
use crate::hex::HexColor;
use crate::rgb::RGBColor;

pub trait Color {
  fn to_rgb(self) -> RGBColor;
  fn to_cmyk(self) -> CMYKColor;
  fn to_hex(self) -> HexColor;
}
