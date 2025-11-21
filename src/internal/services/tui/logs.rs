use ansi_to_tui::IntoText as _;
use std::borrow::Cow;

use ratatui::{
    prelude::*,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::internal::common::log_err;
use crate::internal::domain::records::Records;

pub struct Logs {
    records: Records,
    lines_len: usize,
    vertical_scroll: usize,
    vertical_scroll_state: ratatui::widgets::ScrollbarState,
    area_height: usize,
    auto_scroll: bool,
    show_scrollbar: bool,
}

impl Logs {
    pub fn new(records: Records) -> Self {
        Self {
            records,
            lines_len: 0,
            vertical_scroll: 0,
            vertical_scroll_state: ratatui::widgets::ScrollbarState::new(0),
            area_height: 0,
            auto_scroll: true,
            show_scrollbar: true,
        }
    }
    pub fn records(&self) -> &Records {
        &self.records
    }
    pub fn records_mut(&mut self) -> &mut Records {
        &mut self.records
    }
    fn bottom_scroll_amount(&self) -> Option<u32> {
        let amount = self.lines_len as i32 - self.area_height as i32 - 1;
        if amount >= 0 {
            return Some(amount as u32);
        }
        None
    }
    fn scroll_down_disabled(&self) -> bool {
        if let Some(max_scroll) = self.bottom_scroll_amount() {
            if self.vertical_scroll > max_scroll as usize {
                // we are already at the bottom
                return true;
            } else {
                return false;
            }
        }
        true
    }
    pub fn set_show_scrollbar(&mut self, val: bool) {
        self.show_scrollbar = val
    }
    pub fn set_auto_scroll(&mut self) {
        self.auto_scroll = true;
        self.scroll_to_bottom();
    }
    pub fn scroll_to_top(&mut self) {
        self.vertical_scroll = 0;
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }
    pub fn scroll_to_bottom(&mut self) {
        if let Some(max_scroll) = self.bottom_scroll_amount() {
            self.vertical_scroll = max_scroll as usize + 1;
            self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
        } else {
            self.scroll_to_top();
        }
    }
    pub fn scroll_down(&mut self) {
        if self.scroll_down_disabled() {
            return;
        }
        self.auto_scroll = false;
        self.vertical_scroll = self.vertical_scroll.saturating_add(1);
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }
    pub fn scroll_down_many(&mut self) {
        if self.scroll_down_disabled() {
            return;
        }
        self.auto_scroll = false;
        self.vertical_scroll = self.vertical_scroll.saturating_add(self.area_height);
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }
    pub fn scroll_up(&mut self) {
        self.auto_scroll = false;
        self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }
    pub fn scroll_up_many(&mut self) {
        self.auto_scroll = false;
        self.vertical_scroll = self.vertical_scroll.saturating_sub(self.area_height);
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }

    fn mark_result(&self, line: &mut Line) {
        if self.records.filter_key().is_empty() {
            return;
        }
        let text = &line.spans[0].content;
        if text.is_empty() {
            return;
        }

        let found_style = Style::new().bg(Color::Yellow).fg(Color::Black);
        let not_found_style = Style::new().white();
        let mut k = 0;
        let mut spans: Vec<Span<'_>> = vec![];
        let mut span_not_found = String::from("");
        let mut span_found = String::from("");

        for c in text.chars() {
            if c.to_ascii_lowercase()
                == self
                    .records
                    .filter_key()
                    .as_ref()
                    .to_lowercase()
                    .chars()
                    .nth(k)
                    .unwrap()
            {
                span_found.push(c);
                if k == (self.records.filter_key().as_ref().len() - 1) {
                    if !span_not_found.is_empty() {
                        spans.push(Span::styled(span_not_found.clone(), not_found_style));
                        span_not_found = String::from("");
                    }
                    if !span_found.is_empty() {
                        spans.push(Span::styled(span_found.clone(), found_style));
                        span_found = String::from("");
                    }
                    k = 0;
                } else {
                    k += 1;
                }
            } else {
                k = 0;
                if !span_found.is_empty() {
                    span_not_found.push_str(&span_found);
                    span_found = String::from("");
                }
                span_not_found.push(c);
            }
        }

        if !span_not_found.is_empty() {
            spans.push(Span::styled(span_not_found.clone(), not_found_style));
        }
        if !span_found.is_empty() {
            spans.push(Span::styled(span_found.clone(), not_found_style));
        }

        line.spans = spans
    }
}
impl Default for Logs {
    fn default() -> Self {
        Logs::new(Records::default())
    }
}
fn wrap(line: Line, width: usize) -> impl Iterator<Item = Line> {
    let mut line = line;
    std::iter::from_fn(move || {
        if line.width() > width {
            let (first, second) = line_split_at(line.clone(), width);
            line = second;
            Some(first)
        } else if line.width() > 0 {
            let first = line.clone();
            line = Line::default();
            Some(first)
        } else {
            None
        }
    })
}

fn span_split_at(span: Span, mid: usize) -> (Span, Span) {
    let (first, second) = span.content.split_at(mid);
    let first = Span {
        content: Cow::Owned(first.into()),
        style: span.style,
    };
    let second = Span {
        content: Cow::Owned(second.into()),
        style: span.style,
    };
    (first, second)
}

fn line_split_at(line: Line, mid: usize) -> (Line, Line) {
    let mut first = Line::default();
    let mut second = Line::default();
    first.alignment = line.alignment;
    second.alignment = line.alignment;
    for span in line.spans {
        let first_width = first.width();
        let span_width = span.width();
        if first_width + span_width <= mid {
            first.spans.push(span);
        } else if first_width < mid && first_width + span_width > mid {
            let span_mid = mid - first_width;
            let (span_first, span_second) = span_split_at(span, span_mid);
            first.spans.push(span_first);
            second.spans.push(span_second);
        } else {
            second.spans.push(span);
        }
    }
    (first, second)
}
impl ratatui::widgets::Widget for &mut Logs {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        self.area_height = area.height.into();

        // **MUST BE THE FIRST STEP**
        // Build lines and run autoscroll

        let mut lines = vec![];
        for record in self.records.iter() {
            // can't use let line = Line::from(record.to_string());
            // because ANSI is not parsed properly
            let text = record
                .to_string()
                .into_text()
                .unwrap_or(log_err("Error processing log").into());
            for line in text.lines {
                let wrapped_lines: Vec<_> = wrap(line, area.width as usize - 2).collect();
                for mut wl in wrapped_lines {
                    self.mark_result(&mut wl);
                    lines.push(wl);
                }
            }
        }

        self.lines_len = lines.len();

        if self.auto_scroll {
            self.scroll_to_bottom();
        }

        // Render paragraph

        Paragraph::new(lines)
            .style(Style::default().fg(Color::White))
            // .scroll((self.vertical_scroll as u16, 0))
            .render(area, buf);

        if self.show_scrollbar {
            // Render scroll

            let mut scroll_lines = 0;
            if self.lines_len > area.height.into() {
                scroll_lines = self.lines_len as i32 - self.area_height as i32 + 1;
            }

            self.vertical_scroll_state = self
                .vertical_scroll_state
                .content_length(scroll_lines as usize);

            let scroll_style = if self.auto_scroll {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default().fg(Color::Yellow)
            };

            ratatui::widgets::Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .begin_style(scroll_style)
                .end_symbol(Some("↓"))
                .end_style(scroll_style)
                .track_symbol(Some("."))
                .track_style(scroll_style)
                .thumb_symbol("▐")
                .thumb_style(scroll_style)
                .render(area, buf, &mut self.vertical_scroll_state);
        }
    }
}
