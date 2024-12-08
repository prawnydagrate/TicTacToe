use crate::AppResult;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    widgets::Widget,
    DefaultTerminal, Frame,
};
use toetactic_lib::mech::{Game, Move};

const MIN_GRID_SIZE: usize = 3;
const MAX_GRID_SIZE: usize = 8; // exclusive
const GRID_SIZES: std::ops::Range<usize> = MIN_GRID_SIZE..MAX_GRID_SIZE;
const SIZE_DEPTHS: [usize; MAX_GRID_SIZE] = [0, 0, 0, 6, 6, 5, 5, 4];

#[derive(Debug, Default)]
enum CurrentScreen {
    #[default]
    Setup,
    Game,
}

#[derive(Debug)]
enum State {
    Setup(usize),
    Game(Option<Move>, Option<Game>),
}

impl Default for State {
    fn default() -> Self {
        Self::Setup(MIN_GRID_SIZE)
    }
}

#[derive(Debug, Default)]
pub struct App {
    screen: CurrentScreen,
    state: State,
    exit: bool,
}

impl Widget for &App {
    fn render(self) {
        todo!()
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> AppResult {
        // main loop
        while !self.exit {
            // render
            terminal.draw(|frame| self.draw(frame));
            // handle events
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
