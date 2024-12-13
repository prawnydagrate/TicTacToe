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
        " ←↑↓→".bold().blue(),
        " Navigate".into(),
        "  ⎵".bold().blue(),
        " Play ".into(),
    ]
}

#[derive(Debug, Clone)]
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
        let mut st = self.0.lock().unwrap();
        let grid_size = st.game.grid().n();
        Block::default()
            .title(
                Line::from(format!(
                    "{}",
                    if st.game.turn() == st.user {
                        "Your turn"
                    } else {
                        "The computer is thinking..."
                    }
                ))
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
                        mech::Cell::X => "X",
                        mech::Cell::O => "O",
                        _ => unreachable!(),
                    })
                    .centered()
                    .render(
                        helpers::center(cell, Constraint::Length(1), Constraint::Length(1)),
                        buf,
                    );
                }
                let userturn = st.game.turn() == st.user;
                if st.selected == (r, c)
                    && st.game.turn() == st.user
                    && st.game.state() == GameState::Ongoing
                {
                    Block::new()
                        .bg(if st.game.empty().contains(&(r, c)) {
                            Color::LightBlue
                        } else {
                            Color::LightRed
                        })
                        .render(helpers::centered_scale(cell, 0.4, 0.4), buf);
                }
                            }
        }
    }
}
