use crate::connectors::medium::MediumConnector;

pub struct AppState {
    pub medium: Box<dyn MediumConnector + Sync + Send>,
}
