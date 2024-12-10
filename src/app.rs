use crate::{
    app_state::AppState,
    consts, helpers,
    screens::{exiting, ingame, pregame, CurrentScreen},
    AppResult,
};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
    current_screen: CurrentScreen,
    state: AppState,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut instructions = Vec::new();
        instructions.extend(match self.current_screen {
            CurrentScreen::Pregame => pregame::instructions(),
            CurrentScreen::Exiting(_) => exiting::instructions(),
            _ => Vec::new(),
        });
        if !matches!(self.current_screen, CurrentScreen::Exiting(_)) {
            instructions.extend([" qq".bold().blue(), " Exit ".into()]);
        }
        let instructions = Line::from(instructions);
        let area = helpers::center(area, Constraint::Percentage(94), Constraint::Percentage(94));
        let block = Block::bordered()
            .title(Line::from(" Toe Tac Tic ".bold()).centered())
            .title_bottom(instructions.centered())
            .bg(consts::BGCOLOR)
            .border_set(border::THICK);
        block.render(area, buf);
        match self.current_screen {
            CurrentScreen::Pregame => self.scr_pregame_render(area, buf),
            CurrentScreen::Ingame => self.scr_ingame_render(area, buf),
            CurrentScreen::Exiting(_) => self.scr_exiting_render(area, buf),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> AppResult {
        'mainloop: loop {
            // exit
            if let Some(s) = &self.state.exiting {
                if *s.borrow() == exiting::ExitingState::Left {
                    break 'mainloop;
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
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_press(key_event)
            }
            _ => (),
        }
        Ok(())
    }

    fn handle_key_press(&mut self, key_event: KeyEvent) {
        match key_event.code {
            key @ KeyCode::Char('q') => {
                if let CurrentScreen::Exiting(_) = self.current_screen {
                    self.scr_exiting_handle_key(key)
                } else {
                    self.exit()
                }
            }
            key => match self.current_screen {
                CurrentScreen::Pregame => self.scr_pregame_handle_key(key),
                CurrentScreen::Ingame => self.scr_ingame_handle_key(key),
                CurrentScreen::Exiting(_) => self.scr_exiting_handle_key(key),
            },
        }
    }

    // pregame screen
    fn scr_pregame_render(&self, area: Rect, buf: &mut Buffer) {
        if let Some(ref st) = self.state.pregame {
            pregame::PregameWidget(helpers::pass(st)).render(area, buf);
        }
    }

    fn scr_pregame_handle_key(&mut self, key: KeyCode) {
        if let Some(ref st) = self.state.pregame {
            let mut s = (*st).borrow_mut();
            match key {
                KeyCode::Up => {
                    if s.grid_size < consts::MAX_GRID_SIZE {
                        s.grid_size += 1
                    }
                }
                KeyCode::Down => {
                    if s.grid_size > consts::MIN_GRID_SIZE {
                        s.grid_size -= 1
                    }
                }
                _ => (),
            }
        }
    }

    // ingame screen
    fn scr_ingame_render(&self, area: Rect, buf: &mut Buffer) {
        if let Some(ref st) = self.state.ingame {
            ingame::IngameWidget(helpers::pass(st)).render(area, buf);
        }
    }

    fn scr_ingame_handle_key(&mut self, key: KeyCode) {}

    // exiting screen
    fn scr_exiting_render(&self, area: Rect, buf: &mut Buffer) {
        if let (CurrentScreen::Exiting(scr), Some(st)) = (&self.current_screen, &self.state.exiting)
        {
            match **scr {
                CurrentScreen::Pregame => self.scr_pregame_render(area, buf),
                CurrentScreen::Ingame => self.scr_ingame_render(area, buf),
                CurrentScreen::Exiting(_) => unreachable!(),
            }
            exiting::ExitingWidget(helpers::pass(st)).render(
                helpers::center(area, Constraint::Percentage(60), Constraint::Percentage(40)),
                buf,
            );
        }
    }

    fn scr_exiting_handle_key(&mut self, key: KeyCode) {
        use exiting::ExitingState::*;

        if let Some(ref s) = self.state.exiting {
            match key {
                KeyCode::Right | KeyCode::Char('l') => {
                    *s.borrow_mut() = Leave;
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    *s.borrow_mut() = Stay;
                }
                KeyCode::Char(c @ ('y' | 'n' | 'q')) => {
                    // final ones
                    *s.borrow_mut() = match c {
                        'y' | 'q' => Leave,
                        'n' => Stay,
                        _ => unreachable!(),
                    };
                    self.scr_exiting_finish();
                }
                KeyCode::Esc => {
                    *s.borrow_mut() = Stay;
                    self.scr_exiting_finish();
                }
                KeyCode::Enter => self.scr_exiting_finish(),
                _ => (),
            }
        }
    }

    fn scr_exiting_finish(&mut self) {
        use exiting::ExitingState::*;

        if let CurrentScreen::Exiting(prev) = &self.current_screen {
            if let Some(s) = &self.state.exiting {
                self.current_screen = *prev.clone();
                let mut st = s.borrow_mut();
                if *st == Leave {
                    *st = Left;
                }
            }
        }
    }

    fn exit(&mut self) {
        if !matches!(self.current_screen, CurrentScreen::Exiting(_)) {
            self.current_screen = CurrentScreen::Exiting(Box::new(self.current_screen.clone()));
            self.state.exiting = Some(helpers::rfc(exiting::ExitingState::default()));
        }
    }
}
