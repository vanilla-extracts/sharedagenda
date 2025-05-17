use core::panic;
use std::io;

use common::configuration::loader::{Loaded, load, load_config};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::ui::ui;

pub enum UserRegisteringCurrentlyEditing {
    Name,
    Email,
    Password,
}

pub enum UserLoggingInCurrentlyEditing {
    Email,
    Password,
}

pub enum UserModifyingCurrentlyEditing {
    Name,
    Email,
    Password,
}

pub enum EventCreatingCurrentlyEditing {
    Name,
    DateStart,
    DateEnd,
    Invitees,
}

pub enum EventModifyingCurrentlyEditing {
    Name,
    DateStart,
    DateEnd,
}

pub enum CurrentScreen {
    Main,
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
            current_screen: CurrentScreen::Main,
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

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
