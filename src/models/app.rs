use super::{
    os::FileSystem,
    task::{Status, Task},
};
use core::fmt;
use ratatui::widgets::*;
use std::fs;
use tui_textarea::TextArea;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Filter {
    New,
    Hold,
    InProgress,
    Done,
    NotDone,
    All,
}

impl Filter {
    pub fn iterator() -> impl Iterator<Item = Filter> {
        [
            Filter::New,
            Filter::Hold,
            Filter::InProgress,
            Filter::Done,
            Filter::NotDone,
            Filter::All,
        ]
        .iter()
        .copied()
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Filter::New => write!(f, "<n> by [New]"),
            Filter::Hold => write!(f, "<h> by [Hold]"),
            Filter::InProgress => write!(f, "<i> by [In Progress]"),
            Filter::Done => write!(f, "<d> by [Done]"),
            Filter::NotDone => write!(f, "<o> by [Not Done]"),
            Filter::All => write!(f, "<a> by [All]"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum InputMode {
    Normal,
    Editing,
    Modify,
    Comment,
    CommentEdit,
    SubTask,
    SubTaskModify,
    Help,
    FilterMode,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PageLayout {
    Vertical,
    Horizontal,
}

#[derive(Copy, Clone)]
pub enum InputArea {
    Topic,
    Task,
    Description,
    Comment,
}

impl InputArea {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

pub struct App {
    pub scroll_state: ScrollbarState,
    pub scroll: usize,
    pub state: ListState,
    pub layout: PageLayout,
    pub items: Vec<Task>,
    pub sub_items: Vec<Task>,
    pub input_mode: InputMode,
    pub input_area: InputArea,
    pub filter: Filter,
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
                    id: 0,
                    topic: "main".to_string(),
                    status: Status::New,
                    name: "Hello this is default task".to_string(),
                    description: "Some description...".to_string(),
                    creation_timestamp: 1,
                    creation_date: "1".to_string(),
                    status_change_timestamp: None,
                    status_change_date: None,
                    duration: None,
                    comments: Vec::new(),
                    child_list: Vec::new(),
                    parent_id: None,
                    is_sub_task: false,
                    display: true,
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
            scroll_state: ScrollbarState::default(),
            scroll: 0,
            state: {
                if tasks.len() > 0 {
                    let mut st = ListState::default();
                    st.select(Some(0));
                    st
                } else {
                    ListState::default()
                }
            },
            layout: PageLayout::Vertical,
            items: tasks,
            sub_items: Vec::new(),
            input_mode: InputMode::Normal,
            input_area: InputArea::Topic,
            filter: Filter::All,
        }
    }

    pub fn create(
        &mut self,
        topic: &TextArea,
        name: &TextArea,
        descripiton: &TextArea,
        comment: &TextArea,
    ) {
        match self.input_mode {
            InputMode::CommentEdit => match self.state.selected() {
                Some(i) => {
                    self.delete_comment();
                    let _comment: String = comment.clone().into_lines().join("\n").to_string();
                    self.items[i].create_comment(_comment);
                    self.write();
                    self.input_mode = InputMode::Normal;
                    self.input_area = InputArea::Topic;
                }
                None => (),
            },
            InputMode::Comment => match self.state.selected() {
                Some(i) => {
                    let _comment: String = comment.clone().into_lines().join("\n").to_string();
                    self.items[i].create_comment(_comment);
                    self.write();
                    self.input_mode = InputMode::Normal;
                    self.input_area = InputArea::Topic;
                }
                None => (),
            },
            _ => {
                let _name: String = name.clone().into_lines().concat().to_string();
                if !_name.is_empty() {
                    let _topic: String = topic.clone().into_lines().concat().to_string();
                    let _description: String =
                        descripiton.clone().into_lines().join("\n").to_string();
                    let mut task =
                        Task::create(Some(_topic), _name, Some(_description), None, None);

                    match self.input_mode {
                        InputMode::Modify | InputMode::SubTaskModify => match self.state.selected()
                        {
                            Some(s) => {
                                let mut modify_task = self.items[s].clone();
                                modify_task.topic = topic.clone().into_lines().concat().to_string();
                                modify_task.name = name.clone().into_lines().concat().to_string();
                                modify_task.description =
                                    descripiton.clone().into_lines().join("\n").to_string();
                                modify_task.child_list = self.items[s].child_list.clone();
                                modify_task.is_sub_task = self.items[s].is_sub_task;
                                self.items[s] = modify_task;
                            }
                            None => (),
                        },
                        InputMode::SubTask => match self.state.selected() {
                            Some(s) => {
                                let parent_index = {
                                    match self.items[s].parent_id {
                                        Some(p) => self.index_by_id(p).unwrap_or(s),
                                        None => s,
                                    }
                                };
                                let child_id = task.id;
                                self.items[parent_index].child_list.push(child_id);
                                task.topic = self.items[parent_index].topic.clone();
                                task.is_sub_task = true;
                                task.parent_id = Some(self.items[parent_index].id);
                                self.items.insert(
                                    parent_index + self.items[parent_index].child_list.len(),
                                    task,
                                );
                            }
                            None => (),
                        },
                        _ => {
                            self.items.push(task);
                        }
                    }
                    self.write();
                    self.input_mode = InputMode::Normal;
                    self.input_area = InputArea::Topic;
                }
            }
        }
    }

    pub fn delete(&mut self) {
        match self.state.selected() {
            Some(i) => {
                let task = self.items[i].clone();
                if task.is_sub_task {
                    let p_task = self
                        .items
                        .iter_mut()
                        .enumerate()
                        .filter(|(_idx, f)| f.id == task.parent_id.unwrap())
                        .nth(0);
                    match p_task {
                        Some(item) => {
                            let (idx, parent_task) = item;
                            let mut clone_task = parent_task.clone();
                            let mod_sub_task: Vec<_> = clone_task
                                .child_list
                                .iter()
                                .filter(|id| *id != &task.id)
                                .cloned()
                                .collect();
                            clone_task.child_list = mod_sub_task;
                            self.items[idx] = clone_task;
                        }
                        None => (),
                    }
                    self.items.remove(i);
                    self.write();
                    self.next();
                } else if task.child_list.len() > 0 {
                    let new_items: Vec<_> = self
                        .items
                        .iter()
                        .filter(|x| !task.child_list.contains(&x.id))
                        .filter(|x| task.id != x.id)
                        .cloned()
                        .collect();
                    self.items = new_items;
                    self.write();
                    self.next();
                } else {
                    self.items.remove(i);
                    self.write();
                    self.next();
                }
            }
            None => (),
        };
    }

    pub fn delete_comment(&mut self) {
        match self.state.selected() {
            Some(i) => {
                self.items.get_mut(i).unwrap().comments.pop();
                self.write();
            }
            None => (),
        };
    }

    pub fn edit(&mut self) -> Option<(String, String, String, bool)> {
        let data = match self.state.selected() {
            Some(i) => Some((
                self.items.get(i).unwrap().topic.clone(),
                self.items.get(i).unwrap().name.clone(),
                self.items.get(i).unwrap().description.clone(),
                self.items.get(i).unwrap().is_sub_task.clone(),
            )),
            None => None,
        };
        return data;
    }

    pub fn edit_last_comment(&mut self) -> Option<String> {
        let data = match self.state.selected() {
            Some(i) => {
                let comment = self.items.get(i).unwrap().comments.last();
                if let Some(c) = comment {
                    Some(c.text.clone())
                } else {
                    None
                }
            }
            None => None,
        };
        return data;
    }

    pub fn change_status(&mut self) {
        match self.state.selected() {
            Some(i) => {
                let mut subtask_done_filter = true;
                let task = self.items.get_mut(i).unwrap();
                for item in self.sub_items.iter() {
                    if item.status != Status::Done {
                        subtask_done_filter = false;
                    }
                }
                task.change_status(subtask_done_filter);
            }
            None => (),
        };
        self.write();
    }

    fn get_child_list(&mut self, some_item: Option<usize>) {
        match some_item {
            Some(i) => {
                let child_list = self.items.get(i).unwrap().child_list.clone();
                let sub_task: Vec<Task> = self
                    .items
                    .iter()
                    .filter(|f| child_list.iter().any(|&item| item == f.id))
                    .map(|x| x.clone())
                    .collect();
                self.sub_items = sub_task;
            }
            None => (),
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if self.items.len() > 0 && i >= self.items.len() - 1 {
                    Some(0)
                } else {
                    Some(i + 1)
                }
            }
            None => {
                if self.items.len() != 0 {
                    Some(0)
                } else {
                    None
                }
            }
        };
        if self.items.len() > 0 {
            self.get_child_list(i);
            self.state.select(i);
        } else {
            self.state.select(None);
        }
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    Some(self.items.len() - 1)
                } else {
                    Some(i - 1)
                }
            }
            None => {
                if self.items.len() != 0 {
                    Some(0)
                } else {
                    None
                }
            }
        };
        self.get_child_list(i);
        self.state.select(i);
    }

    pub fn change_input_area(&mut self) {
        match self.input_area {
            InputArea::Topic => self.input_area = InputArea::Task,
            InputArea::Task => self.input_area = InputArea::Description,
            InputArea::Description => self.input_area = InputArea::Topic,
            _ => (),
        }
    }

    fn index_by_id(&self, id: i64) -> Option<usize> {
        for (idx, task) in self.items.iter().enumerate() {
            if task.id == id {
                return Some(idx);
            }
        }
        return None;
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
        self.scroll_state = self.scroll_state.position(self.scroll)
    }

    pub fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(1);
        self.scroll_state = self.scroll_state.position(self.scroll)
    }

    pub fn filter_items(&mut self, new_filter: Filter) {
        self.filter = new_filter;
        self.input_mode = InputMode::Normal;
        let new_items: Vec<Task> = self
            .items
            .iter()
            .map(|x| {
                let mut task = x.clone();

                match new_filter {
                    Filter::New => {
                        if task.status == Status::New {
                            task.display = true
                        } else {
                            task.display = false
                        }
                    }
                    Filter::Hold => {
                        if task.status == Status::Hold {
                            task.display = true
                        } else {
                            task.display = false
                        }
                    }
                    Filter::InProgress => {
                        if task.status == Status::InProgress {
                            task.display = true
                        } else {
                            task.display = false
                        }
                    }
                    Filter::Done => {
                        if task.status == Status::Done {
                            task.display = true
                        } else {
                            task.display = false
                        }
                    }
                    Filter::NotDone => {
                        if task.status != Status::Done {
                            task.display = true
                        } else {
                            task.display = false
                        }
                    }
                    Filter::All => task.display = true,
                }
                task
            })
            .map(|mut x| {
                if x.child_list.len() > 0 {
                    let child_list = x.child_list.clone();
                    for child_id in child_list {
                        let child_index = self.index_by_id(child_id);
                        match child_index {
                            Some(idx) => {
                                let sub_task_display = self.items.get(idx).unwrap().display;
                                if x.display == false && sub_task_display == true {
                                    x.display = true;
                                }
                            }
                            None => (),
                        }
                    }
                }
                x
            })
            .collect();
        self.items = new_items;
    }
}

impl FileSystem for App {}
