use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap}, Frame
};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: & mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "KBrah!", Style::default().fg(Color::Green),
        ))
        .centered()
        .block(title_block);

    frame.render_widget(title, chunks[0]);

    let body_block = Block::default()
        .title("  PLAYGROUND  ")
        .borders(Borders::ALL)
        .padding(ratatui::widgets::Padding { left: 0, right: 0,
        top: (chunks[1].height / 2) - 2,
        bottom: 0 })
        .border_type(BorderType::Rounded)
        .style(Style::default());


    let mut rest: Vec<char> = app.current_text.chars().collect();
    let mut cursor = ' ';
    if app.current_text.len() != 0 {
        cursor = rest.remove(0);
    } 
    let rest_str = rest.clone().into_iter()
        .map(|i| i.to_string())
        .collect::<String>();
    let style = match app.current_screen {
        CurrentScreen::Editing => {
            Style::default().fg(Color::Black).bg(Color::LightCyan)}, 
        _ => { Style::default().fg(Color::Black).bg(Color::White)}, };
    let cursor_span = if !(app.wrong) {
        Span::styled(cursor.to_string(), style) } else {
            Span::styled(cursor.to_string(),
            Style::default().fg(Color::Black).bg(Color::LightRed))
        };

    let mut text = vec![cursor_span.clone(),
    Span::styled(rest_str, Style::default().fg(Color::White))];

    if app.key_input.len() != 0 {
        let input = app.key_input.clone();
        let mut the_rest: Vec<char> = app.current_text.chars().collect();
        if the_rest.len() != 0 { the_rest.remove(0); } 
        let new_str = the_rest.clone().into_iter()
            .map(|i| i.to_string())
            .collect::<String>();

        text = vec![
            Span::styled(input,
                Style::default().fg(Color::LightGreen)),
                cursor_span,
                Span::styled(new_str,
                    Style::default().fg(Color::White)),
        ];
    };

    let rnum_str = app.rights.clone().into_iter()
        .map(|i| i.to_string())
        .collect::<String>();
    let lnum_str = app.lefts.clone().into_iter()
        .map(|i| i.to_string())
        .collect::<String>();
    let text_thingy = Paragraph::new(vec![
        Line::from(rnum_str.clone()).centered().fg(Color::LightYellow),
        Line::from(text).centered(),
        Line::from(lnum_str.clone()).centered().fg(Color::LightYellow),
    ]).block(body_block);
    frame.render_widget(text_thingy, chunks[1]);

    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => {
            Span::styled("  --NORMAL--  ",
                Style::default().fg(Color::DarkGray))},
            CurrentScreen::Editing => Span::styled("  --INSERT--  ",
                Style::default().fg(Color::LightRed)),
            CurrentScreen::Exiting => Span::styled("NOPE",
                Style::default().fg(Color::LightRed)),
            CurrentScreen::Stats => Span::styled("  --PAUSED--  ",
                Style::default().fg(Color::LightRed)),
        }
    .to_owned(),
    // A white divider bar to separate the two sections
    Span::styled(" | ", Style::default().fg(Color::White)),
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    crate::app::CurrentlyEditing::Key => {
                        let written: Vec<&str> = app.key_input.split(' ').collect();
                        let words: Vec<&str> = app.original_text.split(' ').collect();
                        Span::styled(format!("  {} / {}  ", written.len() - 1, words.len()),
                            Style::default().fg(Color::Green))
                    },
                    crate::app::CurrentlyEditing::Value => {
                        Span::styled("Editing Json Value",
                            Style::default().fg(Color::LightGreen))
                    },
                }
            } else {
                let words: Vec<&str> = app.original_text.split(' ').collect();
                Span::styled(format!("  0 / {}  ", words.len()),
                Style::default().fg(Color::DarkGray))
            }
        },
        Span::styled(" | ", Style::default().fg(Color::White)),
        Span::styled(format!("  {} % ", app.accuracy),
        Style::default().fg(match app.current_screen {
            CurrentScreen::Editing => Color::LightCyan,
            CurrentScreen::Main => Color::DarkGray,
            CurrentScreen::Stats => Color::DarkGray,
            CurrentScreen::Exiting => Color::DarkGray})),
        Span::styled(" | ", Style::default().fg(Color::White)),
        Span::styled(format!("  wpm: {:?}  ",
                app.wpm.unwrap_or(0)),
        Style::default().fg(match app.current_screen {
            CurrentScreen::Editing => Color::LightCyan,
            CurrentScreen::Main => Color::DarkGray,
            CurrentScreen::Stats => Color::DarkGray,
            CurrentScreen::Exiting => Color::DarkGray})),
        ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "  [k]eybindings  |  [p]rofile  ",
                Style::default().fg(Color::Red),
                ),
            CurrentScreen::Editing => Span::styled(
                "[ESC] Normal / (Tab) to switch boxes/ (enter) to complete",
                Style::default().fg(Color::Red),
                ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
                ),
            CurrentScreen::Stats => Span::styled(
                "Whatever",
                Style::default().fg(Color::Red),
                ),
        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    match &app.current_screen {
        CurrentScreen::Main => {},
        CurrentScreen::Editing => {},
        CurrentScreen::Exiting => {
            // frame.render_widget(Clear, frame.area());
            let popup_block = Block::default()
                .title("Y/N")
                .borders(Borders::NONE)
                .border_type(BorderType::Rounded)
                .style(Style::default().bg(Color::DarkGray));

            let exit_text = Text::styled(
                "Are you sure you want to quit?",
                Style::default().fg(Color::Red),
            );

            let exit_paragraph = Paragraph::new(exit_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });

            let area = centered_rect(60, 25, frame.area());
            frame.render_widget(exit_paragraph, area);
        },

        CurrentScreen::Stats => {
            let popup_block = Block::default()
                .title(" STATS ")
                .title_alignment(ratatui::layout::Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let area = centered_rect(50, 70, frame.area());
            frame.render_widget(Clear, area);
            frame.render_widget(popup_block, area);

            let popup_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(30),
                Constraint::Percentage(40), Constraint::Percentage(30)])
                .margin(1)
                .split(area);

            let row_one_block = Block::default()
                .borders(Borders::NONE)
                .border_type(BorderType::Rounded);

            let row_one_area = popup_chunks[0];
            frame.render_widget(Clear, row_one_area);
            frame.render_widget(row_one_block, row_one_area);

            let row_one_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(25),
                Constraint::Percentage(25), Constraint::Percentage(25),
                Constraint::Percentage(25)])
                .split(row_one_area);

            let wpm_block = Block::default()
                .title(" WPM ")
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Cyan))
                .borders(Borders::ALL);

            let acc_block = Block::default()
                .title(" ACCURACY ")
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Green))
                .borders(Borders::ALL);

            let words_block = Block::default()
                .title(" WORDS ")
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Yellow))
                .borders(Borders::ALL);

            let mis_block = Block::default()
                .title(" MISTAKES ")
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Red))
                .borders(Borders::ALL);

            let wpm_text = Paragraph::new(app.wpm.unwrap()
                .to_string().clone()).block(wpm_block).centered();
            frame.render_widget(wpm_text, row_one_chunks[0]);

            let acc_text = Paragraph::new(app.accuracy
                .to_string().clone()).block(acc_block).centered();
            frame.render_widget(acc_text, row_one_chunks[1]);

            let words: Vec<&str> = app.original_text.split(' ').collect();
            let words_text = Paragraph::new(words.len()
                .to_string().clone()).block(words_block).centered();
            frame.render_widget(words_text, row_one_chunks[2]);

            let mis_text = Paragraph::new(app.mistakes
                .to_string().clone()).block(mis_block).centered();
            frame.render_widget(mis_text, row_one_chunks[3]);

            let row_four_block = Block::default()
                .borders(Borders::NONE)
                .border_type(BorderType::Rounded);

            let row_four_area = popup_chunks[2];
            frame.render_widget(row_four_block, row_four_area);

            let row_four_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(100)])
                .split(row_four_area);
            
            let ok_block = Block::default()
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Cyan))
                .borders(Borders::ALL);

            let ok_text = Paragraph::new("⏎ OK").block(ok_block).centered();
            frame.render_widget(ok_text, row_four_chunks[0]);
        },


    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut vertically to 3 portions
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2), 
            Constraint::Percentage(percent_y), 
            Constraint::Percentage((100 - percent_y) / 2), 
        ])
        .split(r);

    // Cut horizontally to 3 portions
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2), 
            Constraint::Percentage(percent_x), 
            Constraint::Percentage((100 - percent_x) / 2), 
        ])
        .split(popup_layout[1])[1]
}
