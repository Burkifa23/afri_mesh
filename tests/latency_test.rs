// tests/latency_test.rs
use afri_mesh::network::{Network, EduEvent};
use libp2p::futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use libp2p::gossipsub;
use std::time::Instant;
use tokio::time::{timeout, Duration}; // Cleaned up unused 'sleep' import

#[tokio::test]
async fn test_latency_proof() {
    // 1. Initialize nodes
    let mut node_a = Network::new().await.expect("Failed to create Node A");
    let mut node_b = Network::new().await.expect("Failed to create Node B");

    let b_addr = "/ip4/127.0.0.1/tcp/0".parse().unwrap();
    node_b.swarm.listen_on(b_addr).unwrap();

    let addr = loop {
        if let Some(SwarmEvent::NewListenAddr { address, .. }) = node_b.swarm.next().await {
            break address;
        }
    };

    // 2. Connect and Subscribe
    node_a.swarm.dial(addr).expect("Node A failed to dial Node B");
    let topic = "test-topic";
    let ident_topic = gossipsub::IdentTopic::new(topic); // Pre-calculate topic

    node_a.subscribe(topic).unwrap();
    node_b.subscribe(topic).unwrap();

    // 3. WAIT for Mesh Formation
    let mesh_ready = timeout(Duration::from_secs(10), async {
        loop {
            tokio::select! {
                event = node_a.swarm.next() => {
                    if let Some(SwarmEvent::Behaviour(EduEvent::Gossipsub(gossipsub::Event::Subscribed { .. }))) = event {
                        // FIX: Use .hash() to get TopicHash, and .next().is_some() to check the Iterator
                        if node_a.swarm.behaviour().gossipsub.mesh_peers(&ident_topic.hash()).next().is_some() {
                            return;
                        }
                    }
                }
                _ = node_b.swarm.next() => {}
            }
        }
    }).await;

    assert!(mesh_ready.is_ok(), "Timed out waiting for mesh peers to connect.");

    // 4. The Benchmark
    let start_time = Instant::now();
    let test_msg = "latency_ping".to_string();

    node_a.publish(topic, test_msg).expect("Publish failed even with peers");

    // 5. Wait for Node B to receive
    let result = timeout(Duration::from_secs(5), async {
        loop {
            tokio::select! {
                event = node_b.swarm.next() => {
                    if let Some(SwarmEvent::Behaviour(EduEvent::Gossipsub(gossipsub::Event::Message { message, .. }))) = event {
                        if String::from_utf8_lossy(&message.data) == "latency_ping" {
                            return Instant::now().duration_since(start_time);
                        }
                    }
                }
                _ = node_a.swarm.next() => {}
            }
        }
    }).await;

    match result {
        Ok(latency) => {
            println!("\n[RESEARCH DATA] Verified One-Way Latency: {:?}", latency);
            assert!(latency < Duration::from_millis(50), "Latency too high for local loopback!");
        },
        Err(_) => panic!("Test timed out: Node B never received the payload."),
    }
}