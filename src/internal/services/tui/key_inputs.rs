use crate::Event;

#[allow(clippy::single_match)]
pub fn handle_key_inputs(tx: std::sync::mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => tx.send(Event::KeyInput(key_event)).unwrap(),
            _ => {}
        }
    }
}
