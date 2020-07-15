use crate::card::Card;

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Creates a new deck, assuming standard format
    fn new() -> Deck {
        Deck {
            cards: Vec::with_capacity(60)
        }
    }

    /// Creates a new deck, with soecified format
    fn with_format(format: &str) -> Deck {
        match format {
            "STANDARD" => Deck::new(),
            "COMMANDER" => Deck::new(),
            _ => Deck::new()
        }
    }

    fn from_cards(cards: Vec<Card>, format: &str) -> Deck {
        let mut deck = Deck::with_format(format);

        for card in cards {
            deck.cards.push(card);
        }

        deck
    }
}