use serde::{Deserialize, Serialize};
use crate::card::Card;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub hand: Vec<Card>,
    pub tricks_won: u8,
    pub bid: Option<u8>,
}

impl Player {
    pub fn new(id: String, name: String) -> Self {
        Player {
            id,
            name,
            hand: Vec::new(),
            tricks_won: 0,
            bid: None,
        }
    }
}
