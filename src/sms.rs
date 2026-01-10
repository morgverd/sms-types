//! Generic types that apply to both HTTP and Websocket interfaces.

use serde::{Deserialize, Serialize};

/// Represents a stored SMS message from the database.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SmsMessage {
    /// Unique identifier for the message.
    pub message_id: Option<i64>,

    /// The phone number associated with this message.
    pub phone_number: String,

    /// The actual text content of the message.
    pub message_content: String,

    /// Optional reference number for message tracking.
    /// This is assigned by the modem and is only present for outgoing messages.
    pub message_reference: Option<u8>,

    /// Whether this message was sent (true) or received (false).
    pub is_outgoing: bool,

    /// Unix timestamp when the message was created.
    pub created_at: Option<u32>,

    /// Optional Unix timestamp when the message was completed/delivered.
    pub completed_at: Option<u32>,

    /// Service message center delivery status.
    pub status: Option<u8>,
}
impl SmsMessage {
    /// Returns a clone of the message with the `message_id` option replaced.
    #[must_use]
    pub fn with_message_id(&self, id: Option<i64>) -> Self {
        Self {
            message_id: id,
            ..self.clone()
        }
    }

    /// Get the message created_at time as SystemTime.
    #[must_use]
    pub fn created_at(&self) -> Option<std::time::SystemTime> {
        self.created_at
            .map(|ts| std::time::UNIX_EPOCH + std::time::Duration::from_secs(u64::from(ts)))
    }
}

/// The outgoing SMS message to be sent to a target number.
#[derive(Serialize, PartialEq, Default, Debug, Clone)]
pub struct SmsOutgoingMessage {
    /// The target phone number, this should be in international format.
    pub to: String,

    /// The full message content. This will be split into multiple messages
    /// by the server if required. This also supports Unicode emojis etc.
    pub content: String,

    /// The relative validity period to use for message sending. This determines
    /// how long the message should remain waiting while undelivered.
    /// By default, this is determined by the server (24 hours).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<u8>,

    /// Should the SMS message be sent as a Silent class? This makes a popup
    /// show on the users device with the message content if they're logged in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash: Option<bool>,

    /// A timeout that should be applied to the entire request.
    /// If one is not set, the default timeout is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
}
impl SmsOutgoingMessage {
    /// Create a new outgoing message with a default validity period and no flash.
    /// The default validity period is applied by SMS-API, so usually 24 hours.
    pub fn simple_message(to: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            to: to.into(),
            content: content.into(),
            ..Default::default()
        }
    }

    /// Set the message flash state. This will show a popup if the recipient is
    /// logged-in to their phone, otherwise as a normal text message.
    #[must_use]
    pub fn with_flash(mut self, flash: bool) -> Self {
        self.flash = Some(flash);
        self
    }

    /// Set a relative validity period value.
    #[must_use]
    pub fn with_validity_period(mut self, period: u8) -> Self {
        self.validity_period = Some(period);
        self
    }

    /// Set a request timeout value.
    #[must_use]
    pub fn with_timeout(mut self, timeout: u32) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Get the message sending validity period, either as set or default.
    /// Returns class 0 for a flash message.
    #[must_use]
    pub fn get_validity_period(&self) -> u8 {
        if self.flash.unwrap_or(false) {
            return 0;
        }
        self.validity_period.unwrap_or(167) // 24hr
    }
}
impl From<&SmsOutgoingMessage> for SmsMessage {
    fn from(outgoing: &SmsOutgoingMessage) -> Self {
        SmsMessage {
            message_id: None,
            phone_number: outgoing.to.clone(),
            message_content: outgoing.content.clone(),
            message_reference: None,
            is_outgoing: true,
            status: None,
            created_at: None,
            completed_at: None,
        }
    }
}

/// An incoming message from the Modem.
#[derive(Debug, Clone)]
pub struct SmsIncomingMessage {
    /// The incoming sender address. This could also be an alphanumeric sender name.
    /// This is usually for registered businesses or carrier messages.
    pub phone_number: String,

    /// The decoded multipart header.
    pub user_data_header: Option<SmsMultipartHeader>,

    /// The raw message content.
    pub content: String,
}
impl From<&SmsIncomingMessage> for SmsMessage {
    fn from(incoming: &SmsIncomingMessage) -> Self {
        SmsMessage {
            message_id: None,
            phone_number: incoming.phone_number.clone(),
            message_content: incoming.content.clone(),
            message_reference: None,
            is_outgoing: false,
            status: None,
            created_at: None,
            completed_at: None,
        }
    }
}

/// A received or stored delivery report.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct SmsDeliveryReport {
    /// Unique identifier for this delivery report.
    pub report_id: Option<i64>,

    /// Delivery status code from the network.
    pub status: u8,

    /// Whether this is the final delivery report for the message.
    pub is_final: bool,

    /// Unix timestamp when this report was created.
    pub created_at: Option<u32>,
}

/// A partial message delivery report, as it comes from the modem.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SmsPartialDeliveryReport {
    /// The target phone number that received the message (and has now sent back a delivery report).
    pub phone_number: String,
    /// The modem assigned message reference, this is basically useless outside short-term tracking
    /// the `message_id` is unique should always be used instead for identification.
    pub reference_id: u8,

    /// The SMS TP-Status: <https://www.etsi.org/deliver/etsi_ts/123000_123099/123040/16.00.00_60/ts_123040v160000p.pdf#page=71>
    pub status: u8,
}

/// A general category of status message delivery status reports.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum SmsDeliveryReportStatusCategory {
    /// The message has been sent, however not yet delivered.
    Sent,

    /// The message has been delivered.
    Received,

    /// The message has a temporary error, and sending will be retried by the carrier.
    Retrying,

    /// The message has a permanent error, the message will not be retried.
    Failed,
}
impl From<u8> for SmsDeliveryReportStatusCategory {
    fn from(value: u8) -> Self {
        match value {
            0x00 => SmsDeliveryReportStatusCategory::Received, // Received by SME
            0x01..=0x02 => SmsDeliveryReportStatusCategory::Sent, // Forwarded/Replaced
            0x03..=0x1F => SmsDeliveryReportStatusCategory::Sent, // Reserved/SC-specific success
            0x20..=0x3F => SmsDeliveryReportStatusCategory::Retrying,
            0x40..=0x6F => SmsDeliveryReportStatusCategory::Failed,
            _ => SmsDeliveryReportStatusCategory::Failed,
        }
    }
}
impl std::fmt::Display for SmsDeliveryReportStatusCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SmsDeliveryReportStatusCategory::Sent => "Sent",
            SmsDeliveryReportStatusCategory::Received => "Received",
            SmsDeliveryReportStatusCategory::Retrying => "Retrying",
            SmsDeliveryReportStatusCategory::Failed => "Failed",
        })
    }
}
impl From<&SmsDeliveryReport> for SmsDeliveryReportStatusCategory {
    fn from(value: &SmsDeliveryReport) -> Self {
        SmsDeliveryReportStatusCategory::from(value.status)
    }
}
impl From<&SmsPartialDeliveryReport> for SmsDeliveryReportStatusCategory {
    fn from(value: &SmsPartialDeliveryReport) -> Self {
        SmsDeliveryReportStatusCategory::from(value.status)
    }
}

/// The sms message multipart header.
#[derive(Debug, Clone, Copy)]
pub struct SmsMultipartHeader {
    /// Modem assigned message send reference (overflows).
    pub message_reference: u8,

    /// The total amount of messages within this multipart.
    pub total: u8,

    /// The current received message index.
    pub index: u8,
}
impl TryFrom<Vec<u8>> for SmsMultipartHeader {
    type Error = &'static str;

    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        if data.len() != 3 {
            return Err("Invalid user data length!");
        }
        Ok(Self {
            message_reference: data[0],
            total: data[1],
            index: data[2],
        })
    }
}
