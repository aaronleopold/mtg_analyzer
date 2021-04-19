use crate::card::{Card, CardType, ManaType};
extern crate fast_map;
use rand::seq::SliceRandom;


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
pub struct TurnStats {
}

pub struct RunStats {
    turns: Vec<TurnStats>
}

struct DeckStats {
    type_distribution: TypeDistribution,
    cost_distribution: CostDistribution
}

impl DeckStats {
    pub fn update_type_distribution(&mut self, _type: CardType) {
            let current_count: usize = 
                self.type_distribution
                    .get(&_type)
                    .cloned()
                    .unwrap_or_else(|| 0);

            self.type_distribution.insert(&_type, current_count + 1);
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
    // MODERN,
    COMMANDER,
    // LEGACY,
    // VINTAGE,
    // BRAWL,
    // THGIANT,
    // PAUPER
}

impl From<&str> for DeckFormat {
    fn from(format: &str) -> DeckFormat {
        match format {
            "STANDARD" => DeckFormat::STANDARD,
            "COMMANDER" => DeckFormat::COMMANDER,
            _ => todo!("handle me")
        }
    }
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
        match DeckFormat::from(format) {
            DeckFormat::STANDARD => Deck::new(),
            DeckFormat::COMMANDER => {
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
        let current_type = card.get_type();
        let parsed_cost = card.get_mana();

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

    /// Given the type of card, will return probability that the next card draw
    /// is of that type
    pub fn probability_type(target_type: CardType) {

    }



    pub fn generate_stats(&mut self) {
        // generate type distribution
    }

    fn hand_lands(&self, hand: &Vec<usize>){

    } 

    fn hand_type_count(&self, hand: &Vec<usize>, target_type: CardType) -> usize {
        0    
    }

    // 1). priority_type -> user declared priority (i.e. creatures or instants)
    //     these will determine how many of type is good to have at once
    // 2). 
    fn should_mulligan(&self, hand: &Vec<usize>) -> bool {
        // how many priority types are there in my hand? enough for 2-3 turns?
        // are there enough lands given the hand and probility for next card land?
        // 
        true
    }

    pub fn run_simulation(&self) -> RunStats {
        let turns: Vec<TurnStats> = Vec::new();
        let mut deck_indices: Vec<usize> = (0..self.cards.len()).collect();
        let mut draw_size: usize = 7;

        let mut hand: Vec::<usize> = Vec::new();

        // this is the first turn loop, checking if the hand should be 
        // mulliganed. Realistically this should have a floor of 3-4, but I 
        // let it go to 0 just because I am curious what would happen.
        loop {
            if draw_size == 0 {
                // generate very negative stats lol
                break;
            }

            let mut rng = rand::thread_rng();

            deck_indices.shuffle(&mut rng);

            // starting hand would be cards[0 -> draw_size - 1]
            // storing indices
            hand = 
                (0..draw_size)
                .into_iter()
                .map(|i| deck_indices[i])
                .collect();
            
            if self.should_mulligan(&hand) {
                draw_size -= 1;
            } else {
                break;
            }
        }

        // // starting hand would be cards[0 -> draw_size - 1]
        // // storing indices
        // let mut hand: Vec<usize> = 
        //     (0..draw_size)
        //     .into_iter()
        //     .map(|i| deck_indices[i])
        //     .collect();
        
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

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}