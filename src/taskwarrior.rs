use std::process::Command;
use std::io::Result;
use chrono::{DateTime,Utc};
use serde::{Serialize, Deserialize};

pub struct TaskWarrior {
}

impl TaskWarrior {
  pub fn run(self) -> Result<Vec<Task>> {
      Command::new("task")
          .arg("export")
          .output()
          .map(|out| {
            serde_json::from_slice(&out.stdout).unwrap()
          })
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u16,
    pub description: String,
    #[serde(flatten)]
    pub status: Status,
    #[serde(default)]
    pub project: Option<String>,

    #[serde(default, with = "opt_export_datetime")]
    due: Option<DateTime<Utc>>,
    #[serde(default, with = "opt_export_datetime")]
    end: Option<DateTime<Utc>>,
    #[serde(with = "export_datetime")]
    entry: DateTime<Utc>,
    #[serde(with = "export_datetime")]
    modified: DateTime<Utc>,
    #[serde(default, with = "opt_export_datetime")]
    until: Option<DateTime<Utc>>,

    #[serde(default)]
    mask: Option<String>,
    #[serde(default)]
    imask: Option<f64>,
    #[serde(default)]
    parent: Option<String>,
    #[serde(default)]
    recur: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    uuid: String,
    pub urgency: f64,
}

#[derive(Serialize,Deserialize,Debug)]
#[serde(tag = "status", rename_all="lowercase")]
pub enum Status {
    Pending,
    Waiting,
    Started,
    Deleted,
    Completed,
    Recurring,
}

mod opt_export_datetime {
use chrono::{DateTime,Utc};
    use super::export_datetime;
    use serde::{self, Serialize, Serializer, Deserialize, Deserializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "export_datetime")] &'a DateTime<Utc>);

        date.as_ref().map(Helper).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
        where D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "export_datetime")] DateTime<Utc>);

        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}

mod export_datetime {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &str = "%Y%m%dT%H%M%SZ";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
