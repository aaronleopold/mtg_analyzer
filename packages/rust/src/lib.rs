pub mod deck;
pub mod card;

use deck::{RunStats, Deck};
use card::{Card, CardType};
use rayon::prelude::*;
use std::time::Instant;


// TODO: return stats
pub fn run_n_simulations(n: u32, deck: &mut Deck) {
  let mut runs: Vec<RunStats> = Vec::new();

  (0..n).into_par_iter().map(|_| {
      deck.run_simulation()
  }).collect_into_vec(&mut runs);

}

// TODO: return stats
pub fn run_n_simulations_nonpar(n: u32, deck: &mut Deck) {
  let mut runs: Vec<RunStats> = Vec::new();

  (0..n).for_each(|_| {
      runs.push(deck.run_simulation());
  });
}

fn parallel_benchmark(n: u32, deck: &mut Deck) {
  let start_par = Instant::now();
  run_n_simulations(n, deck);
  let duration_par = start_par.elapsed();
  println!("multi-threaded time for {} simulations:  {:?}", n, duration_par);

}

fn single_benchmark(n: u32, deck: &mut Deck) {
  let start_sgl = Instant::now();
  run_n_simulations_nonpar(n, deck);
  let duration_sgl = start_sgl.elapsed();
  println!("single-threaded time for {} simulations {:?}:", n, duration_sgl);
}



fn compare_par_v_nonpar() {
    let card = Card::new(
        String::from("Test"),
        String::from("{2}{R}"),
        vec![String::from("R")],
        vec![String::from("R")],
        CardType::Creature,
        2,
        1,
        12345
    );

    let mut deck = Deck::from_cards(vec![card; 60], "STANDARD");

    println!("Deck length: {:?}", deck.len());
    println!("Deck cards: \n{:?}\n", &deck.cards);

    println!("Starting tests...");

    parallel_benchmark(10, &mut deck);
    parallel_benchmark(50, &mut deck);
    parallel_benchmark(100, &mut deck);
    parallel_benchmark(1000, &mut deck);
    parallel_benchmark(10000, &mut deck);
    parallel_benchmark(30000, &mut deck);
    parallel_benchmark(60000, &mut deck);
    parallel_benchmark(100000, &mut deck);
    parallel_benchmark(1000000, &mut deck);
    // parallel_benchmark(10000000, &mut deck);
    
    single_benchmark(10, &mut deck);
    single_benchmark(50, &mut deck);
    single_benchmark(100, &mut deck);
    single_benchmark(1000, &mut deck);
    single_benchmark(10000, &mut deck);
    single_benchmark(30000, &mut deck);
    single_benchmark(60000, &mut deck);
    single_benchmark(100000, &mut deck);
    single_benchmark(1000000, &mut deck);
    // single_benchmark(10000000, &mut deck);
}

#[cfg(test)]
mod tests {

  use super::*;

    #[test]
    fn suite() {
      compare_par_v_nonpar();
    }
}