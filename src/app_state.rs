use crate::{
    helpers,
    screens::{exiting, ingame, pregame},
};

#[derive(Debug, Eq, PartialEq, Default)]
pub enum CurrentScreen {
    #[default]
    Pregame,
    Ingame,
    Exiting,
}

pub struct PassedState<T>(pub helpers::Rfc<Option<T>>);

impl<T> PassedState<T> {
    pub fn state(&self) -> &helpers::Rfc<Option<T>> {
        &self.0
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AppState<'a> {
    pub pregame: helpers::Rfc<Option<pregame::PregameState>>,
    pub ingame: helpers::Rfc<Option<ingame::IngameState<'a>>>,
    pub exiting: helpers::Rfc<Option<exiting::ExitingState>>,
}

impl Default for AppState<'_> {
    fn default() -> Self {
        Self {
            pregame: helpers::rfc(Some(pregame::PregameState::default())),
            ingame: helpers::rfc(None),
            exiting: helpers::rfc(None),
        }
    }
}
