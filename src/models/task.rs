use super::time::{current_timestamp, duration, time_delta, to_human_date};
use core::fmt;
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Comment {
    pub date: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Task {
    pub id: i64,
    pub topic: String,
    pub status: Status,
    pub name: String,
    pub description: String,
    pub creation_timestamp: i64,
    pub creation_date: String,
    pub status_change_timestamp: Option<i64>,
    pub status_change_date: Option<String>,
    pub duration: Option<String>,
    pub comments: Vec<Comment>,
    pub child_list: Vec<i64>,
    pub parent_id: Option<i64>,
    pub is_sub_task: bool,
    pub display: bool,
}

impl Task {
    pub fn create(
        topic: Option<String>,
        name: String,
        description: Option<String>,
        child_list: Option<Vec<i64>>,
        is_sub_task: Option<bool>,
    ) -> Self {
        let c_time = current_timestamp();
        return Self {
            id: c_time,
            topic: topic.unwrap_or(String::from("main")),
            status: Status::New,
            name,
            description: description.unwrap_or(String::new()),
            creation_timestamp: c_time,
            creation_date: to_human_date(c_time),
            status_change_timestamp: None,
            status_change_date: None,
            duration: None,
            comments: Vec::new(),
            child_list: child_list.unwrap_or(Vec::new()),
            parent_id: None,
            is_sub_task: is_sub_task.unwrap_or(false),
            display: true,
        };
    }

    pub fn create_comment(&mut self, text: String) {
        let date = to_human_date(current_timestamp());
        self.comments.push(Comment { date, text })
    }

    pub fn change_status(&mut self, is_sub_task_done:bool) {
        match self.status {
            Status::Done => {
                self.status = Status::New;
                self.status_change_timestamp = None;
                self.status_change_date = None;
                self.duration = None;
            }
            Status::Hold => {
                if !is_sub_task_done {
                    self.status = Status::New;
                } else {
                    self.status = Status::Done;
                }
                let c_time = current_timestamp();
                self.status_change_timestamp = Some(c_time);
                self.status_change_date = Some(to_human_date(c_time));
                self.duration = Some(duration(time_delta(
                    self.creation_timestamp,
                    self.status_change_timestamp,
                )));
            }
            Status::InProgress => {
                let c_time = current_timestamp();
                self.status = Status::Hold;
                self.status_change_timestamp = Some(c_time);
                self.status_change_date = Some(to_human_date(c_time));
                self.duration = Some(duration(time_delta(
                    self.creation_timestamp,
                    self.status_change_timestamp,
                )));
            }
            Status::New => {
                let c_time = current_timestamp();
                self.status = Status::InProgress;
                self.status_change_timestamp = Some(c_time);
                self.status_change_date = None;
                self.duration = None;
            }
        }
    }
}

