
use std::sync::mpsc::{sync_channel, SyncSender, Receiver,};

mod entities;
mod combat;
mod worldpool;

pub use position::Pos;
use self::combat::*;
pub use self::worldpool::WorldPool;

pub type WorldState = Vec<(Pos, u8)>;

pub struct World {
    world: WorldPool,
    state_output: SyncSender<WorldState>,
}

impl World {
    const BUFFER: usize = 10;

    pub fn new(world: WorldPool) -> (World, Receiver<WorldState>) {
        let (state_output, receiver) = sync_channel(Self::BUFFER);

        (Self { world, state_output }, receiver)
    }
    pub fn update(&mut self) {
        self.world.update();
        self.state_output.send(
            self.world.iter()
            .map(|(pos, entity)| (pos.clone(), entity.as_byte()))
            .collect()
        ).ok();
    }
}
