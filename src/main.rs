use toetactic::{App, AppResult};

fn main() -> AppResult {
    let mut terminal = ratatui::init();
    let app_res = App::default().run(&mut terminal);
    ratatui::restore();
    app_res
}
