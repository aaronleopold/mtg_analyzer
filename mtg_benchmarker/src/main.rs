mod deck;
mod card;
fn main() {
    let mut card = card::Card::new(
        String::from("Test"),
        String::from("{2}{R}"),
        vec![String::from("R")],
        vec![String::from("R")],
        card::CardType::Creature,
        2,
        1,
        12345
    );

    let mut deck = deck::Deck::from_cards(vec![card], "STANDARD");

}