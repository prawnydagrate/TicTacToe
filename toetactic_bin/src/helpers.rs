use ratatui::{layout::{Constraint, Flex, Layout, Rect}, symbols::{border, line}, widgets::Borders};
use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

pub type AppResult = std::io::Result<()>;
pub type Rfc<T> = Rc<RefCell<T>>;
pub type Amtx<T> = Arc<Mutex<T>>;

pub fn centered_scale(area: Rect, horiz_scale: f64, vert_scale: f64) -> Rect {
    let round = |f: f64| f.round() as u16;
    let mean = |a, b| (a as f64 + b as f64) / 2.;
    let (w, h) = (area.width as f64, area.height as f64);
    let (hw, hh) = (w / 2., h / 2.);
    let (wc, hc) = (
        mean(area.left(), area.right()),
        mean(area.top(), area.bottom()),
    );
    let (l, r, t, b) = (
        wc - hw * horiz_scale,
        wc + hw * horiz_scale,
        hc - hh * vert_scale,
        hc + hh * vert_scale,
    );
    Rect {
        x: round(l),
        y: round(t),
        width: round(r - l),
        height: round(b - t),
    }
}

pub fn center(area: Rect, horiz: Constraint, vert: Constraint) -> Rect {
    let [area] = Layout::horizontal([horiz]).flex(Flex::Center).areas(area);
    let [area] = Layout::vertical([vert]).flex(Flex::Center).areas(area);
    area
}

pub fn rfc<T>(v: T) -> Rfc<T> {
    Rc::new(RefCell::new(v))
}

pub fn amtx<T>(v: T) -> Amtx<T> {
    Arc::new(Mutex::new(v))
}

pub fn pass<T>(state: &Rfc<T>) -> Rfc<T> {
    Rc::clone(state)
}

pub fn pass_atomic<T>(state: &Amtx<T>) -> Amtx<T> {
    Arc::clone(state)
}

pub fn get_collapsed_borders(
    r: usize,
    c: usize,
    gsize: usize,
    bset: border::Set,
    lset: line::Set,
    ttt: bool,
) -> (Borders, border::Set) {
    if r == 0 {
        // first row
        if c == 0 {
            // first cell
            (
                if !ttt {
                    Borders::LEFT | Borders::TOP
                } else {
                    Borders::NONE
                },
                bset,
            )
        } else if c + 1 == gsize {
            // last column
            (
                Borders::LEFT
                    | if ttt {
                        Borders::NONE
                    } else {
                        Borders::TOP | Borders::RIGHT
                    },
                border::Set {
                    top_left: lset.horizontal_down,
                    ..bset
                },
            )
        } else {
            // somewhere in the middle
            (
                Borders::LEFT | if ttt { Borders::NONE } else { Borders::TOP },
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
                Borders::TOP
                    | if ttt {
                        Borders::NONE
                    } else {
                        Borders::BOTTOM | Borders::LEFT
                    },
                border::Set {
                    top_left: lset.vertical_right,
                    ..bset
                },
            )
        } else if c + 1 == gsize {
            // last column
            (
                Borders::TOP
                    | Borders::LEFT
                    | if ttt {
                        Borders::NONE
                    } else {
                        Borders::RIGHT | Borders::BOTTOM
                    },
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
                Borders::LEFT | Borders::TOP | if ttt { Borders::NONE } else { Borders::BOTTOM },
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
                Borders::TOP | if ttt { Borders::NONE } else { Borders::LEFT },
                border::Set {
                    top_left: lset.vertical_right,
                    ..bset
                },
            )
        } else if c + 1 == gsize {
            // last column
            (
                Borders::LEFT | Borders::TOP | if ttt { Borders::NONE } else { Borders::RIGHT },
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

