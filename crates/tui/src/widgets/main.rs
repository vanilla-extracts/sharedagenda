use ratatui::widgets::{Paragraph, Widget};

pub struct MainWidget {}
/*
layout:
--------




*/
impl Widget for MainWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Paragraph::new("Hello World").render(area, buf);
    }
}
