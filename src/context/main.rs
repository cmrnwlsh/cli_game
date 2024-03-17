use anyhow::Result;
use delegate::delegate;
use hecs::World;
use ratatui::{buffer::Buffer, layout::Rect, widgets::WidgetRef};

use super::Behavior;

pub enum Main {
    Entry(Entry),
}

impl Behavior for Main {
    delegate! {
        to match self {
            Main::Entry(e) => e
        } {
            fn update(&mut self, world: &mut World) -> Result<()>;
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
    fn update(&mut self, world: &mut World) -> Result<()> {
        Ok(())
    }
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {}
}
