use std::env;

pub struct Invocation {
  pub outside: String,
  pub info: Option<String>,
  pub how: Retv,
}

pub enum Retv {
  Initial,
  Entry(String),
  Custom(String),
  Key(Keybinding)
}

pub struct Keybinding {
  pub key: u8,
  pub shift: bool,
  pub entry: Option<String>,
}

impl Invocation {
  pub fn env() -> Option<Self> {
    let outside = match env::var("ROFI_OUTSIDE") {
      Ok(v) => v,
      _ => return None
    };
    let retv = match env::var("ROFI_RETV") {
      Ok(v) => v.parse().expect("non-numeric RETV"),
      _ => return None
    };
    let info = match env::var("ROFI_INFO") {
      Ok(v) => Some(v),
      Err(_) => None
    };

    let selected = env::args().nth(1);

    let how = match retv {
      0 => Retv::Initial,
      1 => Retv::Entry(selected.expect("ROFI_RETV=1, but no arg")),
      2 => Retv::Custom(selected.expect("ROFI_RETV=2, but no arg")),
      i if (10..20).contains(&i)  => Retv::Key(Keybinding{key: i - 9, shift: false, entry: selected}),
      i if (20..=28).contains(&i) => Retv::Key(Keybinding{key: i - 19, shift: true, entry: selected}),
      _ => return None,
    };


    Some(Invocation{outside, info, how})
  }
}
