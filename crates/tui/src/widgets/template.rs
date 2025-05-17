use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::Widget,
};

use super::{navigation_bar::NavigationBar, top::Top};

pub struct TemplateWidget<'a, T: Widget> {
    pub token: &'a str,
    pub api_link: &'a str,
    pub middle: T,
}
/*
layout:
--------




*/
impl<T: Widget> Widget for TemplateWidget<'_, T> {
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
        let top_bar = Top {
            token: self.token,
            api_link: self.api_link,
        };
        top_bar.render(chunks[0], buf);
        self.middle.render(chunks[1], buf);
        let navigation_bar = NavigationBar {};
        navigation_bar.render(chunks[2], buf);
    }
}
