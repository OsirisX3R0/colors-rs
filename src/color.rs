use crate::hex::Hex

pub trait Color {
  pub fn to_rgb(self: Self) -> RGB
  pub fn to_cmyk(self: Self) -> CMYK
  pub fn to_hex(self: Self) -> Hex
}