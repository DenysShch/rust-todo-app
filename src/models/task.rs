use core::fmt;

use super::time::{current_timestamp, duration, time_delta, to_human_date};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    InProgress,
    Done,
    Hold,
    New,
}

impl Copy for Status {}

impl Clone for Status {
    fn clone(&self) -> Status {
        *self
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Done => write!(f, "DONE"),
            Status::Hold => write!(f, "HOLD"),
            Status::New => write!(f, "NEW"),
            Status::InProgress => write!(f, "IN PROGRESS"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub message: String,
    pub topic: String,
    pub status: Status,
    pub create_timestamp: i64,
    pub create_time: String,
    pub done_timestamp: Option<i64>,
    pub done_time: Option<String>,
    pub duration: Option<String>,
}

impl Task {
    pub fn create(msg: String, topic: String) -> Self {
        let t = if topic.is_empty() {
            "main".to_string()
        } else {
            topic
        };

        return Task {
            message: msg,
            topic: t,
            status: Status::New,
            create_timestamp: current_timestamp(),
            create_time: to_human_date(current_timestamp()),
            done_timestamp: None,
            done_time: None,
            duration: None,
        };
    }

    pub fn change_status(&mut self) {
        match self.status {
            Status::Done => {
                self.status = Status::New;
                self.done_timestamp = None;
                self.done_time = None;
                self.duration = None;
            }
            Status::Hold => {
                self.status = Status::Done;
                self.done_timestamp = Some(current_timestamp());
                self.done_time = Some(to_human_date(current_timestamp()));
                self.duration = Some(duration(time_delta(
                    self.create_timestamp,
                    self.done_timestamp,
                )));
            }
            Status::InProgress => {
                self.status = Status::Hold;
                self.done_timestamp = Some(current_timestamp());
                self.done_time = Some(to_human_date(current_timestamp()));
                self.duration = Some(duration(time_delta(
                    self.create_timestamp,
                    self.done_timestamp,
                )));
            }
            Status::New => {
                self.status = Status::InProgress;
                self.done_timestamp = Some(current_timestamp());
                self.done_time = None;
                self.duration = None;
            }
        }
    }
}
