//! SMS Server and Client shared types.

#![deny(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]

pub mod events;
pub mod modem;
pub mod sms;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "gnss")]
pub mod gnss;
