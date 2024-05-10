use rand::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CardType {
    Number(u8),
    King,
    Queen,
    Joker,
    Jack,
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    suit: Suit,
    card_type: CardType,
}

#[derive(Clone, Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Card {
    pub fn new(suit: Suit, card_type: CardType) -> Self {
        if let CardType::Number(x) = card_type {
            if (x > 10) || (x < 2) {
                panic!("wrong card size for {:?} of {:?}!", x, suit);
            }
        }
        Card { suit, card_type }
    }
    pub fn suit(&self) -> Suit {
        self.suit
    }
    pub fn card_type(&self) -> CardType {
        self.card_type
    }
    pub fn get_value(&self) -> Option<u8> {
        match self.card_type {
            CardType::Number(x) => Some(x),
            _ => None,
        }
    }
}

impl Deck {
    pub fn empty() -> Self {
        Self { cards: vec![] }
    }
    pub fn add(&mut self, suit: Suit, card_type: CardType) {
        self.cards.push(Card::new(suit, card_type));
    }
    pub fn full() -> Self {
        let mut temp = Deck::empty();
        let faces = vec![
            CardType::King,
            CardType::Queen,
            CardType::Ace,
            CardType::Joker,
            CardType::Jack,
        ];
        let suits = vec![Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        for suit in suits {
            // Add all the numbers.
            for i in 2u8..=10 {
                temp.add(suit, CardType::Number(i))
            }
            // Add all the face cards.
            for face in faces.iter() {
                temp.add(suit, *face)
            }
        }
        temp
    }
    pub fn full_no_jokers() -> Self {
        let mut temp = Deck::empty();
        let faces = vec![
            CardType::King,
            CardType::Queen,
            CardType::Ace,
            CardType::Jack,
        ];
        let suits = vec![Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        for suit in suits {
            // Add all the numbers.
            for i in 2u8..=10 {
                temp.add(suit, CardType::Number(i))
            }
            // Add all the face cards.
            for face in faces.iter() {
                temp.add(suit, *face)
            }
        }
        temp
    }
    pub fn card_count(&self) -> u8 {
        self.cards.len() as u8
    }
    pub fn filter(&self, check: fn(&Card) -> Option<Card>) -> Vec<Card> {
        self.cards.iter().filter_map(check).collect()
    }
    pub fn pick_random(&self) -> Card {
        let mut rng = thread_rng();
        *self.cards.choose(&mut rng).unwrap()
    }
    pub fn take_random(&mut self) -> Card {
        let (i, &card) = self.cards.iter()
                       .enumerate()
                       .choose(&mut thread_rng())
                       .unwrap();
        self.cards.remove(i);
        card
    }
    pub fn cards(&self) -> Vec<Card> {
        self.cards.clone()
    }
}
