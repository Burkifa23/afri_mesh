// src/main.rs
use afri_mesh::app::{App, InputMode};
use afri_mesh::network;
use afri_mesh::ui;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::StreamExt;
use libp2p::{gossipsub, swarm::SwarmEvent};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};
use tokio::{sync::mpsc, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. Setup Network
    let mut network = network::Network::new().await?;
    network.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // 2. Setup Channels (The phone lines between UI and Network)
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // 3. Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 4. Create App State
    let my_id = network.swarm.local_peer_id().to_string();
    let mut app = App::new(my_id.clone(), tx.clone());

    // --- FIX 1: Handle Subscribe Result after App is created ---
    if let Err(e) = network.subscribe("classroom-chat") {
        app.messages.push(format!("SYSTEM ERROR: Failed to join channel: {}", e));
    }

    // 5. The Main Loop
    let tick_rate = Duration::from_millis(100);

    loop {
        terminal.draw(|f| ui::ui(f, &app))?;

        tokio::select! {
            // --- FIX 2: Corrected rx.recv() location and syntax ---
            Some(msg) = rx.recv() => {
                if let Err(e) = network.publish("classroom-chat", msg) {
                    app.messages.push(format!("SYSTEM ERROR: Message failed to send: {}", e));
                }
            }

            event = network.swarm.select_next_some() => {
                match event {
                    SwarmEvent::Behaviour(network::EduEvent::Mdns(
                        libp2p::mdns::Event::Discovered(list)
                    )) => {
                        for (peer_id, multiaddr) in list {
                            let  _ = network.swarm.dial(multiaddr);

                            if !app.peers.contains(&peer_id.to_string()) {
                                app.peers.push(peer_id.to_string());
                            }
                        }
                    },
                    SwarmEvent::Behaviour(network::EduEvent::Gossipsub(
                        gossipsub::Event::Message { propagation_source: peer, message, .. }
                    )) => {
                        let text = String::from_utf8_lossy(&message.data);
                        app.messages.push(format!("{}: {}", peer, text));
                    },
                    _ => {}
                }
            }

            _ = tokio::time::sleep(tick_rate) => {
                 if event::poll(Duration::from_millis(0))? {
                    if let Event::Key(key) = event::read()? {
                        if key.kind != KeyEventKind::Press {
                            continue;
                        }
                        match app.input_mode {
                            InputMode::Normal => match key.code {
                                KeyCode::Char('q') => app.should_quit = true,
                                KeyCode::Char('i') => app.input_mode = InputMode::Editing,
                                _ => {}
                            },
                            InputMode::Editing => match key.code {
                                KeyCode::Enter => {
                                    let msg = app.input_buffer.clone();
                                    app.messages.push(format!("Me: {}", msg));

                                    // --- FIX 3: Safe channel sending ---
                                    if let Err(_) = app.tx.send(msg) {
                                        app.messages.push("SYSTEM ERROR: Network thread disconnected.".to_string());
                                    }

                                    app.input_buffer.clear();
                                    app.input_mode = InputMode::Normal;
                                }
                                KeyCode::Char(c) => app.input_buffer.push(c),
                                KeyCode::Backspace => { app.input_buffer.pop(); }
                                KeyCode::Esc => app.input_mode = InputMode::Normal,
                                _ => {}
                            },
                        }
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}