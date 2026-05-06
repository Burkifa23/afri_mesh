// src/network.rs
use libp2p::{
    gossipsub, mdns, noise, swarm::NetworkBehaviour, tcp, yamux, Swarm,
};
use std::time::Duration;
use crate::error::{AfriMeshError, AfriMeshResult}; // Use the renamed Result

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "EduEvent")]
pub struct EduBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

#[derive(Debug)]
pub enum EduEvent {
    Gossipsub(gossipsub::Event),
    Mdns(mdns::Event), // Fixed path
}

impl From<gossipsub::Event> for EduEvent {
    fn from(event: gossipsub::Event) -> Self {
        EduEvent::Gossipsub(event)
    }
}

impl From<mdns::Event> for EduEvent {
    fn from(event: mdns::Event) -> Self {
        EduEvent::Mdns(event)
    }
}

pub struct Network {
    pub swarm: Swarm<EduBehaviour>,
}

impl Network {
    pub async fn new() -> AfriMeshResult<Self> {
        let id_keys = libp2p::identity::Keypair::generate_ed25519();

        let swarm = libp2p::SwarmBuilder::with_existing_identity(id_keys)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )
            .map_err(|e| AfriMeshError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?
            .with_behaviour(|key| {
                let gossip_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(1))
                    .validation_mode(gossipsub::ValidationMode::Strict)
                    .build()
                    .map_err(|msg| std::io::Error::new(std::io::ErrorKind::Other, msg))?;

                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossip_config,
                ).map_err(|msg| std::io::Error::new(std::io::ErrorKind::Other, msg))?;

                let mdns = mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id()
                )?;

                Ok(EduBehaviour { gossipsub, mdns })
            })
            .map_err(|e| AfriMeshError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        Ok(Self { swarm })
    }

    pub fn subscribe(&mut self, topic_name: &str) -> AfriMeshResult<bool> {
        let topic = gossipsub::IdentTopic::new(topic_name);
        self.swarm.behaviour_mut().gossipsub.subscribe(&topic)
            .map_err(AfriMeshError::SubscribeFailed)
    }

    pub fn publish(&mut self, topic_name: &str, message: String) -> AfriMeshResult<gossipsub::MessageId> {
        let topic = gossipsub::IdentTopic::new(topic_name);
        self.swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic, message.as_bytes())
            .map_err(AfriMeshError::PublishFailed)
    }
}