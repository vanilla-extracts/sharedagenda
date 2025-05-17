use ratatui::Frame;

use crate::{
    app::{App, CurrentScreen},
    widgets::{navigation_bar::NavigationBar, template::TemplateWidget, top::Top},
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    match app.clone().current_screen {
        CurrentScreen::Main(mut w) => {
            let main_screen = TemplateWidget {
                top_bar: Top {
                    token: &app.config.token,
                    api_link: &app.config.api_link,
                },
                middle: &mut w,
                navigation_bar: NavigationBar::default(),
            };
            frame.render_widget(main_screen, frame.area());
        }
        _ => {}
    }
}
