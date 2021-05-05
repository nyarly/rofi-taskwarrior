use std::env;
use base64::{decode,encode};
use rmp_serde::{from_read,to_vec};
use serde::{Serialize, Deserialize};
use std::io::{Error, ErrorKind, Result};

pub struct Invocation<I: Deserialize<'static>> {
  pub outside: String,
  pub info: Option<I>,
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
  pub entry: Option<String>,
}

impl<I: for<'de> Deserialize<'de>> Invocation<I> {
  pub fn env() -> Result<Self> {
    let outside = match env::var("ROFI_OUTSIDE") {
      Ok(v) => v,
      _ => return Err(Error::new(ErrorKind::InvalidData, "no ROFI_OUTSIDE"))
    };
    let retv = match env::var("ROFI_RETV") {
      Ok(v) => v.parse().expect("non-numeric RETV"),
      _ => return Err(Error::new(ErrorKind::InvalidData, "no ROFI_RETV"))
    };
    let info = match env::var("ROFI_INFO") {
      Ok(v) => Some(dec(v)?),
      Err(_) => None
    };

    let selected = env::args().nth(1);

    let how = match retv {
      0 => Retv::Initial,
      1 => Retv::Entry(selected.expect("ROFI_RETV=1, but no arg")),
      2 => Retv::Custom(selected.expect("ROFI_RETV=2, but no arg")),
      i if (10..=28).contains(&i)  => Retv::Key(Keybinding{key: i - 10, entry: selected}),
      _ => return Err(Error::new(ErrorKind::InvalidData, "invalid ROFI_RETV"))
    };


    Ok(Invocation{outside, info, how})
  }
}

pub fn info_string<I: Serialize>(info: I) -> String {
  format!("info\x1f{}", enc(info))
}

fn enc<I: Serialize>(info: I) -> String {
  encode(to_vec(&info).unwrap())
}

fn dec<I: for<'de> Deserialize<'de>>(msg: String) -> Result<I> {
  from_read::<&[u8],_>(
    (decode(msg).map_err(|e| Error::new(ErrorKind::InvalidData, e)))?
    .as_ref()
  ).map_err(|e| Error::new(ErrorKind::InvalidData, e))
}
