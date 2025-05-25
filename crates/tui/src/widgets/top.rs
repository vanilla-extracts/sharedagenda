use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::{API_URL, TOKEN};

pub struct Top {
    pub token: String,
    pub api_link: String,
}

impl Default for Top {
    fn default() -> Self {
        Self {
            token: TOKEN.lock().unwrap().to_string(),
            api_link: API_URL.lock().unwrap().to_string(),
        }
    }
}

impl Widget for Top {
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
