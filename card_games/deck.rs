pub mod cards {

    #[derive(Debug)]
    pub struct Card {
        pub rank: char,
        pub suit: String,
    }
 
    impl Card {

        pub fn new(rank: char, suit: String) -> Card {
            Card {rank, suit}
        }

        pub fn build_deck() -> Vec<Card> {

            let mut deck: Vec<Card> = vec![];
            let ranks: [char; 13] = ['2','3','4','5','6','7','8','9','T','J','Q','K','A'];
    
            for i in 0..52 {
                
                if i < 13 {
                    deck.push(Card::new(ranks[i], "SPADES".to_string()));
                }else if i < 26 {
                    deck.push(Card::new(ranks[i-13], "HEARTS".to_string()));
                }else if i < 39 {
                    deck.push(Card::new(ranks[i-26], "CLUBS".to_string()));
                }else {
                    deck.push(Card::new(ranks[i-39], "DIAMONDS".to_string()));
                }
                
            }
            deck
        }

        pub fn print_card(&self) {
            println!("{} {}", self.rank, self.suit);
        }
    }
}