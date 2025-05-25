use common::configuration::loader::{load, write_config};
use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Widget},
};
use tui_textarea::TextArea;

use crate::{API_URL, app::TuiWidget};

use super::main::{MainWidget, switch_screen};

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
                switch_screen::<MainWidget>();
            }
            KeyCode::Enter => {
                let url = self.text.clone().into_lines().join("");
                if url.trim() == "" {
                    return;
                }

                let mut config = load().unwrap_or_default();
                config.api_link = url.clone();
                *API_URL.lock().unwrap() = url.clone();
                match write_config(&config) {
                    Ok(_) => {}
                    Err(_) => {
                        panic!("Error while updating configuration")
                    }
                }
                switch_screen::<MainWidget>();
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
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);

        let block = Block::bordered()
            .title("API URL")
            .title_alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));

        self.text.set_block(block);
        self.text.set_placeholder_text("http://localhost:8008");
        self.text.set_style(Style::default().fg(Color::Blue).bold());
        self.text.render(chunks[0], buf);
    }
}
