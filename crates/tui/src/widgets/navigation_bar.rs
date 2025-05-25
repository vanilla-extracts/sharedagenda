use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

pub struct NavigationBar<'a> {
    instructions: Line<'a>,
}

impl Default for NavigationBar<'_> {
    fn default() -> Self {
        Self {
            instructions: Line::from(vec![
                "↑ ".blue().bold(),
                "← ".blue().bold(),
                " Up ".white(),
                "↓ ".blue().bold(),
                "→ ".blue().bold(),
                " Down ".white(),
                " ESC ".blue().bold(),
                " Quit ".white(),
                " ENT ".blue().bold(),
                " Execute ".white(),
            ]),
        }
    }
}

impl Widget for NavigationBar<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered()
            .style(Style::default().fg(Color::Yellow))
            .title_bottom("Navigation Bar")
            .title_alignment(Alignment::Center);

        let footer = Paragraph::new(self.instructions).block(block).centered();

        footer.render(area, buf);
    }
}
