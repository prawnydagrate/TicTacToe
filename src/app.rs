use crate::{
    app_state::AppState,
    helpers,
    screens::{exiting, ingame, pregame, CurrentScreen},
    AppResult,
};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    style::{Color, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
    current_screen: CurrentScreen,
    state: AppState,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![" <Q> ".bold().blue(), "to exit ".cyan()]);
        Block::new().bg(Color::Rgb(17, 17, 17)).render(area, buf);
        let area = helpers::centered_scale(area, 0.9, 0.9);
        let block = Block::bordered()
            .title(Line::from(" Toe Tac Tic ".bold()).centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        Paragraph::new("").centered().block(block).render(area, buf);
        match self.current_screen {
            CurrentScreen::Pregame => self.scr_pregame_render(area, buf),
            CurrentScreen::Ingame => self.scr_ingame_render(area, buf),
            CurrentScreen::Exiting(_) => self.scr_exiting_render(area, buf),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> AppResult {
        // main loop
        loop {
            // exit
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
            KeyCode::Char('q') => self.exit(),
            key => match self.current_screen {
                CurrentScreen::Pregame => self.scr_pregame_handle_key(key),
                CurrentScreen::Ingame => self.scr_ingame_handle_key(key),
                CurrentScreen::Exiting(_) => self.scr_exiting_handle_key(key),
            },
        }
    }

    // pregame screen
    fn scr_pregame_render(&self, area: Rect, buf: &mut Buffer) {
        if let (CurrentScreen::Pregame, Some(st)) = (&self.current_screen, &self.state.pregame) {
            pregame::PregameWidget(helpers::pass(st)).render(area, buf);
        }
    }

    fn scr_pregame_handle_key(&mut self, key: KeyCode) {}

    // ingame screen
    fn scr_ingame_render(&self, area: Rect, buf: &mut Buffer) {
        if let (CurrentScreen::Ingame, Some(st)) = (&self.current_screen, &self.state.ingame) {
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
            exiting::ExitingWidget(helpers::pass(st)).render(area, buf);
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
            KeyCode::Enter => {
                if let (CurrentScreen::Exiting(prev), Some(s)) =
                    (&self.current_screen, &self.state.exiting)
                {
                    self.current_screen = *prev.clone();
                    if *s.borrow() == exiting::ExitingState::Leave {
                        *s.borrow_mut() = exiting::ExitingState::Left;
                    }
                }
            }
            _ => (),
        }
    }

    fn exit(&mut self) {
        if !matches!(self.current_screen, CurrentScreen::Exiting(_)) {
            self.current_screen = CurrentScreen::Exiting(Box::new(self.current_screen.clone()));
            self.state.exiting = Some(helpers::rfc(exiting::ExitingState::default()));
        }
    }
}
