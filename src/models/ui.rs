use super::{
    app::{App, Filter, InputArea, InputMode, PageLayout},
    config::AppConfig,
    formatter::Formatter,
    task::{Status, Task},
};
use ratatui::{prelude::*, widgets::*};
use std::{iter::once, str::FromStr};
use tui_textarea::TextArea;

pub fn ui(
    f: &mut Frame,
    app: &mut App,
    text_area: &mut [TextArea; 4],
    conf: &AppConfig,
    formatter: &Formatter,
) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(f.size());

    let selected_style = get_selected_style();
    let border_type = get_border_type(conf);
    let border_style = Style::new().fg(Color::from_str(conf.colors.border_color.as_str()).unwrap());
    let text_style =
        Style::new().fg(Color::from_str(conf.colors.task_text_color.as_str()).unwrap());

    let info_text = vec![text::Line::from(
        " <down/up>|<j/k>:move, <q>:quit, ?:keybidings",
    )];
    let info_paragraph =
        Paragraph::new(info_text).fg(Color::from_str(&conf.colors.footer_color.as_str()).unwrap());

    f.render_widget(info_paragraph, main_layout[2]);

    let inner_layout = {
        match app.layout {
            PageLayout::Vertical => Layout::new(
                Direction::Horizontal,
                [Constraint::Percentage(50), Constraint::Percentage(50)],
            )
            .split(main_layout[1]),
            PageLayout::Horizontal => Layout::new(
                Direction::Vertical,
                [Constraint::Percentage(50), Constraint::Percentage(50)],
            )
            .split(main_layout[1]),
        }
    };

    let items: Vec<ListItem> = app
        .items
        .clone()
        .iter()
        .filter(|f| f.display)
        // .map(|x| (x.child_list.len(), x.is_sub_task))
        .map(|x| (display_child_list(&x.child_list, &app.items), x.is_sub_task))
        .into_iter()
        .scan(0, |state, (x, y)| {
            if !y {
                *state = x + 1;
                return Some(0);
            } else {
                *state = *state - (x + 1);
                if *state == 1 {
                    *state = 0;
                    return Some(2);
                }
                return Some(1);
            }
        })
        .zip(app.items.iter().filter(|f| f.display))
        .map(|(x, t)| {
            let mut topic = topic_formatter(x, t, conf);
            let lines = {
                let styled = Span::styled(t.name.as_str(), text_style);
                if t.status == Status::Done {
                    styled
                        .crossed_out()
                        .fg(Color::from_str(conf.colors.task_text_color_done.as_str()).unwrap())
                } else {
                    styled
                }
            };
            topic.extend(vec![lines]);
            ListItem::new(Line::from(topic))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(border_type)
                .border_style(border_style)
                .title("Tasks"),
        )
        .highlight_style(selected_style)
        .highlight_symbol(conf.icons.cursor.as_str());

    f.render_stateful_widget(list, inner_layout[0], &mut app.state);

    let details = match app.state.selected() {
        Some(i) => { let paragraph = {
           let style = Style::default().fg(Color::from_str(conf.colors.border_color.as_str()).unwrap());
            let wrapper = "─".repeat(inner_layout[1].width as usize - 2);
            let task = app.items.get(i).unwrap();
            let child_list = &app.sub_items;
            let task_status_last_change = task.status_change_date.clone().unwrap_or("-//-".to_string());
            let task_duration = task.duration.clone().unwrap_or("-//-".to_string());
            let formatter_name = formatter.format(&task.name);
            let formatter_desc = formatter.format(&task.description);
            let mut text_block = Text::raw("");
            let mut info_block = vec![
                Text::from(
                    Line::from(
                        vec![
                            Span::styled("Status: ", Style::default().fg(Color::from_str(conf.colors.header_color.as_str()).unwrap())).bold(),
                            Span::styled(conf.icons.topic_icon_left.to_string(), get_topic_icon_color(task.status.to_string(), &conf)),
                            Span::styled(
                                task.status.to_string(),
                                get_topic_color(task.status.to_string(), &conf),
                            ),
                            Span::styled(conf.icons.topic_icon_right.to_string(), get_topic_icon_color(task.status.to_string(), &conf)),
                        ]
                    )
                ),
                Text::raw(""),
                Text::styled(format!("created:  {}", &task.creation_date), Style::default().fg(Color::from_str(conf.colors.task_date_color.as_str()).unwrap())),
                Text::styled(format!("updated:  {}", &task.status_change_date.clone().unwrap_or("-//-".to_string())), Style::default().fg(Color::from_str(conf.colors.task_date_color.as_str()).unwrap())),
                Text::styled(format!("duration: {}", &task.duration.clone().unwrap_or("-//-".to_string())), Style::default().fg(Color::from_str(conf.colors.task_date_color.as_str()).unwrap())),
                Text::raw(""),
                Text::styled("Name:", Style::default().fg(Color::from_str(conf.colors.header_color.as_str()).unwrap()).bold()),
                formatter_name,
                Text::raw(""),
                Text::styled("Description:", Style::default().fg(Color::from_str(conf.colors.header_color.as_str()).unwrap()).bold()),
                formatter_desc,
                Text::raw(""),
           ];

            let mut comments_block = vec![
                Text::styled("Comments:", Style::default().fg(Color::from_str(conf.colors.header_color.as_str()).unwrap()).bold()),
                Text::styled(wrapper.clone(), Style::default().fg(Color::from_str(conf.colors.border_color.as_str()).unwrap())),
            ];

            let mut comments:Vec<_> = task.comments.iter()
                .map(|x| (x.date.clone(), x.text.clone()))
                .flat_map(|(a, b)| {
                    let c_date = Text::styled(format!("date: {}", a), Style::default().fg(Color::from_str(conf.colors.task_date_color.as_str()).unwrap()));
                    let c_text = formatter.format(&b);
                    let wrapper = Text::styled(wrapper.clone(), Style::default().fg(Color::from_str(conf.colors.border_color.as_str()).unwrap()));
                    once(c_date).chain(once(c_text)).chain(once(wrapper))
                })
                .collect();


            let mut sub_tasks_block = vec![
                Text::styled("Sub tasks:", Style::default().fg(Color::from_str(conf.colors.header_color.as_str()).unwrap()).bold()),
            ];

            let mut sub_tasks:Vec<_> = child_list.iter().map(|x| Text::from(Line::from(
                vec![
                Span::styled(format!(" {} ", match_status(x.status.to_string(), &conf)), get_topic_icon_color(x.status.to_string(), &conf)),
                Span::styled(format!("{}", x.name.to_string()),
                        {
                            let styled = text_style;
                            if x.status == Status::Done {
                                styled.crossed_out()
                                .fg(Color::from_str(conf.colors.task_text_color_done.as_str()).unwrap())
                            } else {
                                styled
                            }
                        },
                    ),
                ]
            ))).chain(vec![Text::raw("")]).collect();

            if sub_tasks.len() > 1 {
                info_block.append(&mut sub_tasks_block);
                info_block.append(&mut sub_tasks);
            }

            if comments.len() > 0 {
                info_block.append(&mut comments_block);
                info_block.append(&mut comments);
            }

            for raw in info_block {
                text_block.extend(raw);
            }

            let mut t = Text::styled(format!(
                "Status: {}\n\nCreation date:    {}\nLast update date: {}\nDuration:         {}\n\nName: {}\nDescription:\n",
                task.status, task.creation_date, task_status_last_change, task_duration, task.name
            ), style);
            t.extend(Text::raw("\n\nComments:"));
            t.extend(Text::styled(wrapper.clone(), style));
            Paragraph::new(text_block).wrap(Wrap { trim: false })
        }; paragraph }
        None => Paragraph::default(),
    }
    .block(Block::default()
                .borders(Borders::ALL)
                .border_type(border_type)
                .border_style(border_style)
                .title("Details"))
                .scroll((app.scroll as u16, 0));

    f.render_widget(details, inner_layout[1]);

    match app.input_mode {
        InputMode::Normal => (),
        InputMode::Comment | InputMode::CommentEdit => {
            let layout = centered_rect(50, 30, f.size());
            activate(
                &mut text_area[3],
                "Comment".to_string(),
                border_type,
                border_style,
                text_style,
            );
            let widget_comment = text_area[3].widget();
            f.render_widget(Clear, layout[2][0]);
            f.render_widget(widget_comment, layout[2][0]);
        }
        InputMode::SubTask | InputMode::SubTaskModify => {
            let layout = centered_rect(50, 30, f.size());
            match app.input_area {
                InputArea::Task => {
                    activate(
                        &mut text_area[1],
                        "Sub Task".to_string(),
                        border_type,
                        border_style,
                        text_style,
                    );
                    inactivate(
                        &mut text_area[2],
                        "Description".to_string(),
                        border_type,
                        border_style,
                        text_style,
                    );
                }
                InputArea::Description => {
                    inactivate(
                        &mut text_area[1],
                        "Sup Task".to_string(),
                        border_type,
                        border_style,
                        text_style,
                    );
                    activate(
                        &mut text_area[2],
                        "Description".to_string(),
                        border_type,
                        border_style,
                        text_style,
                    );
                }
                _ => (),
            }

            let widget_task = text_area[1].widget();
            let widget_desc = text_area[2].widget();

            f.render_widget(Clear, layout[1][0]);
            f.render_widget(widget_task, layout[1][0]);
            f.render_widget(Clear, layout[2][0]);
            f.render_widget(widget_desc, layout[2][0]);
        }
        InputMode::Help => {
            let info_block_popup = Block::default()
                .title("Keybidings")
                .borders(Borders::ALL)
                .border_style(border_style)
                .border_type(border_type);

            let info_layout_popup = info_rect(40, 50, f.size());
            let text = vec![
                text::Line::from("<esc>: \t\t close popup"),
                text::Line::from("<n>: \t\t new task"),
                text::Line::from("<s>: \t\t new sub task"),
                text::Line::from("<c>: \t\t change status"),
                text::Line::from("<e>: \t\t edit task"),
                text::Line::from("<E>: \t\t edit last comment"),
                text::Line::from("<a>: \t\t add comment"),
                text::Line::from("<D>: \t\t delete task"),
                text::Line::from("<A>: \t\t delete last comment"),
                text::Line::from("<h>: \t\t horizontal view"),
                text::Line::from("<v>: \t\t vertical view"),
                text::Line::from("<Tab>: \t\t change popup window"),
                text::Line::from("<C-s>: \t\t save"),
                text::Line::from("<,>: \t\t scroll down"),
                text::Line::from("<.>: \t\t scroll up"),
            ];
            let paragraph = Paragraph::new(text)
                .style(text_style)
                .block(info_block_popup);
            f.render_widget(Clear, info_layout_popup); //this clears out the background
            f.render_widget(paragraph, info_layout_popup);
        }
        InputMode::FilterMode => {
            let filter_block_popup = Block::default()
                .title("Filter")
                .borders(Borders::ALL)
                .border_style(border_style)
                .border_type(border_type);

            let filter_layout_popup = info_rect(15, 15, f.size());
            let text = self::filter_popup(app.filter, &conf);
            let paragraph = Paragraph::new(text)
                .style(text_style)
                .block(filter_block_popup);
            f.render_widget(Clear, filter_layout_popup); //this clears out the background
            f.render_widget(paragraph, filter_layout_popup);
        }
        _ => {
            let layout = centered_rect(50, 30, f.size());
            match app.input_area {
                InputArea::Topic => {
                    activate(
                        &mut text_area[0],
                        "Topic".to_string(),
                        border_type,
                        border_style,
                        text_style,
                    );
                    inactivate(
                        &mut text_area[1],
                        "Task".to_string(),
                        border_type,
                        border_style,
                        text_style,
                    );
                    inactivate(
                        &mut text_area[2],
                        "Description".to_string(),
                        border_type,
                        border_style,
                        text_style,
                    );
                }
                InputArea::Task => {
                    inactivate(
                        &mut text_area[0],
                        "Topic".to_string(),
                        border_type,
                        border_style,
                        text_style,
                    );
                    activate(
                        &mut text_area[1],
                        "Task".to_string(),
                        border_type,
                        border_style,
                        text_style,
                    );
                    inactivate(
                        &mut text_area[2],
                        "Description".to_string(),
                        border_type,
                        border_style,
                        text_style,
                    );
                }
                InputArea::Description => {
                    inactivate(
                        &mut text_area[0],
                        "Topic".to_string(),
                        border_type,
                        border_style,
                        text_style,
                    );
                    inactivate(
                        &mut text_area[1],
                        "Task".to_string(),
                        border_type,
                        border_style,
                        text_style,
                    );
                    activate(
                        &mut text_area[2],
                        "Description".to_string(),
                        border_type,
                        border_style,
                        text_style,
                    );
                }
                _ => (),
            }

            let widget_topic = text_area[0].widget();
            let widget_task = text_area[1].widget();
            let widget_desc = text_area[2].widget();

            f.render_widget(Clear, layout[0][0]);
            f.render_widget(widget_topic, layout[0][0]);
            f.render_widget(Clear, layout[1][0]);
            f.render_widget(widget_task, layout[1][0]);
            f.render_widget(Clear, layout[2][0]);
            f.render_widget(widget_desc, layout[2][0]);
        }
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
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(5),
        ])
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

fn inactivate(
    textarea: &mut TextArea<'_>,
    title: String,
    b_type: BorderType,
    b_style: Style,
    t_syle: Style,
) {
    textarea.set_cursor_line_style(t_syle);
    textarea.set_cursor_style(t_syle);
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(b_type)
            .border_style(b_style)
            .style(Style::default().fg(Color::DarkGray))
            .title(title),
    );
}

fn activate(
    textarea: &mut TextArea<'_>,
    title: String,
    b_type: BorderType,
    b_style: Style,
    t_syle: Style,
) {
    textarea.set_cursor_line_style(t_syle);
    textarea.set_cursor_style(t_syle.add_modifier(Modifier::REVERSED));
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(b_type)
            .border_style(b_style)
            .style(Style::default())
            .title(title),
    );
}

fn get_selected_style() -> Style {
    return Style::default().add_modifier(Modifier::BOLD);
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

fn get_topic_color(s: String, conf: &AppConfig) -> Style {
    if s == "NEW" {
        return Style::new()
            .fg(Color::from_str(conf.colors.task_topic_color_new_fg.as_str()).unwrap())
            .bg(Color::from_str(conf.colors.task_topic_color_new_bg.as_str()).unwrap())
            .bold();
    } else if s == "IN PROGRESS" {
        return Style::new()
            .fg(Color::from_str(conf.colors.task_topic_color_in_progress_fg.as_str()).unwrap())
            .bg(Color::from_str(conf.colors.task_topic_color_in_progress_bg.as_str()).unwrap())
            .bold();
    } else if s == "HOLD" {
        return Style::new()
            .fg(Color::from_str(conf.colors.task_topic_color_hold_fg.as_str()).unwrap())
            .bg(Color::from_str(conf.colors.task_topic_color_hold_bg.as_str()).unwrap())
            .bold();
    } else {
        return Style::new()
            .fg(Color::from_str(conf.colors.task_topic_color_done_fg.as_str()).unwrap())
            .bg(Color::from_str(conf.colors.task_topic_color_done_bg.as_str()).unwrap())
            .bold();
    }
}

fn get_topic_icon_color(s: String, conf: &AppConfig) -> Style {
    if s == "NEW" {
        return Style::new()
            .fg(Color::from_str(conf.colors.task_topic_color_new_bg.as_str()).unwrap());
    } else if s == "IN PROGRESS" {
        return Style::new()
            .fg(Color::from_str(conf.colors.task_topic_color_in_progress_bg.as_str()).unwrap());
    } else if s == "HOLD" {
        return Style::new()
            .fg(Color::from_str(conf.colors.task_topic_color_hold_bg.as_str()).unwrap());
    } else {
        return Style::new()
            .fg(Color::from_str(conf.colors.task_topic_color_done_bg.as_str()).unwrap());
    }
}

fn filter_popup<'a>(f: Filter, conf: &AppConfig) -> Vec<Line<'a>> {
    Filter::iterator()
        .map(|x| {
            if x == f {
                text::Line::from(Span::styled(
                    format!("[x] {}", x.to_string()),
                    Style::new()
                        .fg(Color::from_str(&conf.colors.icon_hold_color.as_str()).unwrap())
                        .bold(),
                ))
            } else {
                text::Line::from(Span::styled(format!("[ ] {}", x.to_string()), Style::new()))
            }
        })
        .collect()
}

fn topic_formatter<'a>(item_code: usize, task: &Task, conf: &AppConfig) -> Vec<Span<'a>> {
    if item_code == 0 {
        vec![
            Span::styled(
                conf.icons.topic_icon_left.to_string(),
                get_topic_icon_color(task.status.to_string(), &conf),
            ),
            Span::styled(
                task.topic.to_string(),
                get_topic_color(task.status.to_string(), &conf),
            ),
            Span::styled(
                conf.icons.topic_icon_right.to_string(),
                get_topic_icon_color(task.status.to_string(), &conf),
            ),
            Span::from(" "),
        ]
    } else if item_code == 1 {
        return vec![
            Span::from(" "),
            Span::styled(
                conf.icons.sub_task_middle.to_string(),
                Style::new().fg(Color::from_str(conf.colors.border_color.as_str()).unwrap()),
            ),
            Span::styled(
                match_status(task.status.to_string(), conf),
                get_icon_status_style(task.status.to_string(), conf),
            ),
            Span::from(" "),
        ];
    } else {
        return vec![
            Span::from(" "),
            Span::styled(
                conf.icons.sub_task_end.to_string(),
                Style::new().fg(Color::from_str(conf.colors.border_color.as_str()).unwrap()),
            ),
            Span::styled(
                match_status(task.status.to_string(), conf),
                get_icon_status_style(task.status.to_string(), conf),
            ),
            Span::from(" "),
        ];
    }
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

fn display_child_list(c: &Vec<i64>, items: &Vec<Task>) -> usize {
    let mut counter = 0;
    for c_id in c {
        for item in items.iter().filter(|f| f.display) {
            if *c_id == item.id {
                counter += 1;
                continue;
            }
        }
    }
    counter
}
