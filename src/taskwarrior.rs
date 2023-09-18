use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind, Result};
use std::process::{Command, Stdio};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u16,
    pub description: String,
    #[serde(flatten)]
    pub status: Status,
    #[serde(default)]
    pub project: Option<String>,

    #[serde(default, with = "opt_export_datetime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<DateTime<Utc>>,
    #[serde(default, with = "opt_export_datetime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    due: Option<DateTime<Utc>>,
    #[serde(default, with = "opt_export_datetime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<DateTime<Utc>>,
    #[serde(with = "export_datetime")]
    entry: DateTime<Utc>,
    #[serde(with = "export_datetime")]
    modified: DateTime<Utc>,
    #[serde(default, with = "opt_export_datetime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    until: Option<DateTime<Utc>>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    mask: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    imask: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    recur: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    uuid: String,
    pub urgency: f64,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<Annotation>,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Annotation {
    #[serde(with = "export_datetime")]
    entry: DateTime<Utc>,
    description: String,
}

impl Task {
    pub fn all_tasks() -> Result<Vec<Task>> {
        Command::new("task")
            .arg("export")
            .output()
            .map(|out| serde_json::from_slice(&out.stdout).unwrap())
    }

    pub fn from_id(id: u16) -> Result<Task> {
        Command::new("task")
            .arg(id.to_string())
            .arg("export")
            .output()
            .map(|out| {
                let mut list: Vec<Task> = serde_json::from_slice(&out.stdout).unwrap();
                list.remove(0)
            })
    }

    pub fn create(name: String) -> Result<()> {
        Command::new("task")
            .arg("add")
            .arg(name)
            .status()
            .map(|_| ())
    }

    pub fn start(&self) -> Result<()> {
        let started = Task {
            start: Some(SystemTime::now().into()),
            ..self.clone()
        };
        started.update()
    }

    pub fn stop(&self) -> Result<()> {
        let stopped = Task {
            start: None,
            ..self.clone()
        };
        stopped.update()
    }

    pub fn done(&self) -> Result<()> {
        let done = Task {
            status: Status::Completed,
            ..self.clone()
        };
        done.update()
    }

    fn update(&self) -> Result<()> {
        let mut import = Command::new("task")
            .arg("import")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env_remove("TASKRC")
            .spawn()?;

        let stdin = import
            .stdin
            .as_mut()
            .ok_or_else(|| Error::new(ErrorKind::BrokenPipe, ""))?;
        serde_json::to_writer(stdin, &self).map_err(|se| Error::new(ErrorKind::InvalidData, se))?;
        import.wait().map(|_| ())
    }

    pub fn edit(&self) -> Result<()> {
        Command::new("rofi-sensible-terminal")
            .arg("-e")
            .arg("task")
            .arg("edit")
            .arg(self.id.to_string())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map(|_| ())
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum Status {
    Pending,
    Waiting,
    Deleted,
    Completed,
    Recurring,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Status::Pending => "p",
                Status::Waiting => "w",
                Status::Deleted => "X",
                Status::Completed => "C",
                Status::Recurring => "r",
            }
        )
    }
}

mod opt_export_datetime {
    use super::export_datetime;
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "export_datetime")] &'a DateTime<Utc>);

        date.as_ref().map(Helper).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "export_datetime")] DateTime<Utc>);

        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}

mod export_datetime {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y%m%dT%H%M%SZ";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
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
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}
