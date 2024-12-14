use crate::{consts, helpers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    symbols::{border, line},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};
use toetactic_lib::mech::{self, Game, GameState, Move, Player};

pub fn instructions() -> Vec<Span<'static>> {
    vec![
        " ←↑↓→".bold().fg(consts::INSTRUCTIONS_COLOR),
        " Navigate".into(),
        "  ⎵".bold().fg(consts::INSTRUCTIONS_COLOR),
        " Play".into(),
        "  rr".bold().fg(consts::INSTRUCTIONS_COLOR),
        " Start over ".into(),
    ]
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IngameState {
    pub game: Game,
    pub user: Player,
    pub selected: Move,
    pub inthread: bool,
}

pub struct IngameWidget(pub helpers::Amtx<IngameState>);

impl Widget for &IngameWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (gwidth, gheight) = if area.width * 2 < area.height {
            let w = area.width as f64 * consts::INGAME_GRID_SIZE;
            (w.round() as u16, (w / 2.).round() as u16)
        } else {
            let h = area.height as f64 * consts::INGAME_GRID_SIZE;
            ((h * 2.).round() as u16, h.round() as u16)
        };
        let garea = helpers::center(
            area,
            Constraint::Length(gwidth),
            Constraint::Length(gheight),
        );
        let st = self.0.lock().unwrap();
        let grid_size = st.game.grid().n();
        Block::default()
            .title(
                Line::from(if st.game.state() != GameState::Ongoing {
                    vec![
                        "GAME OVER".bold(),
                        ": ".into(),
                        match st.game.state() {
                            GameState::Decisive(winner) => match winner {
                                Player::X => "X wins!",
                                Player::O => "O wins!",
                            },
                            GameState::Tied => "It's a tie!",
                            _ => unreachable!(),
                        }
                        .into(),
                    ]
                } else if st.game.turn() == st.user {
                    vec!["Your turn".into()]
                } else {
                    vec!["The computer is thinking...".into()]
                })
                .centered(),
            )
            .render(helpers::centered_scale(garea, 1.15, 1.15), buf);
        let rows = Layout::vertical((0..grid_size).map(|_| Constraint::Fill(1))).split(garea);
        for (r, &row) in rows.iter().enumerate() {
            let cols = Layout::horizontal((0..grid_size).map(|_| Constraint::Fill(1))).split(row);
            for (c, &cell) in cols.iter().enumerate() {
                let (borders, border_set) = helpers::get_collapsed_borders(
                    r,
                    c,
                    grid_size,
                    border::PLAIN,
                    line::NORMAL,
                    true,
                );
                Block::new()
                    .borders(borders)
                    .border_set(border_set)
                    .render(cell, buf);
                let content = st.game.grid().data()[r][c];
                if content != mech::Cell::Empty {
                    Paragraph::new(match content {
                        mech::Cell::X => {
                            if st.user == Player::X {
                                "X".bold().light_green()
                            } else {
                                "X".light_red()
                            }
                        }
                        mech::Cell::O => {
                            if st.user == Player::O {
                                "O".bold().light_green()
                            } else {
                                "O".fg(Color::Rgb(255, 101, 101))
                            }
                        }
                        _ => unreachable!(),
                    })
                    .centered()
                    .render(
                        helpers::center(cell, Constraint::Length(1), Constraint::Length(1)),
                        buf,
                    );
                }
                if st.selected == (r, c)
                    && st.game.turn() == st.user
                    && st.game.state() == GameState::Ongoing
                {
                    Block::new()
                        .bg(if st.game.empty().contains(&(r, c)) {
                            Color::Cyan
                        } else {
                            Color::DarkGray
                        })
                        .render(helpers::centered_scale(cell, 0.75, 0.75), buf);
                }
            }
        }
    }
}
