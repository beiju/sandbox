use std::collections::hash_map::Entry;
use std::collections::HashMap;
use anyhow::anyhow;
use fed::{FedEvent, FedEventData};
use uuid::Uuid;
use crate::chronicler_schema::{Player, Team};
use crate::game::Game;
use crate::rng::Rng;

#[derive(Debug)]
pub struct SimData {
    pub rng: Rng,
    pub teams: HashMap<Uuid, Team>,
    pub players: HashMap<Uuid, Player>,
}
#[derive(Debug)]
pub struct Sim {
    games: HashMap<Uuid, Game>,
    sim_data: SimData,
}

impl Sim {
    pub fn new(s0: u64, s1: u64, teams: HashMap<Uuid, Team>, players: HashMap<Uuid, Player>) -> Self {
        Self {
            games: Default::default(),
            sim_data: SimData {
                rng: Rng::new(s0, s1),
                teams,
                players,
            }
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

        let event_from_sim = game.tick(&mut self.sim_data)?;

        assert_eq!(event.data, event_from_sim);

        println!("Validated {} for game {}", event_from_sim.as_ref(), game_event.game_id);
        Ok(())
    }
}