mod deck;
mod card;
use std::time::Instant;
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

    let mut deck = deck::Deck::from_cards(vec![card; 60], "STANDARD");

    // println!("Deck length: {:?}", deck.size());
    // println!("Deck cards: \n{:?}\n", &deck.cards);

    let start_par = Instant::now();
    deck.run_n_simulations(100000);
    let duration_par = start_par.elapsed();

    let start_sgl = Instant::now();
    deck.run_n_simulations_nonpar(100000);
    let duration_sgl = start_sgl.elapsed();

    println!("parallel time: {:?}", duration_par);
    println!("single time: {:?}", duration_sgl);
}