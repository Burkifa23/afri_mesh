
mod app;
mod network;
mod ui;

use app::{App, InputMode};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
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
    network.subscribe("classroom-chat"); // Listen to this channel
    network.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?; // Listen on any port

    // 2. Setup Channels (The phone lines between UI and Network)
    let (tx, mut rx) = mpsc::unbounded_channel::<String>(); // UI -> Network

    // 3. Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 4. Create App State
    let my_id = network.swarm.local_peer_id().to_string();
    let mut app = App::new(my_id.clone(), tx.clone());

    // 5. The Main Loop (The Heartbeat)
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = tokio::time::Instant::now();

    loop {
        // A. Draw UI
        terminal.draw(|f| ui::ui(f, &app))?;

        // B. Handle Network Events (Non-blocking)
        tokio::select! {
            // Case 1: UI wants to send a message
            Some(msg) = rx.recv() => {
                network.publish("classroom-chat", msg);
            }

            // Case 2: Network received something
            event = network.swarm.select_next_some() => {
                match event {
                    SwarmEvent::Behaviour(network::EduBehaviourEvent::Mdns(
                        libp2p::mdns::Event::Discovered(list)
                    )) => {
                        for (peer_id, multiaddr) in list {
                            let  _ = network.swarm.dial(multiaddr);

                            if !app.peers.contains(&peer_id.to_string()) {
                                app.peers.push(peer_id.to_string());
                            }
                        }
                    },
                    SwarmEvent::Behaviour(network::EduBehaviourEvent::Gossipsub(
                        gossipsub::Event::Message { propagation_source: peer, message, .. }
                    )) => {
                        let text = String::from_utf8_lossy(&message.data);
                        app.messages.push(format!("{}: {}", peer, text));
                    },
                    _ => {}
                }
            }

            // Case 3: Handle Keyboard Input
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
                                    // Send message to channel
                                    let msg = app.input_buffer.clone();
                                    app.messages.push(format!("Me: {}", msg));
                                    app.tx.send(msg).unwrap(); // Send to network thread
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

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen

    )?;
    terminal.show_cursor()?;

    Ok(())
}