mod rofi;
mod taskwarrior;

use core::str::FromStr;
use log::warn;
use rofi::{info_string, Invocation, Retv};
use serde::{Deserialize, Serialize};
use simplelog::{LevelFilter, WriteLogger};
use std::cmp::Ordering;
use std::env;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Result};
use tabular::{row, Table};
use taskwarrior::{Status, Task};

fn main() -> Result<()> {
    if let Ok(logpath) = env::var("RTW_LOG") {
        let logfile = if match env::var("ROFI_RETV") {
            Ok(r) if r == "0" => matches!(env::var("RTW_KEEPLOG"), Ok(_)),
            _ => true,
        } {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(logpath)
                .unwrap()
        } else {
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(logpath)
                .unwrap()
        };
        let cfg = simplelog::Config::default();
        let level = match env::var("RTW_LOGLEVEL") {
            Ok(l) => LevelFilter::from_str(&l).unwrap_or(LevelFilter::Info),
            Err(_) => LevelFilter::Info,
        };
        let _ = WriteLogger::init(level, cfg, logfile);
    }

    let invo = Invocation::env();

    let (mut table, rofi) = if let Ok(rofi) = invo {
        (Table::new("{:>}⎜{:<}\0{:<}"), rofi)
    } else {
        warn!("err: {:?}", invo);
        return all_tasks(&mut Table::new("{:>}  {:<}  {:<}"));
    };

    match rofi.how {
        Retv::Initial => all_tasks(&mut table),
        Retv::Entry(_) => act_on_task(rofi.info.unwrap()),
        Retv::Custom(name) => add_task(name),
        Retv::Key(kb) => match kb.key {
            0 => Task::from_id(rofi.info.expect("no task selected").task_id()?)?.start(),
            1 => Task::from_id(rofi.info.expect("no task selected").task_id()?)?.stop(),
            2 => Task::from_id(rofi.info.expect("no task selected").task_id()?)?.done(),
            3 => Task::from_id(rofi.info.expect("no task selected").task_id()?)?.edit(),
            _ => {
                warn_no_key(&mut table);
                all_tasks(&mut table)
            }
        },
    }
}

fn all_tasks(table: &mut Table) -> Result<()> {
    Task::all_tasks().map(|all_tasks| {
        let mut tasks: Vec<_> = all_tasks
            .iter()
            .filter(|t| matches!(t.status, Status::Pending))
            .collect();
        tasks.sort_unstable_by(|l, r| {
            l.urgency
                .partial_cmp(&r.urgency)
                .unwrap_or(Ordering::Less)
                .reverse()
        });
        for task in tasks {
            table.add_row(row!(
                task.project.as_ref().unwrap_or(&String::from(" ")),
                task.description.clone(),
                info_string(Info::Choose(task.id))
            ));
        }
        print!("{}", table);
    })
}

fn act_on_task(info: Info) -> Result<()> {
    use Info::*;
    match info {
        Choose(id) => {
            options_list(id);
            Ok(())
        }
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

#[derive(Debug, Serialize, Deserialize)]
pub enum Info {
    Choose(u16),
    Start(u16),
    Stop(u16),
    Done(u16),
    Edit(u16),
    Create(String),
}

impl Info {
    fn task_id(&self) -> Result<u16> {
        use Info::*;
        match self {
            Choose(id) | Start(id) | Stop(id) | Done(id) | Edit(id) => Ok(*id),
            Create(_) => Err(Error::new(
                ErrorKind::InvalidInput,
                "can't create and use shortcut",
            )),
        }
    }
}

fn options_list(id: u16) {
    let mut table = Table::new("{:>} {:<}\0{:<}");
    table.add_row(row!("Alt-1", "Start", info_string(Info::Start(id))));
    table.add_row(row!("Alt-2", "Stop", info_string(Info::Stop(id))));
    table.add_row(row!("Alt-3", "Done", info_string(Info::Done(id))));
    table.add_row(row!("Alt-4", "Edit", info_string(Info::Edit(id))));
    println!("{}", table);
}
