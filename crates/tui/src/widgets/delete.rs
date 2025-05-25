use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, List, ListItem, ListState, StatefulWidget, Widget},
};

use crate::app::TuiWidget;

use super::main::{MainWidget, switch_screen};

#[derive(Clone, Debug)]
pub struct DeleteWidget {
    pub forms: ButtonList,
}

#[derive(Clone, Debug)]
pub struct ButtonList {
    pub buttons: Vec<Button>,
    pub state: ListState,
}

#[derive(Clone, Debug)]
pub struct Button {
    pub button_type: ButtonType,
    pub button_name: String,
}

#[derive(Clone, Debug)]
pub enum ButtonType {
    Yes,
    No,
}

impl Button {
    pub fn new(button_type: ButtonType, name: &str) -> Self {
        Self {
            button_type,
            button_name: name.to_string(),
        }
    }
}

impl Default for DeleteWidget {
    fn default() -> Self {
        let mut wid = Self {
            forms: ButtonList {
                buttons: vec![
                    Button::new(ButtonType::Yes, "Yes"),
                    Button::new(ButtonType::No, "No"),
                ],
                state: ListState::default(),
            },
        };
        wid.forms.state.select(Some(0));
        wid
    }
}

impl From<&Button> for ListItem<'_> {
    fn from(value: &Button) -> Self {
        ListItem::new(Line::from(value.button_name.clone()).fg(Color::White))
    }
}
impl TuiWidget for DeleteWidget {
    fn handle_key_event<T: TuiWidget + Default + Clone>(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) {
        match key.code {
            KeyCode::Esc => {
                switch_screen::<MainWidget>();
            }
            KeyCode::Up | KeyCode::BackTab | KeyCode::Left => {
                self.forms.state.select(Some(0));
            }
            KeyCode::Down | KeyCode::Tab | KeyCode::Right => {
                self.forms.state.select(Some(1));
            }
            KeyCode::Enter => match self.forms.state.selected() {
                Some(i) => match &self.forms.buttons[i].button_type {
                    ButtonType::Yes => {
                        todo!()
                    }
                    ButtonType::No => {
                        switch_screen::<MainWidget>();
                    }
                },
                None => {}
            },
            _ => {}
        }
    }
}

impl Widget for DeleteWidget {
    fn render(mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let ar = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5), Constraint::Min(1)])
            .split(area);

        let block = Block::bordered()
            .title("Are you sure?")
            .title_alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));
        let items: Vec<ListItem<'_>> = self.forms.buttons.iter().map(ListItem::from).collect();
        let list = List::new(items)
            .block(block)
            .style(Style::default().fg(Color::White))
            .highlight_symbol("> ")
            .highlight_style(Style::default().fg(Color::Blue).bold());

        StatefulWidget::render(list, ar[0], buf, &mut self.forms.state);
    }
}
