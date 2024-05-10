use cardio::{Card, Deck};

fn run_scenario(condition: fn(Card) -> bool, deck: Deck, iterations: u16) -> f32 {
    (0u16..iterations).into_iter()
        .map(|_| deck.pick_random())
        .map(|x| condition(x))
        .filter(|x| *x)
        .collect::<Vec<bool>>()
        .len() as f32 / iterations as f32
}
fn is_face(card: Card) -> bool {
    match card.card_type() {
        cardio::CardType::Number(_) | cardio::CardType::Ace => false,
        _ => true
    }
}

fn main() {
    let jokerless_deck = Deck::full_no_jokers();

    let runs = run_scenario(is_face, jokerless_deck, 1000);
    
    println!("Chances of getting a face card: {:#?} ({}%)", runs, runs*100.0);
}
