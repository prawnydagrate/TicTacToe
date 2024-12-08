use crate::AppResult;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    DefaultTerminal,
};

#[derive(Debug, Default)]
enum Screen {
    #[default]
    Selector,
    Game,
}

#[derive(Debug, Default)]
pub struct App {
    screen: Screen,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> AppResult {
        // main loop
        while !self.exit {
            // render
            match self.screen {
                Screen::Selector => todo!(),
                Screen::Game => todo!(),
            }
            // handle events
            self.handle_events()?;
        }
    }

    fn handle_events(&mut self) -> AppResult {
        Ok(match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_press(key_event)
            }
            _ => (),
        })
    }

    fn handle_key_press(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => (),
        }
    }

    fn exit(&mut self) {
        // TODO: add confirmation dialog
        self.exit = true;
    }
}
