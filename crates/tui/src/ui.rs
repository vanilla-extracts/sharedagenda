use ratatui::Frame;

use crate::{
    app::App,
    widgets::{
        main::MainWidget, navigation_bar::NavigationBar, template::TemplateWidget, top::Top,
    },
};

pub fn ui(frame: &mut Frame, app: &App) {
    let main_screen = TemplateWidget {
        top_bar: Top {
            token: &app.config.token,
            api_link: &app.config.api_link,
        },
        middle: &mut MainWidget::new(),
        navigation_bar: NavigationBar::default(),
    };
    frame.render_widget(main_screen, frame.area());
}
