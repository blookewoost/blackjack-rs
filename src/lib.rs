use std::ops::Deref;
use rand::thread_rng;
use rand::seq::SliceRandom;

use csv::{Reader, StringRecord};

pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        let mut reader = Reader::from_path("src/deck.csv").unwrap();
        let mut cards: Vec<Card> = Vec::new();

        for record in reader.records() {
            match record {
                Ok(s) => {
                    cards.push(Card::new(s));
                },
                Err(_) => panic!("Error parsing deck!"),
            }
        }

        Self { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}
pub struct Card {
    name: String,
    pub value: i32, 
    suit: Suit,
    pub unicode: char,
}

impl Card {
    pub fn new(s: StringRecord) -> Self {
        let name = s.get(0).unwrap();
        let suit = match s.get(1) {
            None => panic!("ooo"),
            Some(suit) => match suit {
                "Spades" => Suit::Spades,
                "Clubs" => Suit::Clubs,
                "Diamonds" => Suit::Diamonds,
                "Hearts" => Suit::Hearts,
                _ => panic!("p"),
            }
        };

        let value = match s.get(2) {
            None => panic!("00"),
            Some(s) => match s.parse::<i32>() {
                Ok(i) => i,
                Err(_) => panic!("ppp"),
            }
        };


        let uni = match s.get(3) {
            None => panic!("00"),
            Some(s) => parse_unicode(s),
        };

        Self {name: name.to_string(), value, suit, unicode: uni.unwrap()} 

    }
}

pub fn parse_unicode(unicode_str: &str) -> Option<char> {
    if let Some(hex_code) = unicode_str.strip_prefix("U+") {
        // Convert the hexadecimal string to a Unicode scalar value
        u32::from_str_radix(hex_code, 16).ok().and_then(std::char::from_u32)
    } else {
        None
    }
}

enum Suit {
    Hearts, 
    Diamonds,
    Spades,
    Clubs
}