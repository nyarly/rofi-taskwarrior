use base64::{decode, encode};
use log::{debug, info, trace};
use rmp_serde::{from_read, to_vec};
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::Debug;
use std::io::{Error, ErrorKind, Result};
use tap::prelude::*;

#[derive(Debug)]
pub struct Invocation<I: Deserialize<'static> + Debug> {
    pub outside: String,
    pub info: Option<I>,
    pub how: Retv,
}

#[derive(Debug)]
pub enum Retv {
    Initial,
    Entry(String),
    Custom(String),
    Key(Keybinding),
}

#[derive(Debug)]
pub struct Keybinding {
    pub key: u8,
    pub entry: Option<String>,
}

impl<I: for<'de> Deserialize<'de> + Debug> Invocation<I> {
    pub fn env() -> Result<Self> {
        let outside = match env::var("ROFI_OUTSIDE") {
            Ok(v) => v,
            _ => return Err(Error::new(ErrorKind::InvalidData, "no ROFI_OUTSIDE")),
        };
        let retv = match env::var("ROFI_RETV") {
            Ok(v) => v.parse().expect("non-numeric RETV"),
            _ => return Err(Error::new(ErrorKind::InvalidData, "no ROFI_RETV")),
        };
        let info = match env::var("ROFI_INFO") {
            Ok(v) => Some(dec(v)?),
            Err(_) => None,
        };

        let selected = env::args().nth(1);

        let how = match retv {
            0 => Retv::Initial,
            1 => Retv::Entry(selected.expect("ROFI_RETV=1, but no arg")),
            2 => Retv::Custom(selected.expect("ROFI_RETV=2, but no arg")),
            i if (10..=28).contains(&i) => Retv::Key(Keybinding {
                key: i - 10,
                entry: selected,
            }),
            _ => return Err(Error::new(ErrorKind::InvalidData, "invalid ROFI_RETV")),
        };

        Ok(Invocation { outside, info, how }.tap(|invoke| info!("Invocation: {:?}", invoke)))
    }
}

pub fn info_string<I: Serialize + Debug>(info: I) -> String {
    format!("info\x1f{}", enc(info))
}

fn enc<I: Serialize + Debug>(info: I) -> String {
    encode(to_vec(&info.tap(|e| trace!("encoded info: {:?}", e))).unwrap())
}

fn dec<I: for<'de> Deserialize<'de> + Debug>(msg: String) -> Result<I> {
    from_read::<&[u8], _>(
        (decode(msg.tap(|m| debug!("decoding: {:?}", m)))
            .map_err(|e| Error::new(ErrorKind::InvalidData, e)))?
        .as_ref(),
    )
    .tap(|e| debug!("decoded info: {:?}", e))
    .map_err(|e| Error::new(ErrorKind::InvalidData, e))
}
