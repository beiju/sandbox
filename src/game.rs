use fed::FedEventData;
use crate::rng::Rng;

#[derive(Debug)]
pub struct Game {

}

impl Game {
    pub fn new() -> Self {
        Game {}
    }

    pub fn tick(&mut self, rng: &mut Rng) -> FedEventData {
        todo!()
    }
}