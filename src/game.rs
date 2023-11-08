use fed::{FedEventData, GameEvent, Weather};
use uuid::Uuid;
use crate::rng::Rng;

#[derive(Debug)]
pub enum GamePhase {
    // TODO: Use Blarser to ensure this is in sync with proper game phases
    NotStarted = 0,

}

#[derive(Debug)]
pub struct Game {
    game_id: Uuid,
    home_team: Uuid,
    away_team: Uuid,
    weather: Weather,
    stadium_id: Option<Uuid>,

    phase: GamePhase,
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
        }
    }

    pub fn tick(&mut self, rng: &mut Rng) -> FedEventData {
        match self.phase {
            GamePhase::NotStarted => {
                self.start_game()
            }
        }
    }

    pub fn start_game(&mut self) -> FedEventData {
        FedEventData::LetsGo {
            game: GameEvent {
                game_id: self.game_id,
                home_team: self.home_team,
                away_team: self.away_team,
                play: 0,
                unscatter: None,
                attractor_secret_base: None,
            },
            weather: self.weather,
            stadium_id: self.stadium_id,
        }
    }
}