//! Events that are sent via webhook or websocket.

use serde::{Deserialize, Serialize};

/// The Kind of Event.
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Deserialize)]
pub enum EventKind {
    /// New SMS message received.
    #[serde(rename = "incoming")]
    IncomingMessage,

    /// SMS message being sent from API or other connected client.
    #[serde(rename = "outgoing")]
    OutgoingMessage,

    /// Delivery report update.
    #[serde(rename = "delivery")]
    DeliveryReport,

    /// Modem hat connection status update.
    #[serde(rename = "modem_status_update")]
    ModemStatusUpdate,

    /// An unsolicited position report from GNSS.
    #[serde(rename = "gnss_position_report")]
    GNSSPositionReport,

    /// WebSocket connection status update (client-side only).
    #[serde(rename = "websocket_connection_update")]
    WebsocketConnectionUpdate,
}
impl EventKind {
    /// Total number of `EventKind`'s.
    pub const COUNT: usize = 6;

    /// Make the `EventKind` into it's u8 bit representation.
    #[inline]
    #[must_use]
    pub const fn to_bit(self) -> u8 {
        match self {
            EventKind::IncomingMessage => 1 << 0,
            EventKind::OutgoingMessage => 1 << 1,
            EventKind::DeliveryReport => 1 << 2,
            EventKind::ModemStatusUpdate => 1 << 3,
            EventKind::GNSSPositionReport => 1 << 4,
            EventKind::WebsocketConnectionUpdate => 1 << 5,
        }
    }

    /// Create a bitmask with all server `EventKind`'s.
    #[inline]
    #[must_use]
    pub const fn all_bits() -> u8 {
        (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4)
    }

    /// Takes a set of `EventKinds` and returns its mask.
    #[inline]
    #[must_use]
    pub fn events_to_mask(events: &[EventKind]) -> u8 {
        events.iter().fold(0, |acc, event| acc | event.to_bit())
    }
}
impl From<&Event> for EventKind {
    fn from(value: &Event) -> Self {
        match value {
            Event::IncomingMessage(_) => EventKind::IncomingMessage,
            Event::OutgoingMessage(_) => EventKind::OutgoingMessage,
            Event::DeliveryReport { .. } => EventKind::DeliveryReport,
            Event::ModemStatusUpdate { .. } => EventKind::ModemStatusUpdate,
            Event::WebsocketConnectionUpdate { .. } => EventKind::WebsocketConnectionUpdate,

            #[cfg(feature = "gnss")]
            Event::GnssPositionReport(_) => EventKind::GNSSPositionReport,
        }
    }
}
impl TryFrom<&str> for EventKind {
    type Error = String;

    /// Convert a str into an `EventKind`.
    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "incoming" => Ok(EventKind::IncomingMessage),
            "outgoing" => Ok(EventKind::OutgoingMessage),
            "delivery" => Ok(EventKind::DeliveryReport),
            "modem_status_update" => Ok(EventKind::ModemStatusUpdate),
            "websocket_connection_upgrade" => Ok(EventKind::WebsocketConnectionUpdate),
            "gnss_position_report" => Ok(EventKind::GNSSPositionReport),
            _ => Err(format!("Unknown event type {value}")),
        }
    }
}

/// Event types that can be sent by the server.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Event {
    /// New SMS message received.
    #[serde(rename = "incoming")]
    IncomingMessage(crate::sms::SmsMessage),

    /// SMS message being sent from API or other connected client.
    #[serde(rename = "outgoing")]
    OutgoingMessage(crate::sms::SmsMessage),

    /// Delivery report update.
    #[serde(rename = "delivery")]
    DeliveryReport {
        /// The target `message_id` this delivery report applies to.
        /// This is determined from the `message_reference` and sender.
        message_id: i64,

        /// The received delivery report.
        report: crate::sms::SmsPartialDeliveryReport,
    },

    /// Modem hat connection status update.
    /// This can be either: Startup, Online, `ShuttingDown`, Offline
    #[serde(rename = "modem_status_update")]
    ModemStatusUpdate {
        /// Previous state from last update.
        previous: crate::modem::ModemStatusUpdateState,

        /// Current state after update.
        current: crate::modem::ModemStatusUpdateState,
    },

    /// WebSocket connection status update (client-side only).
    /// This message is generated locally when there is a connection or disconnection.
    WebsocketConnectionUpdate {
        /// Connection status: true = connected, false = disconnected
        connected: bool,

        /// If connection is false, will the client attempt to automatically reconnect?
        reconnect: bool,
    },

    /// An unsolicited position report from GNSS.
    #[cfg(feature = "gnss")]
    #[serde(rename = "gnss_position_report")]
    GnssPositionReport(crate::gnss::PositionReport),
}
