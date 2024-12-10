use ratatui::layout::{Constraint, Flex, Layout, Rect};
use std::{cell::RefCell, rc::Rc};

pub type AppResult = std::io::Result<()>;
pub type Rfc<T> = Rc<RefCell<T>>;

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

pub fn pass<T>(state: &Rfc<T>) -> Rfc<T> {
    Rc::clone(state)
}
