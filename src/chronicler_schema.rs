use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct TeamState {
    pub redacted: Option<bool>,
    pub nullified: Option<bool>,
    pub scattered: Option<TeamScatteredInfo>,
    #[serde(rename = "imp_motion")] // override the rename_all = "camelCase"
    pub imp_motion: Option<Vec<ImpMotionEntry>>,
    pub perm_mod_sources: Option<HashMap<String, Vec<Uuid>>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct ImpMotionEntry {
    day: i32,
    season: i32,
    // I would like this to be a tuple but I don't want to figure out the macro magic to make that happen
    im_position: Vec<f32>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct TeamScatteredInfo {
    full_name: String,
    location: String,
    nickname: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Team {
    pub id: Uuid,
    pub card: Option<i32>,
    pub emoji: String,
    pub level: Option<i32>,
    pub state: Option<TeamState>,
    pub lineup: Vec<Uuid>,
    pub slogan: String,
    pub shadows: Option<Vec<Uuid>>,
    pub bench: Option<Vec<Uuid>>,
    pub bullpen: Option<Vec<Uuid>>,
    pub stadium: Option<Uuid>,
    pub deceased: Option<bool>,
    pub full_name: String,
    pub game_attr: Vec<String>,
    pub league_id: Option<Uuid>,
    pub location: String,
    pub nickname: String,
    pub perm_attr: Vec<String>,
    pub rotation: Vec<Uuid>,
    pub seas_attr: Vec<String>,
    pub week_attr: Vec<String>,
    pub evolution: Option<i32>,
    pub main_color: String,
    pub shame_runs: f32,
    pub shorthand: String,
    pub win_streak: Option<i32>,
    pub division_id: Option<Uuid>,
    pub team_spirit: i32,
    pub subleague_id: Option<Uuid>,
    pub total_shames: i32,
    pub rotation_slot: i32,
    pub season_shames: i32,
    pub championships: i32,
    pub total_shamings: i32,
    pub season_shamings: i32,
    pub secondary_color: String,
    pub tournament_wins: Option<i32>,
    pub underchampionships: Option<i32>,

    #[serde(rename = "eDensity")] pub edensity: Option<f32>,
    #[serde(rename = "eVelocity")] pub evelocity: Option<f32>,
    #[serde(rename = "imPosition")] pub imposition: Option<f32>,
}

impl Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.state.as_ref().and_then(|state| state.scattered.as_ref()).map(|info| &info.full_name) {
            Some(name) => write!(f, "Team: {}", name),
            None => write!(f, "Team: {}", self.full_name),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Item {
    // TODO Implement Item, reinstate deny_unknown_fields
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct PlayerState {
    pub cut_this_election: Option<bool>,
    pub necromancied_this_election: Option<bool>,
    pub redacted: Option<bool>,
    pub elsewhere: Option<PlayerElsewhereInfo>,
    // Detective hunches
    pub hunches: Option<Vec<i32>>,
    pub investigations: Option<i32>,
    // Original player for this Replica
    pub original: Option<Uuid>,
    pub perm_mod_sources: Option<HashMap<String, Vec<String>>>,
    pub seas_mod_sources: Option<HashMap<String, Vec<String>>>,
    pub game_mod_sources: Option<HashMap<String, Vec<String>>>,
    pub item_mod_sources: Option<HashMap<String, Vec<Uuid>>>,
    pub unscattered_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct PlayerElsewhereInfo {
    pub day: i32,
    pub season: i32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub ritual: Option<String>,
    pub fate: Option<i32>,
    pub soul: i32,
    pub blood: Option<i32>,
    pub coffee: Option<i32>,
    pub peanut_allergy: Option<bool>,

    pub bat: Option<String>,
    pub armor: Option<String>,

    pub league_team_id: Option<Uuid>,
    pub tournament_team_id: Option<Uuid>,
    pub deceased: Option<bool>,
    pub evolution: Option<i32>,
    pub items: Option<Vec<Item>>,
    pub state: Option<PlayerState>,
    pub hit_streak: Option<i32>,
    pub consecutive_hits: Option<i32>,

    pub game_attr: Option<Vec<String>>,
    pub week_attr: Option<Vec<String>>,
    pub seas_attr: Option<Vec<String>>,
    pub item_attr: Option<Vec<String>>,
    pub perm_attr: Option<Vec<String>>,

    pub buoyancy: f64,
    pub cinnamon: Option<f64>,
    pub coldness: f64,
    pub chasiness: f64,
    pub divinity: f64,
    pub martyrdom: f64,
    pub base_thirst: f64,
    pub indulgence: f64,
    pub musclitude: f64,
    pub tragicness: f64,
    pub omniscience: f64,
    pub patheticism: f64,
    pub suppression: f64,
    pub continuation: f64,
    pub ruthlessness: f64,
    pub watchfulness: f64,
    pub laserlikeness: f64,
    pub overpowerment: f64,
    pub tenaciousness: f64,
    pub thwackability: f64,
    pub anticapitalism: f64,
    pub ground_friction: f64,
    pub pressurization: f64,
    pub unthwackability: f64,
    pub shakespearianism: f64,
    pub moxie: f64,
    pub total_fingers: i32,

    pub defense_rating: Option<f32>,
    pub hitting_rating: Option<f32>,
    pub pitching_rating: Option<f32>,
    pub baserunning_rating: Option<f32>,

    #[serde(rename = "eDensity")] pub edensity: Option<f32>,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player: {}",
               self.state.as_ref()
                   .and_then(|state| state.unscattered_name.as_ref())
                   .unwrap_or(&self.name))
    }
}

fn list_has_mod(list: &Option<Vec<String>>, mod_name: &str) -> bool {
    list.as_ref().is_some_and(|attrs| attrs.iter().any(|attr| attr == mod_name))
}

impl Player {
    pub fn has_mod(&self, mod_name: &str) -> bool {
        false || // for alignment
            list_has_mod(&self.perm_attr, mod_name) ||
            list_has_mod(&self.seas_attr, mod_name) ||
            list_has_mod(&self.week_attr, mod_name) ||
            list_has_mod(&self.game_attr, mod_name) ||
            list_has_mod(&self.item_attr, mod_name)
    }
}