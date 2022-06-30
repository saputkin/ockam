mod base;
mod forwarder;
mod portal;
mod secure_channel;
mod service;
mod transport;

/// Messaging types for the node manager service
///
/// This module is only a type facade and should not have any logic of
/// its own
pub mod types {
    pub use super::base::*;
    pub use super::forwarder::*;
    pub use super::portal::*;
    pub use super::secure_channel::*;
    pub use super::transport::*;
}

/// The main node-manager service running on remote nodes
pub use service::NodeMan;
