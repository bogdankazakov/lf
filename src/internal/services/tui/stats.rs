use ratatui::{
    prelude::*,
    style::{Color, Style},
    widgets::{Block, Padding, Paragraph},
};

pub struct Stats {
    total: u32,
    current: u32,
}
impl Default for Stats {
    fn default() -> Self {
        Stats::new()
    }
}

impl Stats {
    pub fn new() -> Self {
        Self {
            total: 0,
            current: 0,
        }
    }
    pub fn set(&mut self, total: u32, current: u32) {
        self.total = total;
        self.current = current;
    }
}

impl ratatui::widgets::Widget for &mut Stats {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered()
            .padding(Padding::new(0, 1, 0, 0))
            .title("Found")
            .style(Color::DarkGray);

        Paragraph::new(format!("{}/{}", self.current, self.total))
            .style(Style::default().fg(Color::DarkGray))
            .block(block)
            .alignment(Alignment::Right)
            .render(area, buf);
    }
}
