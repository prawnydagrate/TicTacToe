use toetactic_lib::mech::{Game, Move};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct IngameState<'a> {
    game: &'a Game,
    selected: Move,
}
