use crate::card::{Card, CardType};
extern crate fast_map;
use rand::seq::SliceRandom;

struct DeckStats {
    type_distribution: fast_map::Map7<CardType, usize>
}

enum DeckFormat {
    STANDARD,
    MODERN,
    COMMANDER,
    LEGACY,
    VINTAGE,
    BRAWL,
    THGIANT,
    PAUPER
}

struct Deck {
    cards: Vec<Card>,
    // stats: DeckStats,
    format: DeckFormat
}

impl Deck {
    /// Creates a new deck, assuming standard format
    pub fn new() -> Deck {
        Deck {
            cards: Vec::with_capacity(60),
            format: DeckFormat::STANDARD
        }
    }

    /// Creates a new deck, with soecified format
    pub fn with_format(format: &str) -> Deck {
        match format {
            "STANDARD" => Deck::new(),
            "COMMANDER" => {
                Deck {
                    cards: Vec::with_capacity(100),
                    format: DeckFormat::COMMANDER
                }
            },
            _ => Deck::new()
        }
    }

    // TODO: add size checks for deck based on format passed in
    pub fn from_cards(cards: Vec<Card>, format: &str) -> Deck {
        let mut deck = Deck::with_format(format);

        for card in cards {
            deck.cards.push(card);
        }

        deck
    }

    pub fn generate_stats() {}

    fn should_mulligan(&self, draw_size: usize) -> bool {
        false
    }

    fn get_starting_hand(&self, draw_size: usize) -> Vec<&Card> {
        let mut hand = Vec::<&Card>::new();
        for i in 0..draw_size {
            hand.push(&self.cards[i]);
        }

        hand
    }

    fn run_simulation(&mut self) {
        let mut draw_size: usize = 7;
        loop {
            let mut rng = rand::thread_rng();
            self.cards.shuffle(&mut rng);

            if draw_size == 0 {
                // generate very negative stats lol
                break;
            }
            
            if self.should_mulligan(draw_size) {
                draw_size -= 1;
            }

            else {
                break;
            }
        }

        // starting hand would be cards[0 -> draw_size - 1]
        let mut hand: Vec<&Card> = self.get_starting_hand(draw_size);
        for turn in 0..2 {
            if turn != 0 {
                hand.push(&self.cards[draw_size + turn]);
            }

            // TODO: create function to 'rate' hand and determine moves, then make moves
        }

        /*
            while bad_hand
                shuffle deck
                draw 7-iteration_count
                rate starting hand
            
            for 0..2 turns
                if not 0 draw
                rate hand state
                    has moves
                    has move next turn?
                    hand makeup
                    etc
                make moves
        */
        
    }

    pub fn run_n_simulations(n: u16) {}
}