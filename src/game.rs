use std::collections::HashMap;
use anyhow::anyhow;
use fed::{FedEventData, GameEvent, SubEvent, TogglePerforming, Weather};
use phf::phf_map;
use uuid::Uuid;
use crate::chronicler_schema::Team;
use crate::sim::SimData;

static ITEM_NAMES: phf::Map<&'static str, &'static str> = phf_map! {
    "AN_ACTUAL_AIRPLANE" => "An Actual Airplane",
};

#[derive(Debug)]
pub enum GamePhase {
    // TODO: Use Blarser to ensure this is in sync with proper game phases
    NotStarted = 0,
    Starting,
    StartOfHalfInning,
    SuperyummyAnnouncement,
    BatterUp,
}

#[derive(Debug)]
pub struct GameByTeam {
    pub team_id: Uuid,
    pub team_batter_count: i64,
}

impl GameByTeam {
    pub fn new(team_id: Uuid) -> Self {
        Self {
            team_id,
            team_batter_count: -1,
        }
    }
}

#[derive(Debug)]
pub struct Game {
    game_id: Uuid,
    home: GameByTeam,
    away: GameByTeam,
    weather: Weather,
    stadium_id: Option<Uuid>,

    phase: GamePhase,
    play: i64,
    top_of_inning: bool,
    inning: i32,
}

impl Game {
    pub fn new(game_event: &GameEvent, weather: Weather, stadium_id: Option<Uuid>) -> Self {
        Game {
            game_id: game_event.game_id,
            home: GameByTeam::new(game_event.home_team),
            away: GameByTeam::new(game_event.away_team),
            weather,
            stadium_id,
            phase: GamePhase::NotStarted,
            play: 0,
            // play starts at the "bottom of the 0th" so that the first half-inning-start moves us
            // to the top of the first. innings are zero-indexed so the "zeroth" is -1
            top_of_inning: false,
            inning: -1,
        }
    }

    fn game_event(&mut self) -> GameEvent {
        let result = GameEvent {
            game_id: self.game_id,
            home_team: self.home.team_id,
            away_team: self.away.team_id,
            play: self.play,
            unscatter: None,
            attractor_secret_base: None,
        };
        self.play += 1;
        result
    }

    pub fn tick(&mut self, sim_data: &mut SimData) -> anyhow::Result<FedEventData> {
        match self.phase {
            GamePhase::NotStarted => {
                Ok(self.lets_go())
            }
            GamePhase::Starting => {
                Ok(self.play_ball())
            }
            GamePhase::StartOfHalfInning => {
                self.start_half_inning(sim_data)
            }
            GamePhase::BatterUp => {
                self.batter_up(sim_data)
            }
            GamePhase::SuperyummyAnnouncement => {
                self.superyummy_announcement(sim_data)
            }
        }
    }

    fn lets_go(&mut self) -> FedEventData {
        self.phase = GamePhase::Starting;
        FedEventData::LetsGo {
            game: self.game_event(),
            weather: self.weather,
            stadium_id: self.stadium_id,
        }
    }

    fn play_ball(&mut self) -> FedEventData {
        self.phase = GamePhase::StartOfHalfInning;
        FedEventData::PlayBall {
            game: self.game_event()
        }
    }

    fn start_half_inning(&mut self, sim_data: &mut SimData) -> anyhow::Result<FedEventData> {
        if self.inning < 0 && (
            sim_data.any_player_on_team_has_mod(self.batting_team_game_data().team_id, "SUPERYUMMY")? ||
                sim_data.any_player_on_team_has_mod(self.pitching_team_game_data().team_id, "SUPERYUMMY")?
        ) {
            self.phase = GamePhase::SuperyummyAnnouncement;
        } else {
            self.phase = GamePhase::BatterUp;
        }
        self.top_of_inning = !self.top_of_inning;
        self.inning += 1;
        Ok(FedEventData::HalfInningStart {
            game: self.game_event(),
            top_of_inning: self.top_of_inning,
            inning: self.inning + 1, // one-indexed
            batting_team_name: self.batting_team(sim_data)
                .ok_or_else(|| anyhow!("Couldn't find batting team"))?
                .full_name.clone(),
            subseasonal_mod_effects: vec![], // TODO
        })
    }

    fn batting_team<'a>(&self, sim_data: &'a SimData) -> Option<&'a Team> {
        sim_data.teams.get(&self.batting_team_game_data().team_id)
    }

    fn batting_team_game_data(&self) -> &GameByTeam {
        if self.top_of_inning { &self.away } else { &self.home }
    }

    fn pitching_team_game_data(&self) -> &GameByTeam {
        if self.top_of_inning { &self.home } else { &self.away }
    }

    fn batting_team_game_data_mut(&mut self) -> &mut GameByTeam {
        if self.top_of_inning { &mut self.away } else { &mut self.home }
    }

    fn batter_up(&mut self, sim_data: &mut SimData) -> anyhow::Result<FedEventData> {
        // self.phase = GamePhase::BatterUp;
        self.batting_team_game_data_mut().team_batter_count += 1;
        let team = self.batting_team(sim_data)
            .ok_or_else(|| anyhow!("Couldn't find batting team"))?;
        let batter_id = team.lineup[(self.batting_team_game_data().team_batter_count as usize) % team.lineup.len()];
        let batter = sim_data.players.get(&batter_id)
            .ok_or_else(|| anyhow!("Couldn't find batter"))?;
        Ok(FedEventData::BatterUp {
            game: self.game_event(),
            batter_name: batter.name.to_owned(),
            team_nickname: team.nickname.to_owned(),
            wielding_item: batter.bat.as_ref().map(|bat| {
                Ok::<_, anyhow::Error>(if !bat.is_empty() {
                    Some(ITEM_NAMES.get(bat)
                        .ok_or_else(|| anyhow!("Unknown item name. Note: Item names are in a hard-coded list, maybe it needs to be added?"))?
                        .to_string())
                } else {
                    None
                })
            }).transpose()?.flatten(),
            inhabiting: None,
            is_repeating: false,
        })
    }

    fn superyummy_announcement(&mut self, sim_data: &mut SimData) -> anyhow::Result<FedEventData> {
        self.phase = GamePhase::BatterUp;

        let (player, team_id) = (|| {
            for team_id in [self.batting_team_game_data().team_id, self.pitching_team_game_data().team_id] {
                for player in sim_data.players_on_team(team_id)
                    .ok_or_else(|| anyhow!("Couldn't find batting/pitching team"))? {
                    let player = player.ok_or_else(|| anyhow!("Couldn't find player from team rotation or lineup"))?;
                    if player.has_mod("SUPERYUMMY") { return Ok((player, team_id)); }
                }
            }
            Err(anyhow!("Got to state SuperyummyAnnouncement, but no players in this game are Superyummy"))
        })()?;

        Ok(FedEventData::SuperyummyGameStart {
            game: self.game_event(),
            toggle: TogglePerforming {
                player_id: player.id,
                team_id,
                player_name: player.name.to_owned(),
                is_overperforming: false, // TODO this should look at the weather
                is_first_proc: true, // TODO This should read whether the player has over/underperforming already
                sub_event: SubEvent::nil(),
            },
        })
    }
}