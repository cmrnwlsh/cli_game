mod main;

use anyhow::Result;
use crossterm::event::KeyEvent;
use delegate::delegate;
use enum_dispatch::enum_dispatch;
use hecs::World;
use ratatui::{buffer::Buffer, layout::Rect, widgets::WidgetRef};

pub use self::main::*;

#[enum_dispatch]
pub trait Behavior {
    fn update(&mut self, world: &mut World, key: KeyEvent) -> Result<()>;
}

#[enum_dispatch(Behavior)]
pub enum Context {
    Main(Main),
}

impl WidgetRef for &Context {
    delegate! {
        to match self {
            Context::Main(inner) => inner
        } {
            fn render_ref(&self, area: Rect, buf: &mut Buffer);
        }
    }
}
