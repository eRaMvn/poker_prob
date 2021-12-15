use clap::{crate_authors, App, Arg};
use rs_poker::core::{Card, Deck, Hand, Rankable, Suit, Value};
use std::collections::HashMap;

fn get_cli_args() {
    let matches = App::new("gsheet_writer")
        .version("0.1")
        .author(crate_authors!(""))
        .about("CLI to write to google sheet given ranges and values")
        .arg(
            Arg::new("current-hand")
                .long("ch")
                .value_name("STRING")
                .about("Set current hand")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("num-of-players")
                .short('n')
                .long("num-of-players")
                .value_name("STRING")
                .about("Set number of players")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("flop")
                .short('f')
                .long("flop")
                .value_name("STRING")
                .about("Set current flop")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let current_hand = matches.value_of("current-hand").unwrap();
    println!("Value for current_hand: {}", current_hand);
    let num_of_players = matches.value_of("num-of-players").unwrap();
    println!("Value for num_of_players: {}", num_of_players);
    let flop = matches.value_of("flop").unwrap();
    println!("Value for flop: {}", flop);
}

fn get_unknown_cards(hand: &Hand, community: &Hand) -> Deck {
    // Initial deck with 52 cards
    let mut deck = Deck::default();

    // Remove cards in hand from deck
    let mut temp_card: Card;
    for card in hand.cards() {
        temp_card = card.clone();
        deck.remove(temp_card);
    }

    // Remove community cards from deck
    for card in community.cards() {
        temp_card = card.clone();
        deck.remove(temp_card);
    }

    deck
}

// Given a hand, count the number of card with the same suit or value
fn count_suit_and_value_on_hand(
    hand: &Hand,
    card_suits: &mut HashMap<Suit, i8>,
    card_values: &mut HashMap<Value, i8>,
) -> () {
    for card in hand.cards() {
        card_suits.entry(card.suit).or_insert(0);
        card_suits.insert(card.suit, card_suits[&card.suit] + 1);

        card_values.entry(card.value).or_insert(0);
        card_values.insert(card.value, card_values[&card.value] + 1);
    }
}

// Count the number of card with the same suit or value from all cards on table
fn count_suit_and_value_on_table(
    hand: &Hand,
    community: &Hand,
) -> (HashMap<Suit, i8>, HashMap<Value, i8>) {
    let mut card_suits = HashMap::new();
    let mut card_values = HashMap::new();

    // Going through the cards in the hand to count number of cards based on their suits
    count_suit_and_value_on_hand(hand, &mut card_suits, &mut card_values);
    // Going through the community card to count number of cards based on their suits
    count_suit_and_value_on_hand(community, &mut card_suits, &mut card_values);
    (card_suits, card_values)
}

fn get_high_card_outs() -> i8 {
    0
}
fn get_one_pair_outs() -> i8 {
    0
}

fn get_two_pairs_outs() -> i8 {
    0
}
fn get_three_of_a_kind_outs() -> i8 {
    0
}
fn get_straight_outs() -> i8 {
    0
}

fn get_flush_outs(deck: &Deck, hand: &Hand, community: &Hand) -> i8 {
    let (card_suits, _) = count_suit_and_value_on_table(&hand, &community);

    let mut num_of_highest_suit_outs: i8 = 0;
    let mut outs: i8;

    for (&suit, &count) in card_suits.iter() {
        // When there is more than or equal to 4 community cards, but the count of suits is less then 4, skip the suit
        if community.len() >= 4 && count < 4 {
            continue;
        }

        // When there is less than or equal to 3 community cards, but the count of suits is less then 3, skip the suit
        if community.len() <= 3 && count < 3 {
            continue;
        }

        outs = 0;
        // For the remaining suits, get the number of cards with that suit
        for card in deck.iter() {
            if card.suit == suit {
                outs += 1;
            }
        }
        if outs > num_of_highest_suit_outs {
            num_of_highest_suit_outs = outs
        }
    }

    num_of_highest_suit_outs
}
fn get_full_house_outs() -> i8 {
    0
}
fn get_four_of_a_kind_outs() -> i8 {
    0
}
fn get_straight_flush_outs() -> i8 {
    0
}

enum HandRank {
    /// The lowest rank.
    /// No matches
    HighCard,
    /// One Card matches another.
    OnePair,
    /// Two different pair of matching cards.
    TwoPair,
    /// Three of the same value.
    ThreeOfAKind,
    /// Five cards in a sequence
    Straight,
    /// Five cards of the same suit
    Flush,
    /// Three of one value and two of another value
    FullHouse,
    /// Four of the same value.
    FourOfAKind,
    /// Five cards in a sequence all for the same suit.
    StraightFlush,
}

impl HandRank {
    pub fn calc_4_and_2_probs() {}
    pub fn calc_outs(self, deck: &Deck, hand: &Hand, community: &Hand) -> i8 {
        match self {
            Self::HighCard => get_high_card_outs(),
            Self::OnePair => get_one_pair_outs(),
            Self::TwoPair => get_two_pairs_outs(),
            Self::ThreeOfAKind => get_three_of_a_kind_outs(),
            Self::Straight => get_straight_outs(),
            Self::Flush => get_flush_outs(deck, hand, community),
            Self::FullHouse => get_full_house_outs(),
            Self::FourOfAKind => get_four_of_a_kind_outs(),
            Self::StraightFlush => get_straight_flush_outs(),
        }
    }
}

fn main() {
    // get_cli_args()
    // let hands: Vec<Hand> = ["Adkh", "8c8s"]
    //     .iter()
    //     .map(|s| Hand::new_from_str(s).expect("Should be able to create a hand."))
    //     .collect();

    // println!("{:?}", hand.cards());
    // println!("{:?}", board);

    let hand = Hand::new_from_str("Adkh").expect("Should be able to create a hand.");
    let community = Hand::new_from_str("Jd8c3d").expect("Should be able to create a hand.");

    let deck: Deck = get_unknown_cards(&hand, &community);
    let flush_outs = HandRank::Flush;
    flush_outs.calc_outs(&deck, &hand, &community);

    // println!("{:?}", deck.len());
    // println!("{:?}", deck);
    // let some_card: Card = Card { value: (Value::King), suit: (Suit::Heart) };
    // println!("{:?}", deck.contains(some_card));

    // for each_card in deck.iter(){
    //     println!("{:?}",each_card)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_num_of_unknown_cards() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("Jd8c3d").unwrap();
        assert_eq!(get_unknown_cards(&hand, &community).len(), 47);
    }

    #[test]
    fn test_incorrect_num_of_unknown_cards() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("Jd8c3d4s").unwrap();
        assert_ne!(get_unknown_cards(&hand, &community).len(), 47);
    }

    // Testing if we have the correct number of count of suits and values on a hand
    #[test]
    fn test_count_suit_and_value_on_hand() {
        let mut card_suits = HashMap::new();
        let mut card_values = HashMap::new();
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("Jd8c3d").unwrap();
        count_suit_and_value_on_hand(&hand, &mut card_suits, &mut card_values);
        count_suit_and_value_on_hand(&community, &mut card_suits, &mut card_values);
        assert_eq!(card_suits[&Suit::Diamond], 3);
        assert_eq!(card_suits[&Suit::Heart], 1);
        assert_eq!(card_suits[&Suit::Club], 1);
        assert_eq!(card_values[&Value::Three], 1);
        assert_eq!(card_values[&Value::Eight], 1);
        assert_eq!(card_values[&Value::Ace], 1);
        assert_eq!(card_values[&Value::Jack], 1);
        assert_eq!(card_values[&Value::King], 1);
    }

    // Testing when we have 4 community cards already, but number of cards of same suits is less than 4
    #[test]
    fn test_impossible_flush_outs_1() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("Jd8c3s4s").unwrap();
        let deck: Deck = get_unknown_cards(&hand, &community);
        assert_eq!(get_flush_outs(&deck, &hand, &community), 0);
    }

    // Testing when we have the flops (3 community cards), but number of cards of same suits is less than 3
    #[test]
    fn test_impossible_flush_outs_2() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("Jd8c3s").unwrap();
        let deck: Deck = get_unknown_cards(&hand, &community);
        assert_eq!(get_flush_outs(&deck, &hand, &community), 0);
    }

    // In this case, we have 3 diamond suited cards out of 13 diamond suited cards, the correct number should be 10
    #[test]
    fn test_correct_flush_outs_1() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("Jd8c3d").unwrap();
        let deck: Deck = get_unknown_cards(&hand, &community);
        assert_eq!(get_flush_outs(&deck, &hand, &community), 10);
    }

    // In this case, we have 4 diamond suited cards out of 13 diamond suited cards, the correct number should be 9
    #[test]
    fn test_correct_flush_outs_2() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("Jd8d3d").unwrap();
        let deck: Deck = get_unknown_cards(&hand, &community);
        assert_eq!(get_flush_outs(&deck, &hand, &community), 9);
    }
}
