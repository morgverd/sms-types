//! SMS Server and Client shared types.

#![deny(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]

pub mod modem;
pub mod sms;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "websocket")]
pub mod websocket;

#[cfg(feature = "gnss")]
pub mod gnss;
