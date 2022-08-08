<div align="center">
<h1>Card Games</h1>
<h2>Playing Cards using Rust</h2>
<p>By Abe Choi</p>
</div>

<p align="center">
For this project I want to create a Deck mod with a Card struct, to import for different card games such as Black Jack.
</p>

1.  [Deck Mod](#Deck-Mod)
2.  [Using the Mod](#Using-the-Mod)


## Deck Mod

Lessons learned:

I tried to use an array of cards, because I knew I could use 52 as the fixed length, but I had a lot of issues assigning each element within a for loop.

example:
```
for i in 0..52 {
                
    if i < 13 {
        deck[i] = Card::new(ranks[i], "SPADES".to_string());
    }else if i < 26 {
        deck[i] = Card::new(ranks[i-13], "HEARTS".to_string());
    }else if i < 39 {
        deck[i] = Card::new(ranks[i-26], "CLUBS".to_string());
    }else {
        deck[i] = Card::new(ranks[i-39], "DIAMONDS".to_string());
    }
    
}
```

The array was mutable, and I tried to add `let` in front `deck[i]`, but I kept getting an unassigned error, so I used Vectors instead and had a much easier time.

## Using the Mod

Import Deck mod into main.rs
```
pub mod deck;
use crate::deck::cards::Card;
```

Use build_deck() to build the deck, and deck[i].print_card() to print a card.
```
fn main() {
    let deck = Card::build_deck();

    for i in 0..deck.len() {
        deck[i].print_card();
    }
}
```
