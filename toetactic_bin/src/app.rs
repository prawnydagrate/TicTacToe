use crate::{
    app_state::AppState,
    consts, helpers,
    screens::{exiting, ingame, pregame, pregame_confirm, startover, CurrentScreen},
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
use std::{thread, time::Duration};
use toetactic_lib::mech::{Game, GameState, Player};

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
            CurrentScreen::PregameConfirm => pregame_confirm::instructions(),
            CurrentScreen::Ingame => ingame::instructions(),
            CurrentScreen::Startover => startover::instructions(),
            CurrentScreen::Exiting(_) => exiting::instructions(),
        });
        if !matches!(self.current_screen, CurrentScreen::Exiting(_)) {
            instructions.extend([" qq".bold().fg(consts::INSTRUCTIONS_COLOR), " Exit ".into()]);
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
            CurrentScreen::PregameConfirm => self.scr_pregame_confirm_render(area, buf),
            CurrentScreen::Ingame => self.scr_ingame_render(area, buf),
            CurrentScreen::Startover => self.scr_startover_render(area, buf),
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
        if event::poll(Duration::from_millis(0))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_press(key_event)
                }
                _ => (),
            }
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
                CurrentScreen::PregameConfirm => self.scr_pregame_confirm_handle_key(key),
                CurrentScreen::Ingame => self.scr_ingame_handle_key(key),
                CurrentScreen::Startover => self.scr_startover_handle_key(key),
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
                KeyCode::Up | KeyCode::Right | KeyCode::Char('k') => {
                    if s.grid_size < consts::MAX_GRID_SIZE {
                        s.grid_size += 1
                    }
                }
                KeyCode::Down | KeyCode::Left | KeyCode::Char('j') => {
                    if s.grid_size > consts::MIN_GRID_SIZE {
                        s.grid_size -= 1
                    }
                }
                KeyCode::Enter => {
                    self.state.pregame_confirm =
                        Some(helpers::rfc(pregame_confirm::PregameConfirmState {
                            pregame_state: *s,
                            option_state: pregame_confirm::PregameConfirmOptionState::default(),
                        }));
                    self.current_screen = CurrentScreen::PregameConfirm;
                }
                _ => (),
            }
        }
    }

    // pregame confirm dialog
    fn scr_pregame_confirm_render(&self, area: Rect, buf: &mut Buffer) {
        self.scr_pregame_render(area, buf);
        if let Some(ref st) = self.state.pregame_confirm {
            pregame_confirm::PregameConfirmWidget(helpers::pass(st)).render(
                helpers::center(area, Constraint::Percentage(consts::DIALOG_PERCENTAGES.0), Constraint::Percentage(consts::DIALOG_PERCENTAGES.1)),
                buf,
            );
        }
    }

    fn scr_pregame_confirm_handle_key(&mut self, key: KeyCode) {
        use pregame_confirm::PregameConfirmOptionState::*;

        if let Some(ref s) = self.state.pregame_confirm {
            match key {
                KeyCode::Right | KeyCode::Down | KeyCode::Char('l' | 'j') => {
                    s.borrow_mut().option_state = O;
                }
                KeyCode::Left | KeyCode::Up | KeyCode::Char('h' | 'k') => {
                    s.borrow_mut().option_state = X;
                }
                KeyCode::Char(c @ ('x' | 'o')) => {
                    // final ones
                    s.borrow_mut().option_state = match c {
                        'x' => X,
                        'o' => O,
                        _ => unreachable!(),
                    };
                    self.scr_pregame_confirm_finish();
                }
                KeyCode::Esc => {
                    self.current_screen = CurrentScreen::Pregame;
                }
                KeyCode::Enter => self.scr_pregame_confirm_finish(),
                _ => (),
            }
        }
    }

    fn scr_pregame_confirm_finish(&mut self) {
        use pregame_confirm::PregameConfirmOptionState::*;

        if let CurrentScreen::PregameConfirm = &self.current_screen {
            if let (Some(turn), Some(pregame_st)) =
                (&self.state.pregame_confirm, &self.state.pregame)
            {
                self.state.ingame = Some(helpers::amtx(ingame::IngameState {
                    game: Game::new(pregame_st.borrow().grid_size),
                    user: match turn.borrow().option_state {
                        X => Player::X,
                        O => Player::O,
                    },
                    selected: (0, 0),
                    inthread: false,
                }));
                self.current_screen = CurrentScreen::Ingame;
            }
        }
    }

    // ingame screen
    fn scr_ingame_render(&self, area: Rect, buf: &mut Buffer) {
        if let Some(ref st) = self.state.ingame {
            ingame::IngameWidget(helpers::pass_atomic(st)).render(area, buf);
            let mut s = st.lock().unwrap();
            // TODO
            if s.game.state() == GameState::Ongoing && s.user != s.game.turn() && !s.inthread {
                s.inthread = true;
                let game = s.game.clone();
                let state = helpers::pass_atomic(st);
                thread::spawn(move || {
                    let best =
                        toetactic_lib::get_best_move(&game, consts::SIZE_DEPTHS[game.grid().n()]);
                    let mut st = state.lock().unwrap();
                    st.game.play(best).unwrap();
                    st.inthread = false;
                });
            }
        }
    }

    fn scr_ingame_handle_key(&mut self, key: KeyCode) {
        if let Some(ref st) = self.state.ingame {
            let mut s = st.lock().unwrap();
            let (r, c) = s.selected;
            let maxrc = s.game.grid().n() - 1;
            let left = (r, c.saturating_sub(1));
            let down = (if r < maxrc { r + 1 } else { r }, c);
            let up = (r.saturating_sub(1), c);
            let right = (r, if c < maxrc { c + 1 } else { c });
            match key {
                KeyCode::Left | KeyCode::Char('h') => s.selected = left,
                KeyCode::Down | KeyCode::Char('j') => s.selected = down,
                KeyCode::Up | KeyCode::Char('k') => s.selected = up,
                KeyCode::Right | KeyCode::Char('l') => s.selected = right,
                KeyCode::Char(' ') => {
                    if s.user == s.game.turn() {
                        s.game.play((r, c));
                    }
                }
                KeyCode::Char('r') => {
                    self.state.startover = Some(helpers::rfc(startover::StartoverState::default()));
                    self.current_screen = CurrentScreen::Startover;
                }
                _ => (),
            }
        }
    }

    // start over dialog
    fn scr_startover_render(&self, area: Rect, buf: &mut Buffer) {
        self.scr_ingame_render(area, buf);
        if let Some(ref st) = self.state.startover {
            startover::StartoverWidget(helpers::pass(st)).render(
                helpers::center(area, Constraint::Percentage(consts::DIALOG_PERCENTAGES.0), Constraint::Percentage(consts::DIALOG_PERCENTAGES.1)),
                buf,
            );
        }
    }

    fn scr_startover_handle_key(&mut self, key: KeyCode) {
        use startover::StartoverState::*;

        if let Some(ref s) = self.state.startover {
            match key {
                KeyCode::Right | KeyCode::Down | KeyCode::Char('l' | 'j') => {
                    *s.borrow_mut() = StartOver;
                }
                KeyCode::Left | KeyCode::Up | KeyCode::Char('h' | 'k') => {
                    *s.borrow_mut() = Stay;
                }
                KeyCode::Char('r') => {
                    // final
                    *s.borrow_mut() = StartOver;
                    self.scr_startover_finish();
                }
                KeyCode::Esc => {
                    self.current_screen = CurrentScreen::Ingame;
                }
                KeyCode::Enter => self.scr_startover_finish(),
                _ => (),
            }
        }
    }

    fn scr_startover_finish(&mut self) {
        use startover::StartoverState::*;

        if let CurrentScreen::Startover = &self.current_screen {
            if let Some(st) = &self.state.startover {
                match *st.borrow() {
                    Stay => self.current_screen = CurrentScreen::Ingame,
                    StartOver => self.current_screen = CurrentScreen::Pregame,
                }
            }
        }
    }

    // exiting dialog
    fn scr_exiting_render(&self, area: Rect, buf: &mut Buffer) {
        if let (CurrentScreen::Exiting(scr), Some(st)) = (&self.current_screen, &self.state.exiting)
        {
            match **scr {
                CurrentScreen::Pregame => self.scr_pregame_render(area, buf),
                CurrentScreen::PregameConfirm => self.scr_pregame_confirm_render(area, buf),
                CurrentScreen::Ingame => self.scr_ingame_render(area, buf),
                CurrentScreen::Startover => self.scr_startover_render(area, buf),
                CurrentScreen::Exiting(_) => unreachable!(),
            }
            exiting::ExitingWidget(helpers::pass(st)).render(
                helpers::center(area, Constraint::Percentage(consts::DIALOG_PERCENTAGES.0), Constraint::Percentage(consts::DIALOG_PERCENTAGES.1)),
                buf,
            );
        }
    }

    fn scr_exiting_handle_key(&mut self, key: KeyCode) {
        use exiting::ExitingState::*;

        if let Some(ref s) = self.state.exiting {
            match key {
                KeyCode::Right | KeyCode::Down | KeyCode::Char('l' | 'j') => {
                    *s.borrow_mut() = Leave;
                }
                KeyCode::Left | KeyCode::Up | KeyCode::Char('h' | 'k') => {
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
