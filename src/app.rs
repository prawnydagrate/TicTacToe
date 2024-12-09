use crate::{
    app_state::AppState,
    helpers,
    screens::{exiting, CurrentScreen},
    AppResult,
};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App<'a> {
    current_screen: CurrentScreen,
    state: AppState<'a>,
}

impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(Line::from(" Toe Tac Tic ".bold()).centered())
            .border_set(border::ROUNDED);
        let area = helpers::centered_scale(area, 0.9, 0.9);
        Paragraph::new("").centered().block(block).render(area, buf);
        match self.current_screen {
            CurrentScreen::Pregame => (),
            CurrentScreen::Ingame => (),
            CurrentScreen::Exiting(_) => {
                if let Some(s) = &self.state.exiting {
                    exiting::ExitingWidget(helpers::pass(s)).render(area, buf)
                }
            }
        }
    }
}

impl App<'_> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> AppResult {
        // main loop
        loop {
            if let Some(s) = &self.state.exiting {
                if *s.borrow() == exiting::ExitingState::Left {
                    break;
                }
            }
            // render
            terminal.draw(|frame| self.draw(frame))?;
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
            key => match self.current_screen {
                CurrentScreen::Pregame => (),
                CurrentScreen::Ingame => (),
                CurrentScreen::Exiting(_) => self.scr_exiting_handle_key(key),
            },
        }
    }

    fn scr_exiting_handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Right | KeyCode::Char('l') => {
                if let Some(s) = &self.state.exiting {
                    *s.borrow_mut() = exiting::ExitingState::Leave;
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if let Some(s) = &self.state.exiting {
                    *s.borrow_mut() = exiting::ExitingState::Stay;
                }
            }
            KeyCode::Enter => match (&self.current_screen, &self.state.exiting) {
                (CurrentScreen::Exiting(prev), Some(s)) => {
                    self.current_screen = *prev.clone();
                    if *s.borrow() == exiting::ExitingState::Leave {
                        *s.borrow_mut() = exiting::ExitingState::Left;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    fn exit(&mut self) {
        // TODO: add confirmation dialog
        if !matches!(self.current_screen, CurrentScreen::Exiting(_)) {
            self.current_screen = CurrentScreen::Exiting(Box::new(self.current_screen.clone()));
            self.state.exiting = Some(helpers::rfc(exiting::ExitingState::default()));
        }
    }
}
