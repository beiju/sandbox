use std::collections::HashMap;
use fed::FedEvent;
use uuid::Uuid;
use crate::game::Game;
use crate::rng::Rng;

#[derive(Debug)]
pub struct Sim {
    games: HashMap<Uuid, Game>,
    rng: Rng,
}

impl Sim {
    pub fn new(s0: u64, s1: u64) -> Self {
        Self {
            games: Default::default(),
            rng: Rng::new(s0, s1),
        }
    }

    pub fn check_next_event(&mut self, event: &FedEvent) -> anyhow::Result<()> {
        let Some(game_event) = event.data.game() else {
            return Ok(())
        };

        let game = self.games.entry(game_event.game_id).or_insert(Game::new());

        let event_from_sim = game.tick(&mut self.rng);

        assert_eq!(event.data, event_from_sim);
        Ok(())
    }
}