#[derive(Debug)]
pub struct Deck {
    cards: Vec<String>,
}
impl Deck {
    pub fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&self) {}
}
