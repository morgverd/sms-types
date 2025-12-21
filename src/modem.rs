//! Types used by the SMS server Modem, sent in events.

use serde::{Deserialize, Serialize};

/// Represents the current status of the modem.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ModemStatusUpdateState {
    /// Modem is starting up.
    Startup,

    /// Modem is online and operational.
    Online,

    /// Modem is shutting down.
    ShuttingDown,

    /// Modem is offline and not operational.
    Offline,
}
impl std::fmt::Display for ModemStatusUpdateState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ModemStatusUpdateState::Startup => write!(f, "Startup"),
            ModemStatusUpdateState::Online => write!(f, "Online"),
            ModemStatusUpdateState::ShuttingDown => write!(f, "ShuttingDown"),
            ModemStatusUpdateState::Offline => write!(f, "Offline"),
        }
    }
}
