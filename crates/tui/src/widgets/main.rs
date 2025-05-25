use std::process::exit;

use crossterm::event::KeyCode;
use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, List, ListItem, ListState, StatefulWidget, Widget},
};

use crate::app::{App, CurrentScreen, TuiWidget};

use super::api::ApiUrlWidget;

#[derive(Clone, Debug)]
pub struct MainWidget {
    actions: ActionList,
}
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

impl Default for MainWidget {
    fn default() -> Self {
        let mut wid = MainWidget {
            actions: ActionList {
                actions: vec![
                    Action::new(ActionType::ChangeApiUrl, "Change API URL"),
                    Action::new(ActionType::RegisterAccount, "Register new account"),
                    Action::new(ActionType::Login, "Login into your account"),
                    Action::new(ActionType::Logout, "Logout"),
                    Action::new(ActionType::Delete, "Delete your account"),
                    Action::new(
                        ActionType::ModifyAccount,
                        "Modify your account informations",
                    ),
                    Action::new(
                        ActionType::ShowInformation,
                        "Shows your account informations",
                    ),
                    Action::new(ActionType::ListUsers, "List all users"),
                    Action::new(ActionType::CreateEvent, "Create a new event"),
                    Action::new(ActionType::ShowEvents, "List all events in your calendar"),
                ],
                state: ListState::default(),
            },
        };
        wid.select_first();
        wid
    }
}

impl TuiWidget for MainWidget {
    fn handle_key_event<T: TuiWidget + Default + Clone>(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) {
        match key.code {
            KeyCode::Char('g') => self.select_first(),
            KeyCode::Char('G') => self.select_last(),
            KeyCode::Down | KeyCode::Right => self.select_next(),
            KeyCode::Up | KeyCode::Left => self.select_previous(),
            KeyCode::Char('q') | KeyCode::Esc => exit(0),
            KeyCode::Enter => match self.actions.state.selected() {
                Some(i) => {
                    let action = &self.actions.actions[i];
                    match action.action_type {
                        ActionType::ChangeApiUrl => {
                            ratatui::restore();
                            let mut terminal = ratatui::init();
                            let res = App::new(ApiUrlWidget::default(), CurrentScreen::ApiEditing)
                                .run(&mut terminal);
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
                None => {}
            },
            _ => {}
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
}

impl From<&Action> for ListItem<'_> {
    fn from(value: &Action) -> Self {
        ListItem::new(Line::from(value.action_name.clone()).fg(Color::White))
    }
}

impl Widget for MainWidget {
    fn render(mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered()
            .title("Actions")
            .title_alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));

        let ls: Vec<ListItem> = self.actions.actions.iter().map(ListItem::from).collect();

        let list = List::new(ls)
            .block(block)
            .style(Style::default().fg(Color::White))
            .highlight_symbol("> ")
            .highlight_style(Style::default().fg(Color::Blue).bold());
        StatefulWidget::render(list, area, buf, &mut self.actions.state);
    }
}
