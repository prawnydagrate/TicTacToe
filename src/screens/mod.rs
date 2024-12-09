pub mod exiting;
pub mod ingame;
pub mod pregame;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub enum CurrentScreen {
    #[default]
    Pregame,
    Ingame,
    Exiting(Box<CurrentScreen>),
}
