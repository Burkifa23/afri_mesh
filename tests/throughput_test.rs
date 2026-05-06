// tests/throughput_test.rs
use afri_mesh::network::{Network, EduEvent};
use libp2p::futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use libp2p::gossipsub;
use std::time::Instant;
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn test_throughput_performance() {
    let mut node_a = Network::new().await.expect("Failed to create Node A");
    let mut node_b = Network::new().await.expect("Failed to create Node B");

    let b_addr = "/ip4/127.0.0.1/tcp/0".parse().unwrap();
    node_b.swarm.listen_on(b_addr).unwrap();

    let addr = loop {
        if let Some(SwarmEvent::NewListenAddr { address, .. }) = node_b.swarm.next().await {
            break address;
        }
    };

    node_a.swarm.dial(addr).expect("Node A failed to dial Node B");
    let topic_str = "throughput-channel";
    let ident_topic = gossipsub::IdentTopic::new(topic_str);

    node_a.subscribe(topic_str).unwrap();
    node_b.subscribe(topic_str).unwrap();

    // --- MESH SYNCHRONIZER: Wait for readiness ---
    println!("[RESEARCH] Synchronizing mesh for large payload transfer...");
    let _ = timeout(Duration::from_secs(10), async {
        loop {
            tokio::select! {
                event = node_a.swarm.next() => {
                    if let Some(SwarmEvent::Behaviour(EduEvent::Gossipsub(_))) = event {
                        if node_a.swarm.behaviour().gossipsub.mesh_peers(&ident_topic.hash()).next().is_some() {
                            return;
                        }
                    }
                }
                _ = node_b.swarm.next() => {}
            }
        }
    }).await;

    // 3. Prepare the 20MB Payload
    let file_size_mb = 20;
    let data_payload = vec![0u8; file_size_mb * 1024 * 1024];
    println!("[RESEARCH] Starting transfer of {}MB payload...", file_size_mb);

    // 4. Benchmark - Send as raw bytes to bypass UTF-8 validation
    let start_time = Instant::now();

    // We access the behaviour directly to send a raw message
    node_a.swarm.behaviour_mut().gossipsub
        .publish(ident_topic.hash(), data_payload.clone())
        .expect("Publish failed: Insufficient peers or message size limit hit");

    let result = timeout(Duration::from_secs(15), async {
        loop {
            tokio::select! {
                event = node_b.swarm.next() => {
                    if let Some(SwarmEvent::Behaviour(EduEvent::Gossipsub(gossipsub::Event::Message { message, .. }))) = event {
                        if message.data.len() >= data_payload.len() {
                            return Instant::now().duration_since(start_time);
                        }
                    }
                }
                _ = node_a.swarm.next() => {}
            }
        }
    }).await;

    match result {
        Ok(duration) => {
            let seconds = duration.as_secs_f64();
            let mb_per_second = file_size_mb as f64 / seconds;

            println!("\n[RESEARCH DATA] Throughput Test Results:");
            println!("Payload Size: {} MB", file_size_mb);
            println!("Transfer Time: {:.4} seconds", seconds);
            println!("Verified Software Throughput: {:.2} MB/s", mb_per_second);

            assert!(mb_per_second > 18.0, "Software bottleneck detected!");
        },
        Err(_) => panic!("Throughput test timed out."),
    }
}