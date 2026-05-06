// src/error.rs
use thiserror::Error;
use libp2p::swarm::ConnectionDenied;

#[derive(Error, Debug)]
pub enum AfriMeshError {
    #[error("Network initialization failed: {0}")]
    NetworkInit(#[from] libp2p::noise::Error),

    #[error("Failed to publish message: {0}")]
    PublishFailed(#[from] libp2p::gossipsub::PublishError),

    #[error("Failed to subscribe to topic: {0}")]
    SubscribeFailed(#[from] libp2p::gossipsub::SubscriptionError),

    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Connection Denied: {0}")]
    ConnectionDenied(#[from] ConnectionDenied),

    #[error("Channel send error: UI disconnected from Network")]
    ChannelClosed,
}

// RENAMED to avoid collision with std::result::Result
pub type AfriMeshResult<T> = std::result::Result<T, AfriMeshError>;