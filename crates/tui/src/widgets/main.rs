use std::process::exit;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, List, ListItem, ListState, StatefulWidget, Widget},
};

#[derive(Clone, Debug)]
pub struct MainWidget {
    pub actions: ActionList,
}
/*
layout:
--------
What do you wish to do?
-> api
-> users
->



*/
#[derive(Clone, Debug)]
pub enum ActionType {
    ChangeApiUrl,
    RegisterAccount,
    Login,
    Logout,
    Delete,
    ModifyAccount,
    ShowInformation,
    ListUsers,
    CreateEvent,
    ShowEvents,
}

#[derive(Clone, Debug)]
pub struct ActionList {
    pub actions: Vec<Action>,
    pub state: ListState,
}

#[derive(Clone, Debug)]
pub struct Action {
    pub action_type: ActionType,
    pub action_name: String,
}

impl Action {
    pub fn new(action_type: ActionType, name: &str) -> Self {
        Self {
            action_type,
            action_name: name.to_string(),
        }
    }
}

impl MainWidget {
    pub fn select_none(&mut self) {
        self.actions.state.select(None);
    }

    pub fn select_next(&mut self) {
        self.actions.state.select_next();
    }
    pub fn select_previous(&mut self) {
        self.actions.state.select_previous();
    }

    pub fn select_first(&mut self) {
        self.actions.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.actions.state.select_last();
    }
    pub fn new() -> Self {
        let mut new_widget = MainWidget {
            actions: ActionList {
                actions: vec![
                    Action::new(ActionType::ChangeApiUrl, "Change API URL"),
                    Action::new(ActionType::RegisterAccount, "Register new account"),
                    Action::new(ActionType::Login, "Login into your account"),
                ],
                state: ListState::default(),
            },
        };
        new_widget.actions.state.select(Some(1));
        new_widget
    }
}

impl From<&Action> for ListItem<'_> {
    fn from(value: &Action) -> Self {
        ListItem::new(Line::from(value.action_name.clone()).fg(Color::White))
    }
}

impl Widget for &mut MainWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered()
            .title("Actions")
            .title_alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));

        let ls: Vec<ListItem> = self
            .actions
            .actions
            .iter()
            .map(|f| ListItem::from(f))
            .collect();

        let list = List::new(ls)
            .block(block)
            .style(Style::default().fg(Color::White))
            .highlight_symbol("> ")
            .highlight_style(Style::default().fg(Color::Blue).bold());

        StatefulWidget::render(list, area, buf, &mut self.actions.state);
    }
}
