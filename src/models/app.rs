use ratatui::widgets::*;
use std::fs;

use super::{os::FileSystem, task::Task};

pub enum InputMode {
    Normal,
    Editing,
}

#[derive(PartialEq, Eq)]
pub enum EnterMode {
    Input,
    Topic,
}
pub struct InsertApp {
    pub input: String,
    pub topic: String,
    pub topic_len: usize,
    pub input_len: usize,
    pub cursor_position_x: usize,
    pub cursor_position_y: usize,
    pub input_mode: InputMode,
    pub enter_mode: EnterMode,
    pub messages: Vec<String>,
    pub edit: bool,
}

impl Default for InsertApp {
    fn default() -> InsertApp {
        InsertApp {
            input: String::new(),
            topic: String::new(),
            topic_len: 20,
            input_len: 80,
            cursor_position_x: 0,
            cursor_position_y: 0,
            input_mode: InputMode::Normal,
            enter_mode: EnterMode::Topic,
            messages: Vec::new(),
            edit: false,
        }
    }
}

impl InsertApp {
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position_x.saturating_sub(1);
        self.cursor_position_x = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position_x.saturating_add(1);
        self.cursor_position_x = self.clamp_cursor(cursor_moved_right);
    }

    fn move_cursor_down(&mut self) {
        let cursor_moved_down = self.cursor_position_y.saturating_add(1);
        self.cursor_position_y = self.clamp_cursor(cursor_moved_down);
    }

    fn move_cursor_up(&mut self) {
        let cursor_moved_up = self.cursor_position_y.saturating_sub(1);
        self.cursor_position_y = self.clamp_cursor(cursor_moved_up);
    }

    pub fn enter_char(&mut self, new_char: char) {
        if self.cursor_position_x % self.input_len == 0
            && self.cursor_position_x != 0
            && self.enter_mode == EnterMode::Input
        {
            self.input.insert(self.cursor_position_x, '\n');
            self.move_cursor_right();
            self.move_cursor_down();
            self.input.insert(self.cursor_position_x, new_char);
            self.move_cursor_right();
        } else {
            match self.enter_mode {
                EnterMode::Input => self.input.insert(self.cursor_position_x, new_char),
                EnterMode::Topic => {
                    if self.topic.len() <= self.topic_len {
                        self.topic.insert(self.cursor_position_x, new_char)
                    }
                }
            };
            self.move_cursor_right();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        match self.enter_mode {
            EnterMode::Input => new_cursor_pos.clamp(0, self.input.len()),
            EnterMode::Topic => new_cursor_pos.clamp(0, self.topic.len()),
        }
    }

    pub fn reset_cursor_x(&mut self) {
        self.cursor_position_x = 0;
    }

    pub fn reset_cursor_y(&mut self) {
        self.cursor_position_y = 0;
    }

    pub fn return_caret(&mut self) -> usize {
        if self.cursor_position_y == 0 {
            return self.cursor_position_x;
        }
        return self.cursor_position_x.saturating_sub(1) % self.input_len;
    }

    pub fn switch_enter_mode(&mut self) {
        match self.enter_mode {
            EnterMode::Input => {
                self.enter_mode = EnterMode::Topic;
                self.move_cursor_right();
                self.cursor_position_x = self.clamp_cursor(self.topic.len());
                self.cursor_position_y = 0;
            }
            EnterMode::Topic => {
                self.enter_mode = EnterMode::Input;
                self.cursor_position_x = self.clamp_cursor(self.input.len());
                if self.input.len() < self.input_len {
                    self.cursor_position_y = 0;
                } else {
                    self.cursor_position_y = self.cursor_position_x / self.input_len;
                }
            }
        }
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position_x != 0;
        if is_not_cursor_leftmost {
            let current_index = self.cursor_position_x;
            let from_left_to_current_index = current_index - 1;
            match self.enter_mode {
                EnterMode::Input => {
                    let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
                    let after_char_to_delete = self.input.chars().skip(current_index);
                    self.input = before_char_to_delete.chain(after_char_to_delete).collect();
                }
                EnterMode::Topic => {
                    let before_char_to_delete = self.topic.chars().take(from_left_to_current_index);
                    let after_char_to_delete = self.topic.chars().skip(current_index);
                    self.topic = before_char_to_delete.chain(after_char_to_delete).collect();
                }
            }
            if from_left_to_current_index % self.input_len == 0 {
                self.move_cursor_up();
            }
            self.move_cursor_left();
        }
    }

    pub fn add_new_line(&mut self) {
        self.input.insert(self.cursor_position_x, '\n');
        self.move_cursor_right();
        self.move_cursor_down();
    }

    pub fn clear_popup(&mut self) {
        self.enter_mode = EnterMode::Topic;
        self.cursor_position_x = 0;
        self.topic = String::new();
        self.input = String::new();
    }
}

pub struct App {
    pub state: TableState,
    pub items: Vec<Task>,
    pub insert_popup: bool,
    pub info_popup: bool,
    pub insert_app: InsertApp,
}

impl App {
    fn read() -> Vec<Task> {
        let path = App::get_path();
        let file_path = format!("{}/todo.json", path);
        let file_content = fs::read_to_string(file_path);
        let tasks = {
            match file_content {
                Ok(s) => serde_json::from_str::<Vec<Task>>(&s).unwrap(),
                Err(_) => vec![Task {
                    message: "-//-".to_string(),
                    topic: "main".to_string(),
                    status: super::task::Status::New,
                    create_timestamp: 1,
                    create_time: "   -//-   ".to_string(),
                    done_timestamp: None,
                    done_time: None,
                    duration: None,
                }],
            }
        };
        return tasks;
    }

    pub fn write(&mut self) {
        let path = App::get_path();
        let file_path = format!("{}/todo.json", path);
        let _ = std::fs::write(
            file_path,
            serde_json::to_string_pretty(&self.items).unwrap(),
        );
    }

    pub fn new() -> App {
        let tasks = App::read();
        App {
            state: TableState::default(),
            items: tasks,
            insert_popup: false,
            info_popup: false,
            insert_app: InsertApp::default(),
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        if self.items.len() != 0 {
            self.state.select(Some(i));
        }
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        if self.items.len() != 0 {
            self.state.select(Some(i));
        }
    }

    pub fn create(&mut self) {
        if !self.insert_app.input.is_empty() {
            let task = Task::create(self.insert_app.input.clone(), self.insert_app.topic.clone());
            self.items.push(task);
            self.write();
        }
        self.insert_app.input.clear();
        self.insert_app.reset_cursor_x();
        self.insert_app.reset_cursor_y();
    }

    pub fn delete(&mut self) {
        match self.state.selected() {
            Some(i) => {
                self.items.remove(i);
                self.write();
            }
            None => (),
        };
    }

    pub fn edit(&mut self) {
        match self.state.selected() {
            Some(i) => {
                let topic = self.items.get(i).unwrap().topic.clone();
                let message = self.items.get(i).unwrap().message.clone();
                let len = message.len();
                self.insert_app.enter_mode = EnterMode::Topic;
                self.insert_app.cursor_position_x = self.insert_app.clamp_cursor(len);
                self.insert_app.cursor_position_y = self.insert_app.cursor_position_x / len;
                self.insert_app.topic = topic;
                self.insert_app.input = message;
            }
            None => (),
        };
    }

    pub fn modify(&mut self) {
        match self.state.selected() {
            Some(i) => {
                let task =
                    Task::create(self.insert_app.input.clone(), self.insert_app.topic.clone());
                self.items[i] = task;
                self.write();
                self.insert_app.input.clear();
                self.insert_app.reset_cursor_x();
                self.insert_app.reset_cursor_y();
            }
            None => (),
        }
    }

    pub fn change_status(&mut self) {
        match self.state.selected() {
            Some(i) => {
                let task = self.items.get_mut(i).unwrap();
                task.change_status();
            }
            None => (),
        };
        self.write();
    }

    pub fn sort_by_start(&mut self) {
        self.items.sort_by_key(|f| f.create_timestamp);
        self.write();
    }

    pub fn sort_by_end(&mut self) {
        self.items.sort_by_key(|f| f.done_timestamp);
        self.write();
    }

    pub fn sort_by_status(&mut self) {
        self.items.sort_by_key(|f| f.status);
        self.write();
    }

    pub fn sort_by_topic(&mut self) {
        self.items.sort_by_key(|f| f.topic.clone());
        self.write();
    }
}

impl FileSystem for App {}
