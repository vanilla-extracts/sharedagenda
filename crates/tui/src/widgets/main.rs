use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget, Wrap},
};

pub struct MainWidget<'a> {
    pub token: &'a str,
    pub api_link: &'a str,
}
/*
layout:
--------




*/
impl Widget for MainWidget<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),
                Constraint::Min(1),
                Constraint::Length(2),
            ])
            .split(area);
        let title = Block::bordered()
            .title("SharedAgenda TUI")
            .fg(Color::Yellow)
            .title_alignment(Alignment::Center)
            .bold();
        let logged_in = if self.token.trim().is_empty() {
            "You are not logged in.".to_string()
        } else {
            format!("Hello {}.", self.token)
        };
        let paragraph = Paragraph::new(vec![
            Line::from(logged_in),
            Line::from(""),
            Line::from_iter([
                "API Link: ".into(),
                Span::styled(self.api_link, Style::default().fg(Color::Red).bold()),
            ]),
        ])
        .block(title)
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

        paragraph.render(chunks[0], buf);
    }
}
