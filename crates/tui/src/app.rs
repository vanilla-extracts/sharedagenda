use core::panic;
use std::io;

use common::configuration::loader::{Loaded, load, load_config};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, widgets::Widget};

use crate::widgets::{
    main::MainWidget, navigation_bar::NavigationBar, template::TemplateWidget, top::Top,
};

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

pub trait TuiWidget: Widget {
    fn handle_key_event<T: TuiWidget + Default + Clone>(&mut self, key: KeyEvent);
}

#[derive(Clone, Debug)]
pub struct App<'a, T: TuiWidget + Default + Clone> {
    pub config: Loaded<'a>,
    pub exit: bool,
    pub current_screen: CurrentScreen,
    pub current_widget: T,
}

impl Default for App<'_, MainWidget> {
    fn default() -> Self {
        let config = match load() {
            Ok(c) => load_config(c),
            Err(_) => {
                panic!("Could not load config")
            }
        };
        Self {
            config,
            exit: false,
            current_screen: CurrentScreen::Main,
            current_widget: MainWidget::default(),
        }
    }
}

impl<T: TuiWidget + Default + Clone> App<'_, T> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn new(widget: T, current_screen: CurrentScreen) -> Self {
        let config = match load() {
            Ok(c) => load_config(c),
            Err(_) => {
                panic!("Could not load config")
            }
        };
        Self {
            config,
            exit: false,
            current_screen,
            current_widget: widget,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let wid = TemplateWidget {
            top_bar: Top {
                token: &self.clone().config.token,
                api_link: &self.clone().config.api_link,
            },
            middle: self.current_widget.clone(),
            navigation_bar: NavigationBar::default(),
        };

        frame.render_widget(wid, frame.area());
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
        self.current_widget.handle_key_event::<T>(key);
    }
}
