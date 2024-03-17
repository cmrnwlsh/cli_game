use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use game::Game;
use interface::{draw, Event, Interface};

mod component;
mod context;
mod game;
mod interface;

fn main() -> Result<()> {
    let (mut game, mut interface) = (Game::new(), Interface::init()?);
    draw(&game, &mut interface)?;
    loop {
        match interface.await_event()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => break Ok(()),
            Event::Key(k) => {
                game.update(k)?;
                draw(&game, &mut interface)?;
            }
            Event::Resize => interface.autoresize()?,
            Event::None => (),
        }
    }
}
