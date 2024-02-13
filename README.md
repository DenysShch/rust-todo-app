# Rust Terminal Todo App

<div align="center">
<br>
<br>
  <div>
    <img src="assets/rust-todo-logo-nobg.png" width="230" alt="Warp">
  </div>
<br>
</div>

## Overview

![demo](assets/sc1.png)
![demo](assets/sc2.png)

Welcome to the Rust Terminal Todo App! This is a simple command-line todo application with a user-friendly interface written in Rust. It is inspired by projects like [ gitui ](https://github.com/extrawurst/gitui) and [ lazygit ](https://github.com/jesseduffield/lazygit), providing a convenient way to manage your tasks right from the terminal.

## Features

- **Simple Interface**: The application offers an easy-to-use interface for managing your todo list within the terminal.

- **Add, Remove, and Edit Tasks**: Quickly add new tasks, mark tasks as completed, and edit existing tasks directly from the command line.

- **Interactive UI**: Navigate through your tasks using an interactive UI that makes task management a breeze.

- **Filter**: by status.

- **Sub task** functionality.

- **Comments**: Add, Remove, and Edit comments.

- **Code highlights**

## Key-maps

| Keys        | Description        |
| ----------- | ------------------ |
| esc         | close popup        |
| Tab         | change popup window|
| C-s         | save               |
| n           | new task           |
| s           | new sub task       |
| c           | change status      |
| e           | edit task          |
| E           | edit last comment  |
| a           | add comment        |
| D           | delete task        |
| A           | delete last comment|
| h           | horizontal view    |
| v           | vertical view      |
| ,           | scroll down        |
| .           | scroll up          |

## Installation

To use the Rust Terminal Todo App, you need to have Rust installed on your machine. Once Rust is installed, you can clone this repository and build the application using the following commands:

```bash
git clone https://github.com/your-username/terminal-todo-rust.git
cd terminal-todo-rust
cargo build --release
cd bin
./install
```

This will generate the executable file in the `target/release/` directory.

## Usage

Run the compiled executable to start the application:

```bash
./target/release/rust-todo
```

Use the arrow keys to navigate, and press Enter to interact with the tasks. The application will guide you through adding, removing, and editing tasks.

## Tmux Integration

```bash
#.tmux.config

bind -n 'M-t' neww -c "#{pane_current_path}" "~/.local/bin/rust-todo"
```

Press `Alt + t` to open the 'Todo App,' and press `q` to return to the previous place.

## Configuration

The Rust Terminal Todo App supports configuration through a `$HOME/.config/todo/config.yaml` file. You can customize various settings, such as the appearance and behavior of the application, by modifying this file.
Default configuration you can find in `config` folder.

My personal configuration inspired by `Kanagawa Dragon`:

```yaml
#kanagawa-dragon
icons:
  cursor: '⤙ '
  task_new: ''
  task_in_progress: ''
  task_hold: ''
  task_done: ''
  topic_icon_left: '' #    
  topic_icon_right: '' #   
  sub_task_middle: '├─'
  sub_task_end: '╰─'
colors:
  bat_color_sheme: 'base16-256'
  header_color: '#717C7C'
  footer_color: '#727169'
  border_color: '#49443C'
  task_topic_color_new_fg: '#C8C093'
  task_topic_color_in_progress_fg: '#1F1F28'
  task_topic_color_hold_fg: '#1F1F28'
  task_topic_color_done_fg: '#C8C093'
  task_topic_color_new_bg: '#49443C'
  task_topic_color_in_progress_bg: '#76946A'
  task_topic_color_hold_bg: '#DCA561'
  task_topic_color_done_bg: '#2B3328'
  task_text_color: '#DCD7BA'
  task_text_color_done: '#49443C'
  task_date_color: '#727169'
  task_duration_color: '#7f849c'
  icon_new_color: '#49443C'
  icon_progress_color: '#76946A'
  icon_hold_color: '#DCA561'
  icon_done_color: '#2B3328'
object_type:
  border_type: 'rounded' # rounded, double, thick, quadrant
```

Feel free to experiment with the configuration options to tailor the application to your preferences.

Your feedback and suggestions are welcome! If you have ideas for new features or improvements, please open an issue on the GitHub repository.

Thank you for using the Rust Terminal Todo App!

Happy task managing!
