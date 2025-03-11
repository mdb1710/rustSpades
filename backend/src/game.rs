use crate::card::{Card, Rank, Suit};
use crate::player::Player;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct Game {
    pub id: String,
    pub players: Vec<Player>,
    pub deck: Vec<Card>,
    pub current_trick: Vec<Card>,
    pub current_player: usize,
    pub scores: Vec<i32>,
}

impl Game {
    pub fn new(id: String) -> Self {
        let mut deck = Vec::new();
        for suit in [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs].iter() {
            for rank in [Rank::Two, /* ... */, Rank::Ace].iter() {
                deck.push(Card::new(*suit, *rank));
            }
        }

        Game {
            id,
            players: Vec::new(),
            deck,
            current_trick: Vec::new(),
            current_player: 0,
            scores: vec![0; 4],
        }
    }

    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle(&mut thread_rng());
    }

    pub fn deal_cards(&mut self) {
        self.shuffle_deck();
        for i in 0..52 {
            let player_index = i % 4;
            if let Some(player) = self.players.get_mut(player_index) {
                player.hand.push(self.deck[i]);
            }
        }
    }
}
