mod games;
fn main() {
    let deck = games::Deck::new();
    println!("Heres is your deck {:#?}", deck);
}
