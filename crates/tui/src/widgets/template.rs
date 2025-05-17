use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::Widget,
};

pub struct TemplateWidget<U: Widget, V: Widget, T: Widget> {
    pub top_bar: U,
    pub middle: T,
    pub navigation_bar: V,
}
/*
layout:
--------




*/
impl<U: Widget, V: Widget, T: Widget> Widget for TemplateWidget<U, V, T> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(area);
        self.top_bar.render(chunks[0], buf);

        self.middle.render(chunks[1], buf);

        self.navigation_bar.render(chunks[2], buf);
    }
}
