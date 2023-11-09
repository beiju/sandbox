use std::collections::hash_map::Entry;
use std::collections::HashMap;
use anyhow::anyhow;
use fed::{FedEvent, FedEventData};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::chronicler_schema::{Player, Team};
use crate::game::Game;
use crate::rng::Rng;

#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    pub teams: HashMap<Uuid, Team>,
    pub players: HashMap<Uuid, Player>,
}

impl World {
    // pretty sure self and player_ids could have different lifetimes if needed
    pub fn iter_players<'a>(&'a self, player_ids: &'a [Uuid]) -> impl Iterator<Item=Option<&'a Player>> + 'a {
        player_ids.into_iter()
            .map(|player_id| self.players.get(player_id))
    }

    pub fn players_on_team(&self, team_id: Uuid) -> Option<impl Iterator<Item=Option<&Player>>> {
        let team = self.teams.get(&team_id)?;
        Some(self.iter_players(&team.lineup)
            .chain(self.iter_players(&team.rotation)))
    }

    pub fn any_player_on_team_has_mod(&self, team_id: Uuid, mod_name: &str) -> anyhow::Result<bool> {
        let players = self.players_on_team(team_id)
            .ok_or_else(|| anyhow!("Couldn't find batting team"))?;
        for player in players {
            let player = player.ok_or_else(|| anyhow!("Couldn't find player from team rotation or lineup"))?;
            if player.has_mod(mod_name) { return Ok(true) }
        }

        Ok(false)
    }
}

#[derive(Debug)]
pub struct Sim {
    games: HashMap<Uuid, Game>,
    world: World,
    rng: Rng,
}

impl Sim {
    pub fn new(s0: u64, s1: u64, world: World) -> Self {
        Self {
            games: Default::default(),
            world,
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

        let event_from_sim = game.tick(&mut self.world, &mut self.rng)?;

        assert_eq!(event.data, event_from_sim);

        println!("Validated {} for game {}", event_from_sim.as_ref(), game_event.game_id);
        Ok(())
    }
}