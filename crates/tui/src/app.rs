use core::panic;
use std::io;

use common::configuration::loader::{Loaded, load, load_config};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::{ui::ui, widgets::main::MainWidget};

#[derive(Clone, Debug)]
pub enum UserRegisteringCurrentlyEditing {
    Name,
    Email,
    Password,
}

#[derive(Clone, Debug)]
pub enum UserLoggingInCurrentlyEditing {
    Email,
    Password,
}

#[derive(Clone, Debug)]
pub enum UserModifyingCurrentlyEditing {
    Name,
    Email,
    Password,
}

#[derive(Clone, Debug)]
pub enum EventCreatingCurrentlyEditing {
    Name,
    DateStart,
    DateEnd,
    Invitees,
}

#[derive(Clone, Debug)]
pub enum EventModifyingCurrentlyEditing {
    Name,
    DateStart,
    DateEnd,
}

#[derive(Clone, Debug)]
pub enum CurrentScreen {
    Main(MainWidget),
    ApiEditing,
    UserRegistering(UserRegisteringCurrentlyEditing),
    UserLoggingIn(UserLoggingInCurrentlyEditing),
    UserLoggingOut,
    UserDeleting,
    UserListing,
    UserFetching,
    UserModifying(UserModifyingCurrentlyEditing),
    EventDeleting,
    EventCreating(EventCreatingCurrentlyEditing),
    EventListing,
    EventModifying(EventModifyingCurrentlyEditing),
}

#[derive(Clone, Debug)]
pub struct App<'a> {
    pub config: Loaded<'a>,
    pub current_screen: CurrentScreen,
    pub exit: bool,
}

impl Default for App<'_> {
    fn default() -> Self {
        let config = match load() {
            Ok(c) => load_config(c),
            Err(_) => {
                panic!("Could not load config")
            }
        };
        Self {
            config,
            current_screen: CurrentScreen::Main(MainWidget::new()),
            exit: false,
        }
    }
}

impl App<'_> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    pub fn draw(&mut self, frame: &mut Frame) {
        ui(frame, self);
    }
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        match self.clone().current_screen {
            CurrentScreen::Main(mut w) => {
                if key.kind != KeyEventKind::Press {
                    return;
                }
                match key.code {
                    KeyCode::Char('q') => self.exit(),
                    KeyCode::Char('j') | KeyCode::Down => w.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => w.select_previous(),
                    KeyCode::Right => {
                        w.select_next();

                        println!("{:?}", w.actions.state.selected());
                    }
                    KeyCode::Left => w.select_previous(),
                    KeyCode::Char('g') => w.select_first(),
                    KeyCode::Char('G') => w.select_last(),
                    KeyCode::Esc => w.select_none(),
                    KeyCode::Enter => {
                        println!("{:?}", w.actions.state.selected());
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
