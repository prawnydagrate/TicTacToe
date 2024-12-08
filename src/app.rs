use crate::{
    app_state::{AppState, CurrentScreen, PassedState},
    helpers,
    screens::exiting,
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
use std::rc::Rc;

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
            CurrentScreen::Exiting => {
                exiting::ExitingWidget(PassedState(Rc::clone(&self.state.exiting)))
                    .render(area, buf)
            }
        }
    }
}

impl App<'_> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> AppResult {
        // main loop
        loop {
            if let Some(exiting::ExitingState::Left) = *self.state.exiting.borrow() {
                break;
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
            Event::Key(key_event) if key_event.kind == KeyEventKind::Release => {
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
        self.current_screen = CurrentScreen::Exiting;
        *self.state.exiting.borrow_mut() = Some(exiting::ExitingState::default());
    }
}
