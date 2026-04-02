# **AfriMesh: Infrastructure-Independent Educational Chat**

**AfriMesh** (internal name: `edu_mesh`) is a high-performance, decentralized communication tool built in Rust. It is designed to facilitate real-time collaboration in environments where traditional internet infrastructure is unreliable or unavailable, such as rural classrooms in Sub-Saharan Africa. 

By leveraging peer-to-peer (P2P) networking, the application allows devices to discover each other and exchange messages over a local mesh without requiring a central server or an active internet connection.

## **Key Features**
* **Infrastructure-Less Discovery**: Automatically finds other students on the local network using **mDNS**.
* **Decentralized Messaging**: Utilizes the **Gossipsub** protocol for resilient, peer-to-peer message propagation.
* **Performance-Oriented UI**: A sleek Terminal User Interface (TUI) built with **Ratatui**, designed to run on resource-constrained hardware.
* **Memory Safe Architecture**: Core networking logic is written in **Rust**, ensuring stability and safety across diverse device types.

## **Technical Stack**
* **Language**: Rust (Edition 2021)
* **Networking**: `libp2p` (TCP, mDNS, Gossipsub, Noise, Yamux)
* **Async Runtime**: `tokio`
* **Interface**: `ratatui` & `crossterm`
* **State Management**: Unbounded MPSC channels for UI-to-Network communication

## **Core Components**
The repository is organized into four main modules within the `src` directory:

| Module | Description |
| :--- | :--- |
| **`main.rs`** | The application entry point. Orchestrates the asynchronous event loop, handling network events, terminal rendering, and user input concurrently. |
| **`network.rs`** | Manages the `libp2p` Swarm. Defines the network behavior (mDNS for discovery and Gossipsub for pub/sub) and handles peer dialing/subscription logic. |
| **`app.rs`** | Defines the centralized `App` state, including the message history, connected peer list, and current input mode (Normal vs. Editing). |
| **`ui.rs`** | Handles the terminal rendering. Divided into three sections: a header showing the Node ID, a split main view for messages and peers, and a yellow-themed input bar. |

## **Prerequisites**
To build and run this project, you must have the following installed:
* [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
* A terminal supporting ANSI escape codes

## **Getting Started**

1.  **Clone the Repository**
    ```bash
    git clone https://github.com/[your-username]/afri_mesh.git
    cd afri_mesh
    ```

2.  **Build and Run**
    Run the application using Cargo. The first run will download the necessary dependencies (libp2p, tokio, etc.).
    ```bash
    cargo run
    ```

3.  **Local Testing**
    Open multiple terminal windows on the same machine and run `cargo run` in each. They will automatically discover each other via mDNS and join the shared `classroom-chat` channel.

## **UI Controls**
* **`i`**: Enter **Editing Mode** to start typing a message.
* **`Enter`**: Send your message to all connected peers.
* **`Esc`**: Return to **Normal Mode**.
* **`q`**: Quit the application safely.

## **Project Structure**
```text
afri_mesh/
├── src/
│   ├── app.rs      # Application state management
│   ├── main.rs     # Event loop and orchestration
│   ├── network.rs  # P2P networking logic
│   └── ui.rs       # Terminal UI rendering
├── Cargo.toml      # Dependency definitions
└── README.md
```

---

**Current Project Status**: Research-ready Prototype. Optimized for local network discovery and real-time text synchronization.
