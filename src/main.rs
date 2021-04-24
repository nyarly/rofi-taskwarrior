mod taskwarrior;
mod rofi;

use taskwarrior::{Status, Task};
use rofi::{Invocation,Retv};
use std::io::{Error, ErrorKind, Result};
use std::cmp::Ordering;
use tabular::{row,Table};
use base64::{decode,encode};
use serde::{Serialize, Deserialize};
use rmp_serde::{from_read,to_vec};

fn main() -> Result<()> {
  let invo = Invocation::env();

  let (mut table, rofi) = if let Some(rofi) = invo {
    (Table::new("{:>}⎜{:<}\0info\x1f{:<}"), rofi)
  } else {
    return all_tasks(&mut Table::new("{:>}  {:<}  {:<}"))
  };

  match rofi.how {
    Retv::Initial => all_tasks(&mut table),
    Retv::Entry(_) => act_on_task(rofi.info.clone().unwrap()),
    Retv::Custom(name) => add_task(name),
    Retv::Key(_) => {warn_no_key(&mut table); all_tasks(&mut table)},
  }
}

fn enc(info: Info) -> String {
  encode(to_vec(&info).unwrap())
}

fn dec(msg: String) -> Result<Info> {
  from_read::<&[u8],_>(
    (decode(msg).map_err(|e| Error::new(ErrorKind::InvalidData, e)))?
    .as_ref()
  ).map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

fn all_tasks(table: &mut Table) -> Result<()> {
  Task::all_tasks().map(|all_tasks| {
      let mut tasks: Vec<_> = all_tasks.iter().filter(|t| matches!(t.status, Status::Pending)).collect();
      tasks.sort_unstable_by(|l, r| l.urgency.partial_cmp(&r.urgency).unwrap_or(Ordering::Less).reverse());
      for task in tasks {
          table.add_row(row!(task.project.as_ref().unwrap_or(&String::from(" ")), task.description.clone(), chooser(task)));
      };
      print!("{}", table);
  })
}

fn chooser(task: &Task) -> String {
  enc(Info::Choose(task.id))
}

fn act_on_task(msg: String) -> Result<()> {
  let info = dec(msg)?;
  use Info::*;
  match info {
    Choose(id) => {options_list(id); Ok(())},
    Start(id) => Task::from_id(id)?.start(),
    Stop(id) => Task::from_id(id)?.stop(),
    Done(id) => Task::from_id(id)?.done(),
    Edit(id) => Task::from_id(id)?.edit(),
    Create(name) => add_task(name),
  }
}

fn add_task(name: String) -> Result<()> {
    Task::create(name)
}

fn warn_no_key(table: &mut Table) {
  table.add_heading("No binding for that key");
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Info {
  Choose(u16),
  Start(u16),
  Stop(u16),
  Done(u16),
  Edit(u16),
  Create(String),
}

fn options_list(id: u16) {
  let mut table = Table::new("{:>} {:<}\0info\x1f{:<}");
  table.add_row(row!("Alt-1", "Start", enc(Info::Start(id))));
  table.add_row(row!("Alt-2", "Stop", enc(Info::Stop(id))));
  table.add_row(row!("Alt-3", "Done", enc(Info::Done(id))));
  table.add_row(row!("Alt-4", "Edit", enc(Info::Edit(id))));
  println!("{}", table);
}
