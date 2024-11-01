mod games;
fn main() {
    let mut deck = games::Deck::new();
    let mut _shuffled_deck = deck.shuffle();
    let hand = deck.deal(3);
    println!("Here is your deck {:#?}", deck);
    println!("Heres is your hand {:#?}", hand);
}
