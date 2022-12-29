#[derive(Copy, Clone)]
enum CardType {
    Spades,
    Hearts,
    Diamonds, 
    Ace
}

struct Card {
    cardType: CardType,
    cardNumber: u32
}


struct Deck {
    cards: Vec<Card>
}

fn main() {
    let deck = instatiate_deck();
    // assert!(deck.cards.len() == 52);
    println!("Length of deck {}", deck.cards.len());
}


fn instatiate_deck() -> Deck {
    let mut all_cards = instantiate_cards_of_card_type(CardType::Spades);
    let mut aces_cards = instantiate_cards_of_card_type(CardType::Ace);
    let mut hearts_cards = instantiate_cards_of_card_type(CardType::Hearts);
    let mut diamonds_cards = instantiate_cards_of_card_type(CardType::Diamonds);
    all_cards.append(&mut aces_cards);
    all_cards.append(&mut hearts_cards);
    all_cards.append(&mut diamonds_cards);

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

