use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, InputMode};

pub fn ui(f: &mut Frame, app: &App) {
    // 1. Create the layout
    // Split screen into 3 chunks: Header, Main Content, Input Bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Header
                Constraint::Min(1),    // Main Content (Messages + Peers)
                Constraint::Length(3), // Input Bar
            ]
                .as_ref(),
        )
        .split(f.size());

    // 2. Draw Header
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start typing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to send message."),
            ],
            Style::default(),
        ),
    };

    let mut text = Text::from(Line::from(msg));
    text = text.patch_style(style);

    // HERE WE USE 'my_id' (Warning fixed!)
    let title = format!(" Rust Africa Mesh - Node: {} ", app.my_id);

    let header = Paragraph::new(text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(header, chunks[0]);

    // 3. Main Content (Split into Messages vs Peers)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(chunks[1]);

    // Draw Messages
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = Line::from(Span::raw(format!("{}: {}", i, m)));
            ListItem::new(content)
        })
        .collect();

    let messages_list = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title(" Class Chat "));
    f.render_widget(messages_list, main_chunks[0]);

    // Draw Peers (HERE WE USE 'peers' - Warning fixed!)
    let peers: Vec<ListItem> = app
        .peers
        .iter()
        .map(|p| ListItem::new(Line::from(Span::raw(p))))
        .collect();

    let peers_list = List::new(peers)
        .block(Block::default().borders(Borders::ALL).title(" Connected Students "));
    f.render_widget(peers_list, main_chunks[1]);

    // 4. Draw Input Bar
    let input = Paragraph::new(app.input_buffer.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title(" Input "));
    f.render_widget(input, chunks[2]);

    // Cursor handling
    match app.input_mode {
        InputMode::Normal =>
        // Hide cursor
            {},
        InputMode::Editing => {
            // Make the cursor visible at the end of the text
            f.set_cursor(
                chunks[2].x + app.input_buffer.len() as u16 + 1,
                chunks[2].y + 1,
            )
        }
    }
}