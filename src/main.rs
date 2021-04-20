mod taskwarrior;
mod rofi;

use taskwarrior::{Status, TaskWarrior};
use rofi::Rofi;
use std::io::Result;
use std::cmp::Ordering;
use tabular::{row,Table};

fn main() -> Result<()> {
  let rofip = Rofi::env();

  let mut table = Table::new("{:>}⎜{:<}\0info\x1f{:<}");
  if let Some(rofi) = rofip {
    table.add_row(row!("OUTSIDE", rofi.outside, ""));
    table.add_row(row!("RETV", rofi.retv, ""));
    table.add_row(row!("INFO", rofi.info.unwrap_or_else(||"<none>".to_string()), ""));
    table.add_row(row!("SELECTED", rofi.selected .unwrap_or_else(||"<none>".to_string()), ""));
  }

  TaskWarrior{}.run().map(|all_tasks| {
      let mut tasks: Vec<_> = all_tasks.iter().filter(|t| matches!(t.status, Status::Pending|Status::Started)).collect();
      tasks.sort_unstable_by(|l, r| l.urgency.partial_cmp(&r.urgency).unwrap_or(Ordering::Less).reverse());
      for task in tasks {
          table.add_row(row!(task.project.as_ref().unwrap_or(&String::from(" ")), task.description.clone(), task.id));
      };
      print!("{}", table);
  })
}
