use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

pub struct NavigationBar {}

impl Widget for NavigationBar {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let instructions = Line::from(vec![
            " ↑ ".blue().bold(),
            " Up ".white(),
            " ↓ ".blue().bold(),
            " Down ".white(),
            " ← ".blue().bold(),
            " Left ".white(),
            " → ".blue().bold(),
            " Right ".white(),
            " q ".blue().bold(),
            " Quit ".white(),
            " ENT ".blue().bold(),
            " Execute ".white(),
        ]);
        let block = Block::bordered()
            .style(Style::default().fg(Color::Yellow))
            .title_bottom("Navigation Bar")
            .title_alignment(Alignment::Center);

        let footer = Paragraph::new(instructions).block(block).centered();

        footer.render(area, buf);
    }
}
