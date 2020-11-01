use crate::card::{Card, CardType, ManaType};
extern crate fast_map;
use rand::seq::SliceRandom;
use rayon::prelude::*;


#[derive(Default, fast_map::FastMap)]
#[fast_map(infallible = true, keys(
    CardType::Land,
    CardType::Creature,
    CardType::Artifact,
    CardType::Enchantment,
    CardType::Planeswalker,
    CardType::Instant,
    CardType::Sorcery
))]
struct TypeDistribution(fast_map::Map7<CardType, usize>);

#[derive(Default, fast_map::FastMap)]
#[fast_map(infallible = true, keys(
    ManaType::RED,
    ManaType::GREEN,
    ManaType::BLUE,
    ManaType::BLACK,
    ManaType::WHITE,
    ManaType::MULTICOLORED,
    ManaType::COLORLESS
))]
struct CostDistribution(fast_map::Map7<ManaType, usize>);


/*
think about what should be known about each turn, disregarding
another player, e.g:
DrawStats {
    _type
    colors
    does_flood?
    play_potential? -> (
        i.e. does this create a more well rounded hand. For example,
        lets say I have 3 lands, 3 creatures and 1 instant. creatures 1 & 2
        cost 2 & 3 mana respectively. On draw, do I draw a creature less than 2 
        that would allow a cast earlier. E.g. If I draw a creature/something other
        than land, and it costs 1 mana, that means I now have something to play for
        an additional turn. Previosuly, I would have to wait until turn two to play one,
        and turn three the next. Here, I now play something each turn.
    )

}
*/
struct TurnStats {
}

struct RunStats {
    turns: Vec<TurnStats>
}

struct DeckStats {
    type_distribution: TypeDistribution,
    cost_distribution: CostDistribution
}

impl DeckStats {
    pub fn update_type_distribution(&mut self, _type: CardType) {
        let current_count = self.type_distribution.get(&_type).cloned();

            match current_count {
                None => self.type_distribution.insert(&_type, 1),
                Some(n) => self.type_distribution.insert(&_type, n + 1)
            };
    }

    pub fn update_cost_distribution(&mut self, costs: Vec<(ManaType, usize)>) {
        for pair in costs {
            let current_count = self.cost_distribution.get(&pair.0).cloned();

            match current_count {
                None => self.cost_distribution.insert(pair.0, pair.1),
                Some(n) => self.cost_distribution.insert(pair.0, pair.1 + n)
            };
        }
    }

    fn fmt(&self) {

        println!("Red cost overall: {:?}", self.cost_distribution.get(ManaType::RED).unwrap());

    }
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

pub struct Deck {
    pub cards: Vec<Card>,
    stats: DeckStats,
    format: DeckFormat
}

impl Deck {
    /// Creates a new deck, assuming standard format
    pub fn new() -> Deck {
        Deck {
            cards: Vec::with_capacity(60),
            format: DeckFormat::STANDARD,
            stats: DeckStats {
                type_distribution: TypeDistribution::default(),
                cost_distribution: CostDistribution::default()
            }
        }
    }

    /// Creates a new deck, with soecified format
    pub fn with_format(format: &str) -> Deck {
        match format {
            "STANDARD" => Deck::new(),
            "COMMANDER" => {
                Deck {
                    cards: Vec::with_capacity(100),
                    format: DeckFormat::COMMANDER,
                    stats: DeckStats {
                        type_distribution: TypeDistribution::default(),
                        cost_distribution: CostDistribution::default()
                    }
                }
            },
            _ => Deck::new()
        }
    }

    pub fn insert_card(&mut self, card: Card) {
        let current_type = card._type.clone();
        let parsed_cost = card.mana_cost_parsed();

        self.cards.push(card);

        self.stats.update_type_distribution(current_type);
        self.stats.update_cost_distribution(parsed_cost);
    }

    // TODO: add size checks for deck based on format passed in
    pub fn from_cards(cards: Vec<Card>, format: &str) -> Deck {
        let mut deck = Deck::with_format(format);

        for card in cards {
            deck.insert_card(card);
        }
            
        deck
    }



    pub fn generate_stats(&mut self) {
        // generate type distribution
    }

    fn should_mulligan(draw_size: usize) -> bool {
        false
    }

    fn get_starting_hand(draw_size: usize, deck: &Vec<usize>) -> Vec<usize> {
        let mut hand = Vec::<usize>::new();
        for i in 0..draw_size {
            // hand.push(i);
            hand.push(deck[i]);
        }

        hand
    }

    fn run_simulation(cards: &Vec<Card>) -> RunStats {
        let mut turns: Vec<TurnStats> = Vec::new();
        let mut deck_indices: Vec<usize> = (0..cards.len()).collect();
        let mut draw_size: usize = 7;
        loop {
            let mut rng = rand::thread_rng();
            deck_indices.shuffle(&mut rng);

            if draw_size == 0 {
                // generate very negative stats lol
                break;
            }
            
            if Deck::should_mulligan(draw_size) {
                draw_size -= 1;
            }

            else {
                break;
            }
        }

        // starting hand would be cards[0 -> draw_size - 1]
        // storing indices
        let mut hand: Vec<usize> = Deck::get_starting_hand(draw_size, &deck_indices);
        
        // println!("Hand: {:?}", hand);

        for turn in 0..2 {
            if turn != 0 {
                hand.push(deck_indices[draw_size + turn]);
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

        RunStats {
            turns: turns
        }
        
    }

    // FIXME: I do not like cloning the vector every iteration
    // but otherwise there would be race conditions


    // TODO: look into these:
    // https://github.com/Amanieu/parking_lot
    // Arc<Mutex<Vec<RunStats>>>
    pub fn run_n_simulations(&mut self, n: u32) {
        let mut runs: Vec<RunStats> = Vec::new();
        (0..n).into_par_iter().map(|i| {
            Deck::run_simulation(&self.cards)
        }).collect_into_vec(&mut runs);

        // update deck stats

    }

    pub fn run_n_simulations_nonpar(&mut self, n: u32) {
        let mut runs: Vec<RunStats> = Vec::new();
        (0..n).for_each(|_| {
            runs.push(Deck::run_simulation(&mut self.cards));
        });
    }

    // pub fn size(&self) -> usize {
    //     self.cards.len()
    // }
}