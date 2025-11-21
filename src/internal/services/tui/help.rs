use ratatui::{
    prelude::*,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph},
};

pub struct Help {}
impl Default for Help {
    fn default() -> Self {
        Help::new()
    }
}

impl Help {
    pub fn new() -> Self {
        Self {}
    }
}

impl ratatui::widgets::Widget for &mut Help {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let lines = vec![
            Line::from("Use Ctr +:"),
            Line::from(""),
            Line::from("c/q -> quit"),
            Line::from(""),
            Line::from("b -> toggle scrollbar"),
            Line::from("s -> toggle search input"),
            Line::from("h -> toggle help"),
            Line::from(""),
            Line::from("u/d -> scroll up/down (turns off autoScroll)"),
            Line::from("p/n -> page up/down (turns off autoScroll)"),
            Line::from(""),
            Line::from("a -> turn on autoScroll"),
            Line::from("t -> scroll to the top"),
        ];
        let block = Block::new()
            .borders(Borders::NONE)
            .padding(Padding::new(7, 0, 2, 1))
            .style(Style::default().bg(Color::Black));

        Paragraph::new(lines)
            .style(Style::default().fg(Color::Yellow))
            .block(block)
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}
