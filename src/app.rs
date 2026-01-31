use tokio::sync::mpsc::UnboundedSender;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub should_quit: bool,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub messages: Vec<String>,
    pub peers: Vec<String>,
    pub my_id: String,
    pub tx: UnboundedSender<String>,
}

impl App {
    pub fn new(my_id: String, tx: UnboundedSender<String>) -> Self {
        Self {
            should_quit: false,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            messages: Vec::new(),
            peers: Vec::new(),
            my_id,
            tx
        }
    }

    pub fn on_tick(&mut self) {

    }
}