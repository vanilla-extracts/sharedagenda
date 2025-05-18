use app::App;

pub mod app;
pub mod call;
pub mod widgets;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
