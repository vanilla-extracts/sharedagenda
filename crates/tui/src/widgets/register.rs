use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Widget},
};
use tui_textarea::TextArea;

use crate::app::{App, CurrentScreen, TuiWidget};

use super::main::MainWidget;

const SELECTED_STYLE: Style = Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD);
const UNSELECTED_STYLE: Style = Style::new().fg(Color::White);
#[derive(Clone, Debug)]
pub struct RegisterWidget<'a> {
    pub forms: Vec<TextArea<'a>>,
    pub selected: usize,
}

impl Default for RegisterWidget<'_> {
    fn default() -> Self {
        Self {
            forms: vec![
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
            ],
            selected: 0,
        }
    }
}

impl TuiWidget for RegisterWidget<'_> {
    fn handle_key_event<T: TuiWidget + Default + Clone>(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) {
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
            KeyCode::Up | KeyCode::BackTab => {
                if self.selected == 0 {
                    self.selected = self.forms.len() - 1;
                } else {
                    self.selected -= 1;
                }
            }
            KeyCode::Down | KeyCode::Tab => {
                if self.selected == self.forms.len() - 1 {
                    self.selected = 0;
                } else {
                    self.selected += 1;
                }
            }
            KeyCode::Enter => {}
            _ => {
                self.forms[self.selected].input(key);
            }
        }
    }
}

impl Widget for RegisterWidget<'_> {
    fn render(mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .split(area);

        let name_block = Block::bordered()
            .title("Name")
            .title_alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));

        self.forms[0].set_block(name_block);
        self.forms[0].set_placeholder_text("John DOE");
        if self.selected == 0 {
            self.forms[0].set_style(SELECTED_STYLE);
        } else {
            self.forms[0].set_style(UNSELECTED_STYLE);
        }
        self.forms[0].render(chunks[0], buf);

        let email_block = Block::bordered()
            .title("Email")
            .title_alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));

        self.forms[1].set_block(email_block);
        self.forms[1].set_placeholder_text("email@domain.tld");
        if self.selected == 1 {
            self.forms[1].set_style(SELECTED_STYLE);
        } else {
            self.forms[1].set_style(UNSELECTED_STYLE);
        }
        self.forms[1].render(chunks[1], buf);

        let password_block = Block::bordered()
            .title("Password")
            .title_alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));

        self.forms[2].set_mask_char('*');
        self.forms[2].set_block(password_block);
        self.forms[2].set_placeholder_text("Your password");
        if self.selected == 2 {
            self.forms[2].set_style(SELECTED_STYLE);
        } else {
            self.forms[2].set_style(UNSELECTED_STYLE);
        }
        self.forms[2].render(chunks[2], buf);
    }
}
