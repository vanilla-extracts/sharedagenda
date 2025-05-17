use ratatui::Frame;

use crate::{app::App, widgets::main::MainWidget};

pub fn ui(frame: &mut Frame, app: &App) {
    let main_screen = MainWidget {
        token: app.config.token.as_str(),
        api_link: app.config.api_link.as_str(),
    };
    frame.render_widget(main_screen, frame.area());
}
