use crossterm::event::DisableMouseCapture;
use ratatui::{
    crossterm::{
        event::EnableMouseCapture,
        execute,
        terminal::{enable_raw_mode, EnterAlternateScreen},
    },
    prelude::CrosstermBackend,
    Terminal,
};
use toetactic::{App, AppResult};

fn main() -> AppResult {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // run app
    let app_res = App::default().run(&mut terminal);
    // cleanup
    ratatui::restore();
    execute!(terminal.backend_mut(), DisableMouseCapture)?;
    terminal.show_cursor()?;
    app_res
}
