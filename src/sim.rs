use std::collections::hash_map::Entry;
use std::collections::HashMap;
use anyhow::anyhow;
use fed::{FedEvent, FedEventData};
use fed::FedEventDataStructure::LetsGo;
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

        let game = match self.games.entry(game_event.game_id) {
            Entry::Occupied(entry) => { entry.into_mut() }
            Entry::Vacant(entry) => if let FedEventData::LetsGo { game, weather, stadium_id } = &event.data {
                entry.insert(Game::new(game, *weather, *stadium_id))
            } else {
                return Err(anyhow!("First event for game was not a LetsGo event"))
            }
        };

        let event_from_sim = game.tick(&mut self.rng);

        assert_eq!(event.data, event_from_sim);

        println!("Validated {} for game {}", event_from_sim.as_ref(), game_event.game_id);
        Ok(())
    }
}