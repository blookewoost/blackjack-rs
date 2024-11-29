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
    
    let result = game_loop(&mut dealer_cards, &mut player_cards, &mut deck);

    match result {
        EndGame::DealerWins => println!("You lose!"),
        EndGame::PlayerWins => println!("You win!"),
        EndGame::Tie => println!("You tied!"),
    }

}

fn dealer_hit(mut dealer_cards: Vec<Card>, mut deck: Deck) {
    dealer_cards.push(deck.cards.pop().unwrap());
}

fn game_loop(dealer_cards: &mut Vec<Card>, player_cards: &mut Vec<Card>, deck: &mut Deck) -> EndGame {
    
    loop {
        display_game_status(&dealer_cards[0], player_cards);
        let mut player_value = 0;
        for card in player_cards.iter() {
            player_value += card.value;
        }
        
        if player_value > BLACKJACK {
            return EndGame::DealerWins
        }

        let mut input = String::new();
        println!("You have {}, hit or stay? (Y/N)", player_value);
        std::io::stdin().read_line(&mut input).unwrap();

        match input.as_str() {
            "Y\n" => {
                player_cards.push(deck.cards.pop().unwrap());
            },
            "N\n" => {
                return dealer_plays_to_end(dealer_cards, player_cards, deck);
            },
            _ => {
                panic!("Invalid input!");
            }
        }
    }
    
    
}

fn display_game_status(dealer_card: &Card, player_cards: &Vec<Card>) {

    let mut player_value = 0;
    println!("{} {}", dealer_card.unicode, parse_unicode(HIDDEN).unwrap());
    println!("-------");

    for card in player_cards {
        print!("{} ", card.unicode);
        player_value += card.value;
    }

    print!(":{}", player_value);
}

fn dealer_plays_to_end(dealer_cards: &mut Vec<Card>, player_cards: &mut Vec<Card>, deck: &mut Deck) -> EndGame {
    
    loop {
        display_end_game_status(dealer_cards, player_cards);

        let mut dealer_value = 0; 
        for card in dealer_cards.iter() {
            dealer_value += card.value;
        }

        let mut player_value = 0;
        for card in player_cards.iter() {
            player_value += card.value;
        }

        if dealer_value > player_value && dealer_value < BLACKJACK {
            return EndGame::DealerWins
        } else if dealer_value > BLACKJACK {
            return EndGame::PlayerWins
        } else {
            dealer_cards.push(deck.cards.pop().unwrap());
        }
    }
}

fn display_end_game_status(dealer_cards: &Vec<Card>, player_cards: &Vec<Card>) {
    let mut player_value = 0;
    let mut dealer_value = 0;

    for card in dealer_cards{
        print!("{} ", card.unicode);
        dealer_value += card.value;
    }

    println!("    Dealer has {}", dealer_value);
    
    for card in player_cards {
        print!("{} ", card.unicode);
        player_value += card.value;
    }

    println!("    Player has {}\n", player_value);
}

enum EndGame {
    PlayerWins,
    DealerWins,
    Tie
}

