use crate::cmyk::CMYK;
use crate::hex::Hex;
use crate::rgb::RGB;

pub trait Color {
  fn to_rgb(self) -> RGB;
  fn to_cmyk(self) -> CMYK;
  fn to_hex(self) -> Hex;
}
