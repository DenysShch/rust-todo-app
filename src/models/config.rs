use config::{Config, ConfigError, File};
use serde::Deserialize;

use super::os::FileSystem;

#[derive(Debug, Deserialize)]
pub struct Icons {
    pub cursor: String,
    pub task_new: String,
    pub task_in_progress: String,
    pub task_hold: String,
    pub task_done: String,
    pub topic_icon_left: String,
    pub topic_icon_right: String,
    pub sub_task_middle: String,
    pub sub_task_end: String,
}

#[derive(Debug, Deserialize)]
pub struct ObjectType {
    pub border_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub bat_color_sheme: String,
    pub header_color: String,
    pub footer_color: String,
    pub border_color: String,

    pub task_topic_color_new_fg: String,
    pub task_topic_color_in_progress_fg: String,
    pub task_topic_color_hold_fg: String,
    pub task_topic_color_done_fg: String,

    pub task_topic_color_new_bg: String,
    pub task_topic_color_in_progress_bg: String,
    pub task_topic_color_hold_bg: String,
    pub task_topic_color_done_bg: String,

    pub task_text_color: String,
    pub task_text_color_done: String,
    pub task_date_color: String,
    pub task_duration_color: String,

    pub icon_new_color: String,
    pub icon_progress_color: String,
    pub icon_hold_color: String,
    pub icon_done_color: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub icons: Icons,
    pub colors: Colors,
    pub object_type: ObjectType,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            icons: Icons {
                cursor: "-> ".to_string(),
                task_new: "[ ]".to_string(),
                task_in_progress: "[*]".to_string(),
                task_hold: "[-]".to_string(),
                task_done: "[+]".to_string(),
                topic_icon_left: "".to_string(),
                topic_icon_right: "".to_string(),
                sub_task_middle: "|-".to_string(),
                sub_task_end: "|-".to_string(),
            },
            colors: Colors {
                bat_color_sheme: "base16-256".to_string(),
                header_color: "#b4befe".to_string(),
                footer_color: "#a6e3a1".to_string(),
                border_color: "#b4befe".to_string(),
                task_topic_color_new_fg: "#b4befe".to_string(),
                task_topic_color_in_progress_fg: "#b4befe".to_string(),
                task_topic_color_hold_fg: "#b4befe".to_string(),
                task_topic_color_done_fg: "#b4befe".to_string(),
                task_topic_color_new_bg: "#000000".to_string(),
                task_topic_color_in_progress_bg: "#000000".to_string(),
                task_topic_color_hold_bg: "#000000".to_string(),
                task_topic_color_done_bg: "#000000".to_string(),
                task_text_color: "#cdd6f4".to_string(),
                task_text_color_done: "#cdd6f4".to_string(),
                task_date_color: "#7f849c".to_string(),
                task_duration_color: "#7f849c".to_string(),
                icon_new_color: "#cdd6f4".to_string(),
                icon_progress_color: "#cdd6f4".to_string(),
                icon_hold_color: "#cdd6f4".to_string(),
                icon_done_color: "#cdd6f4".to_string(),
            },
            object_type: ObjectType {
                border_type: "single".to_string(),
            },
        }
    }
}

impl AppConfig {
    fn read_config() -> Result<Config, ConfigError> {
        let mut cfg = Config::default();
        let path = format!("{}/.config/todo/config.yaml", AppConfig::home_dir());
        cfg.merge(File::with_name(&path))?;
        Ok(cfg)
    }

    pub fn load_config() -> AppConfig {
        let configuration = AppConfig::read_config(); //.expect("Failed to load the configuration.");
        match configuration {
            Ok(file) => file.try_into().expect("Error parsing config."),
            Err(_) => AppConfig::default(),
        }
    }
}

impl FileSystem for AppConfig {}
