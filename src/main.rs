use cardio::Deck;

fn main() {
    let mut jokerless_deck = Deck::full_no_jokers();

    for _ in 1u8..=10 {
        println!("{:?}", jokerless_deck.pick_random());
    }
    println!("{}", jokerless_deck.card_count());
    for _ in 1u8..=10 {
        println!("{:?}", jokerless_deck.take_random());
    }
    println!("{}", jokerless_deck.card_count());
}
