use crossterm::event::KeyCode;
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Widget},
};
use tui_textarea::TextArea;

use crate::app::{App, CurrentScreen, TuiWidget};

use super::main::MainWidget;

#[derive(Clone, Debug)]
pub struct ApiUrlWidget<'a> {
    pub text: TextArea<'a>,
}

impl Default for ApiUrlWidget<'_> {
    fn default() -> Self {
        Self {
            text: TextArea::default(),
        }
    }
}

impl TuiWidget for ApiUrlWidget<'_> {
    fn handle_key_event<T: TuiWidget + Default + Clone>(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) {
        self.text.input(key);
        match key.code {
            KeyCode::Esc => {
                ratatui::restore();
                let mut terminal = ratatui::init();
                let res = App::new(MainWidget::default(), CurrentScreen::Main).run(&mut terminal);
                match res {
                    Ok(_) => {}
                    Err(e) => {
                        panic!("Error: {e}")
                    }
                }
                ratatui::restore();
            }
            _ => {}
        }
    }
}

impl Widget for ApiUrlWidget<'_> {
    fn render(mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered()
            .title("API URL")
            .title_alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));

        self.text.set_block(block);
        self.text.set_placeholder_text("http://localhost:8008");

        self.text.render(area, buf);
    }
}
