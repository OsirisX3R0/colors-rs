use std::convert::From;

use crate::cmyk::CMYK;
use crate::color::Color;
use crate::hex::Hex;

fn to_letter(num: u8) -> String {
  let letter = match num {
    10 => String::from("A"),
    11 => String::from("B"),
    12 => String::from("C"),
    13 => String::from("D"),
    14 => String::from("E"),
    15 => String::from("F"),
    _ => num.to_string(),
  };

  letter
}

fn hex_value(num: u8) -> String {
  let color = num;

  if color > 255 {
    color = 255
  }

  if color < 0 {
    color = 0
  }

  format!("{}{}", to_letter(color / 16), to_letter(color % 16))
}

// TODO: REPLACE
fn max(r: u8, g: u8, b: u8) -> u8 {
  if r < g {
    if r < b {
      r
    } else {
      b
    }
  } else {
    if g < b {
      g
    } else {
      b
    }
  }
}

/// A hexidecimal value representing a color
pub struct RGB(u8, u8, u8);

impl From<Vec<u8>> for RGB {
  fn from(values: Vec<u8>) -> RGB {
    RGB(values[0], values[1], values[2])
  }
}

impl Color for RGB {
  fn to_rgb(self: Self) -> RGB {
    self
  }

  fn to_hex(self: Self) -> Hex {
    let RGB(r, g, b) = self;

    let value = format!("{}{}{}", hex_value(r), hex_value(g), hex_value(b));

    Hex::from(value)
  }

  fn to_cmyk(self: Self) -> CMYK {
    let RGB(r, g, b) = self;

    let red_percentage = r / 255;
    let green_percentage = g / 255;
    let blue_percentage = b / 255;

    let k = 1 - max(red_percentage, green_percentage, blue_percentage);
    let c = (1 - red_percentage - k) / (1 - k);
    let m = (1 - green_percentage - k) / (1 - k);
    let y = (1 - blue_percentage - k) / (1 - k);

    CMYK::from(vec![c, m, y, k])
  }
}
