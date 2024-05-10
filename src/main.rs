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

    let runs: Vec<bool> = (1u8..=100)
        .map(|_| is_king(jokerless_deck.pick_random()))
        .collect();

    let trues = runs
        .clone()
        .into_iter()
        .filter(|x| *x)
        .collect::<Vec<bool>>()
        .len() as f32;
    let falses = runs
        .clone()
        .into_iter()
        .filter(|x| !*x)
        .collect::<Vec<bool>>()
        .len() as f32;
    println!("{} {}", trues, falses);

    println!("Chances of getting a king: {:#?}", (trues/100f32) as f32);
}
