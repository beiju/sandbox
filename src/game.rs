use anyhow::anyhow;
use fed::{FedEventData, GameEvent, Weather};
use uuid::Uuid;
use crate::rng::Rng;
use crate::sim::SimData;

#[derive(Debug)]
pub enum GamePhase {
    // TODO: Use Blarser to ensure this is in sync with proper game phases
    NotStarted = 0,
    Starting,
    StartOfHalfInning,
}

#[derive(Debug)]
pub struct Game {
    game_id: Uuid,
    home_team: Uuid,
    away_team: Uuid,
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
            home_team: game_event.home_team,
            away_team: game_event.away_team,
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
            home_team: self.home_team,
            away_team: self.away_team,
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
        // self.phase = GamePhase::StartOfHalfInning;
        self.top_of_inning = !self.top_of_inning;
        self.inning += 1;
        Ok(FedEventData::HalfInningStart {
            game: self.game_event(),
            top_of_inning: self.top_of_inning,
            inning: self.inning + 1, // one-indexed
            batting_team_name: sim_data.teams.get(&if self.top_of_inning { self.away_team } else { self.home_team })
                .ok_or_else(|| anyhow!("Couldn't find batting team"))?
                .full_name.clone(),
            subseasonal_mod_effects: vec![], // TODO
        })
    }
}