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
}

#[derive(Debug, Deserialize)]
pub struct ObjectType {
    pub border_type: String,
    pub selected_style_reversed: bool,
}

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub selected_line_color: String,
    pub header_color: String,
    pub footer_color: String,
    pub border_color: String,
    pub task_status_color_new: String,
    pub task_status_color_progress: String,
    pub task_status_color_hold: String,
    pub task_status_color_done: String,
    pub task_topic_color_new: String,
    pub task_topic_color_in_progress: String,
    pub task_topic_color_hold: String,
    pub task_topic_color_done: String,
    pub task_text_color: String,
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
            },
            colors: Colors {
                selected_line_color: "#cba6f7".to_string(),
                header_color: "#b4befe".to_string(),
                footer_color: "#a6e3a1".to_string(),
                border_color: "#b4befe".to_string(),
                task_status_color_new: "#cdd6f4".to_string(),
                task_status_color_progress: "#a6e3a1".to_string(),
                task_status_color_hold: "#fab387".to_string(),
                task_status_color_done: "#7f849c".to_string(),
                task_topic_color_new: "#b4befe".to_string(),
                task_topic_color_in_progress: "#b4befe".to_string(),
                task_topic_color_hold: "#b4befe".to_string(),
                task_topic_color_done: "#b4befe".to_string(),
                task_text_color: "#cdd6f4".to_string(),
                task_date_color: "#7f849c".to_string(),
                task_duration_color: "#7f849c".to_string(),
                icon_new_color: "#cdd6f4".to_string(),
                icon_progress_color: "#cdd6f4".to_string(),
                icon_hold_color: "#cdd6f4".to_string(),
                icon_done_color: "#cdd6f4".to_string(),
            },
            object_type: ObjectType {
                border_type: "single".to_string(),
                selected_style_reversed: false,
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
