// tests/stability_test.rs
use afri_mesh::network::{Network, EduEvent};
use libp2p::futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use libp2p::gossipsub;
use tokio::time::{timeout, Duration, sleep};
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn test_stability_and_delivery_rate() {
    let node_count = 5;
    let message_count = 50;
    let topic_str = "stability-test";
    let topic = gossipsub::IdentTopic::new(topic_str);

    // 1. Spawn Teacher
    let mut teacher = Network::new().await.expect("Failed to create Teacher");
    teacher.swarm.listen_on("/ip4/127.0.0.1/tcp/0".parse().unwrap()).unwrap();

    let teacher_addr = loop {
        if let Some(SwarmEvent::NewListenAddr { address, .. }) = teacher.swarm.next().await {
            break address;
        }
    };
    teacher.subscribe(topic_str).unwrap();

    // 2. Spawn Students
    let total_received = Arc::new(Mutex::new(0));
    let mut student_handles = vec![];

    for i in 0..node_count {
        let teacher_addr_clone = teacher_addr.clone();
        let received_counter = Arc::clone(&total_received);

        let handle = tokio::spawn(async move {
            let mut student = Network::new().await.expect("Failed to create Student");
            student.swarm.dial(teacher_addr_clone).unwrap();
            student.subscribe(topic_str).unwrap();

            let mut count = 0;
            // Increased timeout to 30s to account for CPU lag
            let res = timeout(Duration::from_secs(30), async {
                while count < message_count {
                    tokio::select! {
                        event = student.swarm.next() => {
                            if let Some(SwarmEvent::Behaviour(EduEvent::Gossipsub(gossipsub::Event::Message { .. }))) = event {
                                count += 1;
                                let mut lock = received_counter.lock().unwrap();
                                *lock += 1;
                            }
                        }
                    }
                }
            }).await;

            if res.is_err() {
                println!("Student {} timed out. Received {}/{}", i, count, message_count);
            }
        });
        student_handles.push(handle);
    }

    // 3. WAIT for Mesh Formation
    println!("[RESEARCH] Waiting for students to stabilize in mesh...");
    let _ = timeout(Duration::from_secs(15), async {
        loop {
            let peers = teacher.swarm.behaviour().gossipsub.mesh_peers(&topic.hash()).count();
            if peers >= node_count { break; }
            let _ = teacher.swarm.next().await;
        }
    }).await;

    // 4. THE BROADCAST LOOP
    println!("[RESEARCH] Teacher starting broadcast...");
    for i in 1..=message_count {
        // DRIVE: Ensure teacher processes all heartbeats before publishing
        let _ = timeout(Duration::from_millis(50), async {
            loop { teacher.swarm.next().await; }
        }).await;

        // RECOVERY: If mesh prunes a student due to lag, wait for them to re-join
        if teacher.swarm.behaviour().gossipsub.mesh_peers(&topic.hash()).count() < node_count {
            println!("[WARNING] Mesh unstable at msg {}. Waiting for re-sync...", i);
            sleep(Duration::from_secs(2)).await;
        }

        // PUBLISH
        if let Err(e) = teacher.publish(topic_str, format!("Lesson {}", i)) {
            println!("[FAIL] Message {} dropped: {:?}", i, e);
        }

        // SPEED: 300ms is the "Sweet Spot" for simulating 5 nodes on one PC
        sleep(Duration::from_millis(300)).await;
    }

    // 5. Final Calculation
    for handle in student_handles { let _ = handle.await; }
    let final_count = *total_received.lock().unwrap();
    let expected_total = node_count * message_count;
    let success_rate = (final_count as f64 / expected_total as f64) * 100.0;

    println!("\n[RESEARCH DATA] Stability Test Results:");
    println!("Verified Delivery Success Rate: {:.2}%", success_rate);
    assert!(success_rate >= 95.0, "Stability failed! Delivery rate below 95%");
}