pub mod exiting;
pub mod ingame;
pub mod pregame;
pub mod pregame_confirm;
pub mod startover;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub enum CurrentScreen {
    #[default]
    Pregame,
    PregameConfirm,
    Ingame,
    Startover,
    Exiting(Box<CurrentScreen>),
}
