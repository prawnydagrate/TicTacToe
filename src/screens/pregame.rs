use crate::{consts, helpers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::{border, line},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};

pub fn instructions() -> Vec<Span<'static>> {
    vec![
        " ↑↓".bold().blue(),
        " Change grid".into(),
        "  ⏎".bold().blue(),
        " Select grid ".into(),
    ]
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct PregameState {
    pub grid_size: usize,
}

impl Default for PregameState {
    fn default() -> Self {
        Self {
            grid_size: consts::MIN_GRID_SIZE,
        }
    }
}

pub struct PregameWidget(pub helpers::Rfc<PregameState>);

fn get_collapsed_borders(
    r: usize,
    c: usize,
    gsize: usize,
    bset: border::Set,
    lset: line::Set,
) -> (Borders, border::Set) {
    if r == 0 {
        // first row
        if c == 0 {
            // first cell
            (Borders::LEFT | Borders::TOP, bset)
        } else if c + 1 == gsize {
            // last column
            (
                Borders::LEFT | Borders::TOP | Borders::RIGHT,
                border::Set {
                    top_left: lset.horizontal_down,
                    ..bset
                },
            )
        } else {
            // somewhere in the middle
            (
                Borders::LEFT | Borders::TOP,
                border::Set {
                    top_left: lset.horizontal_down,
                    ..bset
                },
            )
        }
    } else if r + 1 == gsize {
        // last row
        if c == 0 {
            // first column
            (
                Borders::BOTTOM | Borders::LEFT | Borders::TOP,
                border::Set {
                    top_left: lset.vertical_right,
                    ..bset
                },
            )
        } else if c + 1 == gsize {
            // last column
            (
                Borders::TOP | Borders::RIGHT | Borders::BOTTOM | Borders::LEFT,
                border::Set {
                    top_left: lset.cross,
                    top_right: lset.vertical_left,
                    bottom_left: lset.horizontal_up,
                    ..bset
                },
            )
        } else {
            // somewhere in the middle
            (
                Borders::BOTTOM | Borders::LEFT | Borders::TOP,
                border::Set {
                    top_left: lset.cross,
                    bottom_left: lset.horizontal_up,
                    ..bset
                },
            )
        }
    } else {
        // one of the rows in between
        if c == 0 {
            // first column
            (
                Borders::LEFT | Borders::TOP,
                border::Set {
                    top_left: lset.vertical_right,
                    ..bset
                },
            )
        } else if c + 1 == gsize {
            // last column
            (
                Borders::LEFT | Borders::TOP | Borders::RIGHT,
                border::Set {
                    top_left: lset.cross,
                    top_right: lset.vertical_left,
                    ..bset
                },
            )
        } else {
            // somewhere in between
            (
                Borders::LEFT | Borders::TOP,
                border::Set {
                    top_left: lset.cross,
                    ..bset
                },
            )
        }
    }
}

impl Widget for &PregameWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (gwidth, gheight) = if area.width * 2 < area.height {
            let w = area.width as f64 * consts::PREGAME_GRID_SIZE;
            (w.round() as u16, (w / 2.).round() as u16)
        } else {
            let h = area.height as f64 * consts::PREGAME_GRID_SIZE;
            ((h * 2.).round() as u16, h.round() as u16)
        };
        let garea = helpers::center(
            area,
            Constraint::Length(gwidth),
            Constraint::Length(gheight),
        );
        let grid_size = (*self.0).borrow().grid_size;
        Block::default()
            .title(Line::from(format!("Choose your grid: {grid_size}x{grid_size}")).centered())
            .render(helpers::centered_scale(garea, 1.1, 1.1), buf);
        let rows = Layout::vertical((0..grid_size).map(|_| Constraint::Fill(1))).split(garea);
        for (r, &row) in rows.iter().enumerate() {
            let cols = Layout::horizontal((0..grid_size).map(|_| Constraint::Fill(1))).split(row);
            for (c, &cell) in cols.iter().enumerate() {
                let (borders, border_set) =
                    get_collapsed_borders(r, c, grid_size, border::PLAIN, line::NORMAL);
                Block::new()
                    .borders(borders)
                    .border_set(border_set)
                    .render(cell, buf);
            }
        }
    }
}
