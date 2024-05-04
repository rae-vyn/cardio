use clap::Parser;
use std::fmt::{self, Formatter};

#[derive(Debug, Parser, Copy, Clone)]
pub enum Suit {
    Hearts,
    Clubs,
    Spades,
    Diamonds
}

#[derive(Parser, Debug)]
pub struct CardBuilder {
    pub suit: String,
    pub value: u8
}

impl CardBuilder {
    pub fn verify(&self) -> Option<Card> {
        let suit = match self.suit.to_lowercase().as_str() {
            "hearts"   | "heart"   | "h" => Suit::Hearts,
            "clubs"    | "club"    | "c" => Suit::Clubs,
            "spades"   | "spade"   | "s" => Suit::Spades,
            "diamonds" | "diamond" | "d" => Suit::Diamonds,
            _ => panic!("Wrong suit: {}", self.suit)
        };

        if self.value <= 15 {
            Some(Card {
                suit,
                value: self.value
            })
        } else {
            None
        }

    }
}

pub struct Card {
    suit: Suit,
    value: u8
}

impl fmt::Display for Card {
    fn fmt(&self, buf: &mut Formatter<'_>) -> Result<(), std::fmt::Error>{
        let plainstr = format!("{}", self.value);
        let numstr = match self.value {
            0 => "Ace",
            12 => "Jack",
            13 => "Queen",
            14 => "King",
            15 => "Joker",
            _ => plainstr.as_str()
        };

        write!(buf, "{} of {:?}", numstr, self.suit)
    }
}

impl Card {
    pub fn suit(&self) -> Suit {
        return self.suit;
    }
    pub fn value(&self) -> u8 {
        return self.value;
    }
}
