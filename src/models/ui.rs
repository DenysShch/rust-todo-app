use super::{app::App, app::EnterMode, app::InputMode, config::AppConfig};
use std::str::FromStr;

use ratatui::{prelude::*, widgets::*};

pub fn ui(f: &mut Frame, app: &mut App, conf: &AppConfig) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(98), Constraint::Percentage(2)])
        .split(f.size());

    let selected_style = get_selected_style(&conf);
    let normal_style = Style::default();

    let border_type = get_border_type(conf);

    let header_cells = [
        "  ",
        "TOPIC",
        "TASK",
        "STATUS",
        "START-DATE",
        "END-DATE",
        "DURATION",
    ]
    .iter()
    .map(|h| {
        Cell::from(*h)
            .style(Style::default().fg(Color::from_str(conf.colors.header_color.as_str()).unwrap()))
    });

    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1)
        .bold();

    let rows = app.items.iter().map(|task| {
        let height = task.message.chars().filter(|c| *c == '\n').count() + 1;

        let cells = vec![
            Cell::from(match_status(task.status.to_string(), &conf))
                .style(get_icon_status_style(task.status.to_string(), &conf)),
            Cell::from(task.topic.as_str()).style(get_topic_color(task.status.to_string(), &conf)),
            Cell::from(task.message.as_str())
                .style(get_message_style(task.status.to_string(), &conf)),
            Cell::from(task.status.to_string())
                .style(get_status_style(task.status.to_string(), &conf)),
            Cell::from(task.create_time.as_str()).style(
                Style::default().fg(Color::from_str(conf.colors.task_date_color.as_str()).unwrap()),
            ),
            Cell::from(match task.done_time.as_ref() {
                Some(s) => s.to_string(),
                None => "   -//-   ".to_string(),
            })
            .style(
                Style::default().fg(Color::from_str(conf.colors.task_date_color.as_str()).unwrap()),
            ),
            Cell::from(match task.duration.as_ref() {
                Some(s) => s.as_str(),
                None => "    -//-   ",
            })
            .style(
                Style::default()
                    .fg(Color::from_str(conf.colors.task_duration_color.as_str()).unwrap()),
            ),
        ]
        .into_iter();
        Row::new(cells).height(height as u16)
    });
    let t = Table::new(rows)
        .widths(&[
            Constraint::Length(5),
            Constraint::Length(15),
            Constraint::Percentage(50),
            Constraint::Length(11),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Percentage(10),
        ])
        .header(header)
        .highlight_style(selected_style)
        .column_spacing(2)
        .highlight_symbol(conf.icons.cursor.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(
                    Style::new().fg(Color::from_str(conf.colors.border_color.as_str()).unwrap()),
                )
                .border_type(border_type)
                .title("TODO")
                .padding(Padding::new(5, 5, 1, 0)),
        );
    f.render_stateful_widget(t, rects[0], &mut app.state);

    let text = vec![text::Line::from(
        " <down/up>: move, <q>: quit, ?: keybidings",
    )];
    let paragraph =
        Paragraph::new(text).fg(Color::from_str(&conf.colors.footer_color.as_str()).unwrap());
    f.render_widget(paragraph, rects[1]);

    if app.insert_popup {
        let layout = centered_rect(50, 20, f.size());

        let block_topic = Block::default()
            .title("Enter topic name:")
            .borders(Borders::ALL)
            .border_type(border_type);

        let input_topic = Paragraph::new(app.insert_app.topic.as_str())
            .style(match app.insert_app.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => {
                    Style::default().fg(Color::from_str(conf.colors.border_color.as_str()).unwrap())
                }
            })
            .block(block_topic);

        let block_task = Block::default()
            .title("Enter task name:")
            .borders(Borders::ALL)
            .border_type(border_type);

        let input_task = Paragraph::new(app.insert_app.input.as_str())
            .style(match app.insert_app.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => {
                    Style::default().fg(Color::from_str(conf.colors.border_color.as_str()).unwrap())
                }
            })
            .block(block_task);

        f.render_widget(Clear, layout[0][0]);
        f.render_widget(Clear, layout[1][0]);
        f.render_widget(input_topic, layout[0][0]);
        f.render_widget(input_task, layout[1][0]);

        match app.insert_app.input_mode {
            InputMode::Normal => {}
            InputMode::Editing => match app.insert_app.enter_mode {
                EnterMode::Input => f.set_cursor(
                    layout[1][0].x + app.insert_app.return_caret() as u16 + 1,
                    layout[1][0].y + app.insert_app.cursor_position_y as u16 + 1,
                ),
                EnterMode::Topic => f.set_cursor(
                    layout[0][0].x + app.insert_app.return_caret() as u16 + 1,
                    layout[0][0].y + app.insert_app.cursor_position_y as u16 + 1,
                ),
            },
        }
    }

    if app.info_popup {
        let info_block = Block::default()
            .title("Keybidings")
            .borders(Borders::ALL)
            .border_style(Style::new().blue())
            .border_type(border_type);

        let info_layout = info_rect(15, 20, f.size());
        let text = vec![
            text::Line::from("<esc>: \t\t close popup"),
            text::Line::from("<n>: \t\t new task"),
            text::Line::from("<e>: \t\t edit task"),
            text::Line::from("<D>: \t\t delete task"),
            text::Line::from("<s>: \t\t change status"),
            text::Line::from("<|>: \t\t add new line"),
            text::Line::from("<1>: \t\t sort by topic"),
            text::Line::from("<2>: \t\t sort by status"),
            text::Line::from("<3>: \t\t sort by creation"),
            text::Line::from("<4>: \t\t sort by complition"),
        ];
        let paragraph = Paragraph::new(text)
            .style(Style::default())
            .fg(Color::from_str(conf.colors.border_color.as_str()).unwrap())
            .block(info_block);
        f.render_widget(Clear, info_layout); //this clears out the background
        f.render_widget(paragraph, info_layout);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Vec<Vec<Rect>> {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1]);

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20); 2])
        .split(layout[1])
        .iter()
        .map(|&area| {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70)])
                .split(area)
                .to_vec()
        })
        .collect()
}

fn info_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn get_border_type(conf: &AppConfig) -> BorderType {
    if conf.object_type.border_type == "rounded" {
        return BorderType::Rounded;
    } else if conf.object_type.border_type == "double" {
        return BorderType::Double;
    } else if conf.object_type.border_type == "thick" {
        return BorderType::Thick;
    } else if conf.object_type.border_type == "quadrant" {
        return BorderType::QuadrantOutside;
    } else {
        return BorderType::Plain;
    }
}

fn match_status(s: String, conf: &AppConfig) -> String {
    if s == "NEW" {
        return conf.icons.task_new.clone();
    } else if s == "IN PROGRESS" {
        return conf.icons.task_in_progress.clone();
    } else if s == "HOLD" {
        return conf.icons.task_hold.clone();
    } else {
        return conf.icons.task_done.clone();
    }
}

fn get_status_style(s: String, conf: &AppConfig) -> Style {
    if s == "NEW" {
        return Style::new()
            .fg(Color::from_str(conf.colors.task_status_color_new.as_str()).unwrap());
    } else if s == "IN PROGRESS" {
        return Style::new()
            .fg(Color::from_str(conf.colors.task_status_color_progress.as_str()).unwrap());
    } else if s == "HOLD" {
        return Style::new()
            .fg(Color::from_str(conf.colors.task_status_color_hold.as_str()).unwrap());
    } else {
        return Style::new()
            .fg(Color::from_str(conf.colors.task_status_color_done.as_str()).unwrap());
    }
}

fn get_message_style(s: String, conf: &AppConfig) -> Style {
    if s == "DONE" {
        return Style::default()
            .fg(Color::from_str(conf.colors.task_status_color_done.as_str()).unwrap());
    } else if s == "HOLD" {
        return Style::default()
            .fg(Color::from_str(conf.colors.task_status_color_hold.as_str()).unwrap());
    } else if s == "IN PROGRESS" {
        return Style::default()
            .fg(Color::from_str(conf.colors.task_status_color_progress.as_str()).unwrap());
    } else {
        return Style::default().fg(Color::from_str(conf.colors.task_text_color.as_str()).unwrap());
    }
}

fn get_icon_status_style(s: String, conf: &AppConfig) -> Style {
    if s == "NEW" {
        return Style::new().fg(Color::from_str(conf.colors.icon_new_color.as_str()).unwrap());
    } else if s == "HOLD" {
        return Style::new().fg(Color::from_str(conf.colors.icon_hold_color.as_str()).unwrap());
    } else if s == "IN PROGRESS" {
        return Style::new().fg(Color::from_str(conf.colors.icon_progress_color.as_str()).unwrap());
    } else {
        return Style::new().fg(Color::from_str(conf.colors.icon_done_color.as_str()).unwrap());
    }
}

fn get_selected_style(conf: &AppConfig) -> Style {
    if conf.object_type.selected_style_reversed {
        return Style::default()
            .fg(Color::from_str(conf.colors.selected_line_color.as_str()).unwrap())
            .add_modifier(Modifier::REVERSED);
    } else {
        return Style::default()
            .fg(Color::from_str(conf.colors.selected_line_color.as_str()).unwrap());
    }
}

fn get_topic_color(s: String, conf: &AppConfig) -> Style {
    if s == "NEW" {
        return Style::new()
            .fg(Color::from_str(conf.colors.task_topic_color_new.as_str()).unwrap())
            .bold();
    } else if s == "IN PROGRESS" {
        return Style::new()
            .fg(Color::from_str(conf.colors.task_topic_color_in_progress.as_str()).unwrap())
            .bold();
    } else if s == "HOLD" {
        return Style::new()
            .fg(Color::from_str(conf.colors.task_topic_color_hold.as_str()).unwrap())
            .bold();
    } else {
        return Style::new()
            .fg(Color::from_str(conf.colors.task_topic_color_done.as_str()).unwrap())
            .bold();
    }
}
