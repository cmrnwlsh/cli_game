use crate::game::Game;
use anyhow::{Error, Result};
use crossterm::{
    event::{self, read, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use delegate::delegate;
use ratatui::{backend::CrosstermBackend, CompletedFrame, Frame, Terminal};
use std::io::{stdout, Stdout};

pub fn draw(game: &Game, interface: &mut Interface) -> Result<()> {
    interface
        .draw(|frame| frame.render_widget_ref(&game.context[0], frame.size()))
        .map(|_| ())
        .map_err(Error::from)
}

pub enum Event {
    Key(KeyEvent),
    Resize,
    None,
}

pub struct Interface(Terminal<CrosstermBackend<Stdout>>);

impl Interface {
    pub fn init() -> Result<Self> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        Ok(Interface(Terminal::new(CrosstermBackend::new(stdout()))?))
    }

    pub fn await_event(&self) -> Result<Event> {
        Ok(match read()? {
            event::Event::Key(k) => Event::Key(k),
            _ => Event::None,
        })
    }

    delegate! {
        to self.0 {
            fn draw<F>(&mut self, f: F) -> std::io::Result<CompletedFrame<'_>>
            where F: FnOnce(&mut Frame<'_>);

            pub fn autoresize(&mut self) -> std::io::Result<()>;
        }
    }
}

impl Drop for Interface {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }
}
