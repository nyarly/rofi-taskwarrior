use std::env;

pub struct Rofi {
  pub outside: String,
  pub retv: String,
  pub info: Option<String>,
  pub selected: Option<String>,
}

impl Rofi {
  pub fn env() -> Option<Self> {
    let outside = match env::var("ROFI_OUTSIDE") {
      Ok(v) => v,
      _ => return None
    };
    let retv = match env::var("ROFI_RETV") {
      Ok(v) => v,
      _ => return None
    };
    let info = match env::var("ROFI_INFO") {
      Ok(v) => Some(v),
      Err(_) => None
    };

    let selected = env::args().nth(1);

    Some(Rofi{outside, retv, info, selected})
  }
}
