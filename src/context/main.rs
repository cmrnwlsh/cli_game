use anyhow::Result;
use crossterm::event::KeyEvent;
use delegate::delegate;
use hecs::World;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Widget, WidgetRef},
};

use super::Behavior;

pub enum Main {
    Entry(Entry),
}

impl Behavior for Main {
    delegate! {
        to match self {
            Main::Entry(e) => e
        } {
            fn update(&mut self, world: &mut World, key: KeyEvent) -> Result<()>;
        }
    }
}

impl WidgetRef for Main {
    delegate! {
        to match self {
            Main::Entry(e) => e
        } {
            fn render_ref(&self, area: Rect, buf: &mut Buffer);
        }
    }
}

pub struct Entry();
impl Entry {
    fn update(&mut self, world: &mut World, key: KeyEvent) -> Result<()> {
        Ok(())
    }
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        Line::raw("hello world").render(area, buf);
    }
}
