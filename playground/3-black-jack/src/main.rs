#[allow(unused_assignments)]

use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::io;
use std::io::*;

#[derive(Copy, Clone)]
enum CardType {
    Spades,
    Hearts,
    Diamonds, 
    Ace
}

// #[derive(Display)]

struct Card {
    cardType: CardType,
    cardNumber: u32
}


struct Deck {
    cards: Vec<Card>
}

fn main() {
    let number_of_players = 2;
    let mut deck = instatiate_deck();
    assert!(deck.cards.len() == 52);
    let mut sum = 0;

    loop {
        let card = deal_card(&mut deck);
        sum += card.cardNumber;
        println!("You were dealt {} and your sum is {}", card.cardNumber, sum);

        if (sum == 21) {
            println!("You won!");
            break;
        } else if (sum > 21) {
            println!("You lost");
            break;
        } else {
        }

        println!("Do you want another card? Press 0 to say no");
        let mut yes = String::new();
        io::stdin()
            .read_line(&mut yes)
            .expect("Failed to read line");
        if (yes.trim().parse() == Ok(0)) {
            break;
        }
    }

}


fn deal_card(deck: &mut Deck) -> Card {
   deck.cards.pop().unwrap()
}

fn instatiate_deck() -> Deck {
    let mut all_cards = instantiate_cards_of_card_type(CardType::Spades);
    let mut aces_cards = instantiate_cards_of_card_type(CardType::Ace);
    let mut hearts_cards = instantiate_cards_of_card_type(CardType::Hearts);
    let mut diamonds_cards = instantiate_cards_of_card_type(CardType::Diamonds);
    all_cards.append(&mut aces_cards);
    all_cards.append(&mut hearts_cards);
    all_cards.append(&mut diamonds_cards);
    
    // shuffle
    all_cards.shuffle(&mut thread_rng());


    Deck {
        cards: all_cards
    }


}

fn instantiate_cards_of_card_type(cardType: CardType) -> Vec<Card> {
    let mut cards_vec: Vec<Card> = Vec::new();
    for cardNumber in 2..15 {
        let card = Card {
            cardType: cardType,
            cardNumber: cardNumber
        };
        cards_vec.push(card);
    }
    cards_vec
}

