use crate::{
    helpers,
    screens::{exiting, ingame, pregame, pregame_confirm, startover},
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub pregame: Option<helpers::Rfc<pregame::PregameState>>,
    pub pregame_confirm: Option<helpers::Rfc<pregame_confirm::PregameConfirmState>>,
    pub ingame: Option<helpers::Amtx<ingame::IngameState>>,
    pub startover: Option<helpers::Rfc<startover::StartoverState>>,
    pub exiting: Option<helpers::Rfc<exiting::ExitingState>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            pregame: Some(helpers::rfc(pregame::PregameState::default())),
            pregame_confirm: None,
            ingame: None,
            startover: None,
            exiting: None,
        }
    }
}
