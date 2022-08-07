pub mod deck;
use crate::deck::cards::Card;

fn main() {
    let deck = Card::build_deck();

    for i in 0..deck.len() {
        deck[i].print_card();
    }
}