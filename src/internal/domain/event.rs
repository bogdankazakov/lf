use super::record::Record;

#[derive(Debug)]
pub enum Event {
    StdIn(Record),
    KeyInput(crossterm::event::KeyEvent),
}
