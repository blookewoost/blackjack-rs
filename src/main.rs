use bj::{Deck, Card, parse_unicode};
const HIDDEN: &str = "U+1F0A0";
const BLACKJACK: i32 = 21;

fn main() {

    let mut deck = Deck::new();
    deck.shuffle();

    start_hand(deck);
}

fn start_hand(mut deck: Deck) {
    let mut player_cards: Vec<Card> = vec![];
    let mut dealer_cards: Vec<Card> = vec![];

    player_cards.push(deck.cards.pop().unwrap());
    dealer_cards.push(deck.cards.pop().unwrap());
    player_cards.push(deck.cards.pop().unwrap());
    dealer_cards.push(deck.cards.pop().unwrap());

    println!("{} {}", dealer_cards[0].unicode, parse_unicode(HIDDEN).unwrap());
    println!("-------");
    println!("{} {}", player_cards[0].unicode, player_cards[1].unicode);
    
    game_loop(&mut dealer_cards, &mut player_cards, &mut deck);
}

fn dealer_hit(mut dealer_cards: Vec<Card>, mut deck: Deck) {
    dealer_cards.push(deck.cards.pop().unwrap());
}

fn game_loop(dealer_cards: &mut Vec<Card>, player_cards: &mut Vec<Card>, deck: &mut Deck) {
    
    loop {
        let mut player_value = 0;
        for card in player_cards.iter() {
            player_value += card.value;
        }

        let mut input = String::new();
        println!("You have {}, hit or stay? (Y/N)", player_value);
        std::io::stdin().read_line(&mut input).unwrap();

        match input.as_str() {
            "Y\n" => {
                player_cards.push(deck.cards.pop().unwrap());
            },
            "N\n" => {
                dealer_plays_to_end(dealer_cards, player_cards, deck);
            },
            _ => {
                panic!("Invalid input!");
            }
        }
        display_game_status(dealer_cards, player_cards);
    }
    
    
}

fn display_game_status(dealer_cards: &Vec<Card>, player_cards: &Vec<Card>) {
    let mut dealer_value = 0;
    let mut player_value = 0;

    for card in dealer_cards {
        print!("{} ", card.unicode);
        dealer_value += card.value;
    }

    print!(":{}", dealer_value);

    println!("");
    println!("-------");

    for card in player_cards {
        print!("{} ", card.unicode);
        player_value += card.value;
    }

    if player_value > BLACKJACK {
        panic!("Dealer wins!");
    }
    print!(":{}", player_value);
    
}

fn dealer_plays_to_end(dealer_cards: &mut Vec<Card>, player_cards: &mut Vec<Card>, deck: &mut Deck) {
    
    loop {
        let mut dealer_value = 0; 
        for card in dealer_cards.iter() {
            dealer_value += card.value;
        }

        let mut player_value = 0;
        for card in player_cards.iter() {
            player_value += card.value;
        }

        if dealer_value > player_value && dealer_value < BLACKJACK {
            println!("Dealer wins!");
            break;
        } else if dealer_value > BLACKJACK {
            println!("Player wins!");
            break;
        } else {
            dealer_cards.push(deck.cards.pop().unwrap());
        }
    }
    
}

fn dealer_wins() {
    todo!();
}

fn player_wins() {
    todo!();
}
