use std::collections::VecDeque;

use crate::context::{Behavior, Context, Entry, Main};
use anyhow::Result;
use crossterm::event::KeyEvent;
use hecs::World;

pub struct Game {
    pub world: World,
    pub context: VecDeque<Context>,
}

impl Game {
    pub fn new() -> Game {
        Self {
            world: World::new(),
            context: [Main::Entry(Entry()).into()].into(),
        }
    }

    pub fn update(&mut self, key: KeyEvent) -> Result<()> {
        self.context[0].update(&mut self.world, key)
    }
}
