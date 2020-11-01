mod deck;
mod card;
use std::time::Instant;

fn main() {
    let card = card::Card::new(
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
    parallel_benchmark(10, &mut deck);
    parallel_benchmark(100, &mut deck);
    parallel_benchmark(1000, &mut deck);
    parallel_benchmark(10000, &mut deck);
    parallel_benchmark(30000, &mut deck);
    parallel_benchmark(60000, &mut deck);
    parallel_benchmark(60000, &mut deck);
    parallel_benchmark(100000, &mut deck);
    parallel_benchmark(1000000, &mut deck);
    parallel_benchmark(10000000, &mut deck);
    
    single_benchmark(10, &mut deck);
    single_benchmark(100, &mut deck);
    single_benchmark(1000, &mut deck);
    single_benchmark(10000, &mut deck);
    single_benchmark(30000, &mut deck);
    single_benchmark(60000, &mut deck);
    single_benchmark(60000, &mut deck);
    single_benchmark(60000, &mut deck);
    single_benchmark(60000, &mut deck);
    single_benchmark(100000, &mut deck);
    single_benchmark(1000000, &mut deck);
    single_benchmark(10000000, &mut deck);

    // println!("Deck length: {:?}", deck.size());
    // println!("Deck cards: \n{:?}\n", &deck.cards);

}

fn parallel_benchmark(n: u32, deck: &mut deck::Deck) {
    let start_par = Instant::now();
    deck.run_n_simulations(n);
    let duration_par = start_par.elapsed();
    println!("multi-threaded time for {} simulations:  {:?}", n, duration_par);

}

fn single_benchmark(n: u32, deck: &mut deck::Deck) {
    let start_sgl = Instant::now();
    deck.run_n_simulations_nonpar(n);
    let duration_sgl = start_sgl.elapsed();
    println!("single-threaded time for {} simulations {:?}:", n, duration_sgl);
}