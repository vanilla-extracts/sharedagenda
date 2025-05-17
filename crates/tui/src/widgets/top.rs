use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget, Wrap},
};

pub struct Top<'a> {
    pub token: &'a str,
    pub api_link: &'a str,
}

impl Widget for Top<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = Block::bordered()
            .title("SharedAgenda TUI")
            .fg(Color::Yellow)
            .title_alignment(Alignment::Center);
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
                Span::styled(self.api_link, Style::default().fg(Color::Blue).bold()),
            ]),
        ])
        .block(title)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

        paragraph.render(area, buf);
    }
}
