use libp2p::{
    gossipsub, mdns, noise, swarm::NetworkBehaviour, tcp, yamux, PeerId, Swarm,
};
use std::{error::Error, time::Duration};

// --- 1. Define the "Behaviour" ---
#[derive(NetworkBehaviour)]
pub struct EduBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

// --- 2. The Network Manager ---
pub struct Network {
    pub swarm: Swarm<EduBehaviour>,
}

impl Network {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let id_keys = libp2p::identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(id_keys.public());
        // REMOVED println! - The UI header will show this instead.

        let swarm = libp2p::SwarmBuilder::with_existing_identity(id_keys)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_behaviour(|key| {
                let gossip_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(1))
                    .validation_mode(gossipsub::ValidationMode::Strict)
                    .build()
                    .map_err(|msg| std::io::Error::new(std::io::ErrorKind::Other, msg))?;

                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossip_config,
                )?;

                let mdns = mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id()
                )?;

                Ok(EduBehaviour { gossipsub, mdns })
            })?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        Ok(Self { swarm })
    }

    pub fn subscribe(&mut self, topic_name: &str) {
        let topic = gossipsub::IdentTopic::new(topic_name);
        // CHANGED: We silently ignore errors to keep the TUI clean.
        // In a real app, you would send this error to a log file.
        let _ = self.swarm.behaviour_mut().gossipsub.subscribe(&topic);
    }

    pub fn publish(&mut self, topic_name: &str, message: String) {
        let topic = gossipsub::IdentTopic::new(topic_name);
        // CHANGED: Silently ignore errors.
        let _ = self
            .swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic, message.as_bytes());
    }
}