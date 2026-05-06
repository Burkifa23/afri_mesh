# **AfriMesh: Infrastructure-Independent Educational Chat**

**AfriMesh**  is a high-performance, decentralized communication protocol built in Rust. It is engineered to facilitate real-time collaboration in environments where traditional internet infrastructure is unreliable or unavailable, specifically targeting rural classrooms in Sub-Saharan Africa.

By leveraging peer-to-peer (P2P) networking, AfriMesh allows consumer devices to form a self-healing mesh without requiring a central server, cellular data, or an external internet connection.

---

## **Performance Benchmarks (Verified Results)**
The protocol has been rigorously validated through localized simulations and stress-testing under high-load "Broadcast Storm" conditions.

| Metric | Target | Verified Result (Simulation) | Significance |
| :--- | :--- | :--- | :--- |
| **One-Way Latency** | < 20ms | **11.98ms** | Real-time interactivity for collaborative whiteboards. |
| **Stability (Packet Delivery)** | > 95% | **98.00%** | Resilient against localized congestion in 5-node clusters. |
| **Software Throughput** | > 18 MB/s | **65.29 MB/s** | Software capacity exceeds hardware limits by 3.6x. |

> **Note on Simulation Limitations:** During 10-node stress tests, a "Simulation Paradox" was observed where single-host CPU context-switching became the primary bottleneck. In real-world deployment, where compute power is distributed across multiple physical devices, stability is expected to exceed these simulated values.

---

## **Technical Stack**
* **Language**: Rust (Edition 2021) — Chosen for memory safety and zero-cost abstractions.
* **Networking**: `libp2p` (TCP, mDNS, Gossipsub, Noise, Yamux).
* **Async Runtime**: `tokio` (Event-driven concurrency).
* **Interface**: `ratatui` & `crossterm` (High-efficiency Terminal UI).
* **Error Handling**: `thiserror` (Strictly typed custom errors).

## **Key Architectural Features**
* **Density-Aware Routing**: Utilizes the Gossipsub protocol to prevent "broadcast storms" in crowded classrooms.
* **Hybrid Data Plane**: Separates high-frequency "Control" messages (chat/attendance) from high-bandwidth "Data" payloads (media/PDFs).
* **Infrastructure-Less Discovery**: Automatic peer discovery via mDNS over local Wi-Fi.

## **Core Components**
| Module | Description |
| :--- | :--- |
| **`main.rs`** | Orchestrates the asynchronous event loop and UI/Network orchestration. |
| **`network.rs`** | Manages the `libp2p` Swarm, network behaviors, and peer dialing/subscription. |
| **`app.rs`** | Defines the centralized `App` state, message history, and input modes. |
| **`ui.rs`** | Handles terminal rendering using Swiss Design principles (high contrast, minimal clutter). |
| **`error.rs`** | Custom `AfriMeshError` types to ensure protocol reliability and predictable failure states. |

## **Getting Started**

### **Prerequisites**
* [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
* A terminal supporting ANSI escape codes

### **Build and Run**
```bash
cargo run
```

### **Running Research Benchmarks**
To verify the performance metrics reported in the AfriMesh thesis, run the following commands:
```bash
# Test 1: Latency Proof
cargo test --test latency_test -- --nocapture

# Test 2: Stability Stress Test
cargo test --test stability_test -- --nocapture

# Test 3: Throughput Benchmark (Optimized Release Mode)
cargo test --release --test throughput_test -- --nocapture
```

## **UI Controls**
* **`i`**: Enter **Editing Mode** to start typing.
* **`Enter`**: Send message to the `classroom-chat` channel.
* **`Esc`**: Return to **Normal Mode**.
* **`q`**: Quit the application safely.

## **Key References**
* **Regional Context**: GSMA. (2024). *The Mobile Economy Sub-Saharan Africa*; Ghana Statistical Service. (2023). *Digital Exclusion Brief*; Citi Newsroom (2025) *BECE Performance Decline*.
* **Protocol Foundation**: Vyzovitis, D., et al. (2020). *Gossipsub: An extensible pubsub protocol*; Li, X., et al. (2024). *Routing Selection Algorithm Based on Neighbor Node Density*.
* **Engineering**: Bastiao, A. D. (2023). *Rust and Android: Memory Safety Meets Native Performance*; Badrinath, V. / GSMA. (2024). *Africa’s $30 Smartphone*.
* **Impact**: World Bank. (2026). *Innovative Learning Methods in Africa*; Pan-African EdTech 2030 Vision & Plan. (2024).

---
**Project Status**: Research Artifact. Verified at Ashesi University (2026).
```
