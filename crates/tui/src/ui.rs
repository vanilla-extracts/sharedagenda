use ratatui::Frame;

use crate::{
    app::App,
    widgets::{main::MainWidget, template::TemplateWidget},
};

pub fn ui(frame: &mut Frame, app: &App) {
    let main_screen = TemplateWidget {
        token: app.config.token.as_str(),
        api_link: app.config.api_link.as_str(),
        middle: &mut MainWidget::new(),
    };
    frame.render_widget(main_screen, frame.area());
}
