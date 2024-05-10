use cardio::{Card, Deck};

fn _scenario(condition: fn(Card) -> bool, deck: Deck) -> bool {
    for card in deck.cards() {
        return condition(card);
    }
    return false;
}

fn is_king(card: Card) -> bool {
    card.card_type() == cardio::CardType::King
}

fn main() {
    let jokerless_deck = Deck::full_no_jokers();

    let runs: Vec<bool> = (1u8..100)
        .map(|_| is_king(jokerless_deck.pick_random()))
        .collect();

    let _trues = runs
        .clone()
        .into_iter()
        .filter(|x| *x)
        .collect::<Vec<bool>>()
        .len() as u16;
    let falses = runs
        .clone()
        .into_iter()
        .filter(|x| !*x)
        .collect::<Vec<bool>>()
        .len() as u16;

    println!("Chances of getting a king: {:?}", ((100-falses)/100) as f32);
}
