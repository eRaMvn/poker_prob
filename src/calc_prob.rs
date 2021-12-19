use rs_poker::core::{Card, Deck, Hand, Rankable, Suit, Value};
use std::collections::HashMap;

pub fn calc_4_and_2_probs(all_in: bool, num_community_cards: i8, outs: i8) -> i8 {
    if outs < 0 {
        return 0;
    }

    if num_community_cards >= 4 {
        if all_in {
            return outs * 4;
        }
    }
    outs * 2
}

// Function to remove cards in hand and community from a brand new deck
pub fn get_unknown_cards(hand: &Hand, community: &Hand) -> Deck {
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

fn get_one_pair_outs(hand: &Hand, community: &Hand) -> i8 {
    let (_, card_values) = count_suit_and_value_on_table(&hand, &community);

    for (_, &count) in card_values.iter() {
        // If count of values of cards on table == 2, that means we already a pair, return outs = 0, no calculation needed
        if count == 2 {
            return 0;
        }
    }
    // There are four suits for each card, so 3 outs for each card
    3
}

fn get_two_pairs_outs(hand: &Hand, community: &Hand) -> i8 {
    let (_, card_values) = count_suit_and_value_on_table(&hand, &community);
    let mut one_pair_found = false;
    let mut second_pair_found = false;

    for (_, &count) in card_values.iter() {
        // If count of values of cards on table == 4 (full suits of a card value), that means we already have two pairs, return outs = 0, no calculation needed
        if count == 4 {
            return 0;
        }

        // When there is a pair already
        if count == 2 {
            if one_pair_found == false {
                one_pair_found = true;
                continue;
            }
            if one_pair_found {
                second_pair_found = true;
                continue;
            }
        }
    }

    if one_pair_found && second_pair_found {
        return 0;
    }

    // If we already have a pair, we just have to get another pair, 3 outs for each card left
    if one_pair_found {
        return 3;
    }
    // 3 outs for each card, need two cards for a pair
    hand.len() as i8 * 3
}

fn get_three_of_a_kind_outs(hand: &Hand, community: &Hand) -> i8 {
    let (_, card_values) = count_suit_and_value_on_table(&hand, &community);

    for (_, &count) in card_values.iter() {
        // If count of values of cards on table == 3, that means we already have a set, return outs = 0, no calculation needed
        if count >= 3 {
            return 0;
        }

        // If we already have a pair, we just need one more card, 2 outs
        if count == 2 {
            return 2;
        }
    }
    // If we have no pair in current hand, then we need at least two more cards
    3
}

fn get_straight_outs(hand: &Hand, community: &Hand) -> i8 {
    let (_, card_values) = count_suit_and_value_on_table(&hand, &community);
    let mut value_vector = Vec::new();

    for (&value, _) in card_values.iter() {
        value_vector.push(value as i8);
    }

    value_vector.sort();
    let mut num_in_sequence = 1;

    for (i, each_value) in value_vector.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if each_value - 1 == value_vector[i - 1] {
            num_in_sequence += 1
        }
    }

    // If there are 3 community cards and the num of consecutive card is not 3, then there is no chance for straight
    if community.len() as i8 == 3 && num_in_sequence < 3 {
        return -1;
    }

    // If there are 3 community cards and the num of consecutive card is not 3, then there is no chance for straight
    if community.len() as i8 == 4 && num_in_sequence < 4 {
        return -1;
    }

    // We only need 5 cards to have straight, but from 4 suits
    (5 - num_in_sequence) * 4
}

fn get_flush_outs(deck: &Deck, hand: &Hand, community: &Hand) -> i8 {
    let (card_suits, _) = count_suit_and_value_on_table(&hand, &community);

    let mut num_of_highest_suit_outs: i8 = -1;
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

fn get_full_house_outs(hand: &Hand, community: &Hand) -> i8 {
    let (_, card_values) = count_suit_and_value_on_table(&hand, &community);
    let mut one_pair_found = false;
    let mut second_pair_found = false;
    let mut set_found = false;

    for (_, &count) in card_values.iter() {
        // If count of values of cards on table == 4 (full suits of a card value), that means we already have two pairs, only need 1 more card (2 outs) for full house
        if count == 4 {
            return 2 * 2;
        }

        // If we already one pair
        if count == 2 {
            if set_found {
                return 0;
            }
            if one_pair_found == false {
                one_pair_found = true;
                continue;
            }
            if one_pair_found {
                second_pair_found = true;
                continue;
            }
        }

        if count == 3 {
            if one_pair_found {
                return 0;
            }
            set_found = true;
        }
    }

    if one_pair_found && second_pair_found {
        return 2 * 2;
    }

    // If we already have a pair, we have to get both another pair + 1 other card of the same value with the pair we have or 3 other cards of the same value
    if one_pair_found {
        // Two possibilities but we go with the one with higher outs:
        // 3 outs for a set
        // 3 outs for a pair + 2 outs for 1 other card
        return 2 + 3;
    }
    // If none found
    3 + 3
}

pub enum HandRank {
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
}

impl HandRank {
    pub fn calc_outs(self, deck: &Deck, hand: &Hand, community: &Hand) -> i8 {
        match self {
            Self::OnePair => get_one_pair_outs(hand, community),
            Self::TwoPair => get_two_pairs_outs(hand, community),
            Self::ThreeOfAKind => get_three_of_a_kind_outs(hand, community),
            Self::Straight => get_straight_outs(hand, community),
            Self::Flush => get_flush_outs(deck, hand, community),
            Self::FullHouse => get_full_house_outs(hand, community),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_4_and_2_probs_1() {
        assert_eq!(calc_4_and_2_probs(true, 3, 2), 4);
    }

    #[test]
    fn test_calc_4_and_2_probs_2() {
        assert_eq!(calc_4_and_2_probs(true, 4, 2), 8);
    }

    #[test]
    fn test_calc_4_and_2_probs_3() {
        assert_eq!(calc_4_and_2_probs(false, 4, 2), 4);
    }

    #[test]
    fn test_calc_4_and_2_probs_4() {
        assert_eq!(calc_4_and_2_probs(false, 4, -1), 0);
    }

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

    // Testing when we have at least one pair, outs should 0, pair at the top
    #[test]
    fn test_existing_one_pair_1() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("Ah8c3s4s").unwrap();
        assert_eq!(get_one_pair_outs(&hand, &community), 0);
    }

    #[test]
    fn test_existing_one_pair_2() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("2h8cks4s").unwrap();
        assert_eq!(get_one_pair_outs(&hand, &community), 0);
    }

    // Testing when we have at least one pair, outs should 0, pair at the middle
    #[test]
    fn test_one_pair_outs_2() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("7h8cks4s").unwrap();
        assert_eq!(get_one_pair_outs(&hand, &community), 0);
    }

    // Testing when we have at least one pair, outs should 0, pair at the bottom
    #[test]
    fn test_one_pair_outs_3() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("7h8cks").unwrap();
        assert_eq!(get_one_pair_outs(&hand, &community), 0);
    }

    // Testing when we have at least one pair, outs should 0, pair at the bottom
    #[test]
    fn test_one_pair_outs_4() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("7h8cAs").unwrap();
        assert_eq!(get_one_pair_outs(&hand, &community), 0);
    }

    // Testing when we have no pair
    #[test]
    fn test_one_pair_outs_5() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("7h8c2s").unwrap();
        assert_eq!(get_one_pair_outs(&hand, &community), 3);
    }

    // Testing when we have two pairs already
    #[test]
    fn test_existing_two_pairs_1() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("AsKd2s").unwrap();
        assert_eq!(get_two_pairs_outs(&hand, &community), 0);
    }

    #[test]
    fn test_existing_two_pairs_2() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("As4cKd2s").unwrap();
        assert_eq!(get_two_pairs_outs(&hand, &community), 0);
    }

    // Test when we already have one pair
    #[test]
    fn test_one_in_two_pairs_1() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("As4c5d2s").unwrap();
        assert_eq!(get_two_pairs_outs(&hand, &community), 3);
    }

    // Test when we already have one pair
    #[test]
    fn test_one_in_two_pairs_2() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("3skd4c").unwrap();
        assert_eq!(get_two_pairs_outs(&hand, &community), 3);
    }

    // Test when we have no pair
    #[test]
    fn test_two_pairs_outs_1() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("3s4cqd2s").unwrap();
        assert_eq!(get_two_pairs_outs(&hand, &community), 6);
    }

    // Test when we have already have three of a kind
    #[test]
    fn test_existing_three_of_a_kind_1() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("As4cqdAc").unwrap();
        assert_eq!(get_three_of_a_kind_outs(&hand, &community), 0);
    }

    // Test when we have already have three of a kind
    #[test]
    fn test_existing_three_of_a_kind_2() {
        let hand = Hand::new_from_str("AdAh").unwrap();
        let community = Hand::new_from_str("As4cqdAc").unwrap();
        assert_eq!(get_three_of_a_kind_outs(&hand, &community), 0);
    }

    // Test when we have already have a pair
    #[test]
    fn test_three_of_a_kind_outs_1() {
        let hand = Hand::new_from_str("AdAh").unwrap();
        let community = Hand::new_from_str("2s4cqd4h").unwrap();
        assert_eq!(get_three_of_a_kind_outs(&hand, &community), 2);
    }

    // Testing when we have no chance of having straight
    #[test]
    fn test_impossible_straight_1() {
        let hand = Hand::new_from_str("7dkh").unwrap();
        let community = Hand::new_from_str("Jd2c3s").unwrap();
        assert_eq!(get_straight_outs(&hand, &community), -1);
    }

    #[test]
    fn test_impossible_straight_2() {
        let hand = Hand::new_from_str("7dkh").unwrap();
        let community = Hand::new_from_str("Jd2c3s4s").unwrap();
        assert_eq!(get_straight_outs(&hand, &community), -1);
    }

    #[test]
    fn test_straight_outs_1() {
        let hand = Hand::new_from_str("7d3s").unwrap();
        let community = Hand::new_from_str("Jd2s4s5d").unwrap();
        assert_eq!(get_straight_outs(&hand, &community), 4);
    }

    #[test]
    fn test_straight_outs_2() {
        let hand = Hand::new_from_str("7d3s").unwrap();
        let community = Hand::new_from_str("Jd2s4s").unwrap();
        assert_eq!(get_straight_outs(&hand, &community), 8);
    }

    #[test]
    fn test_straight_outs_3() {
        let hand = Hand::new_from_str("7d3s").unwrap();
        let community = Hand::new_from_str("3d2h4s").unwrap();
        assert_eq!(get_straight_outs(&hand, &community), 8);
    }

    // Testing when we have 4 community cards already, but number of cards of same suits is less than 4
    #[test]
    fn test_impossible_flush_outs_1() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("Jd8c3s4s").unwrap();
        let deck: Deck = get_unknown_cards(&hand, &community);
        assert_eq!(get_flush_outs(&deck, &hand, &community), -1);
    }

    // Testing when we have the flops (3 community cards), but number of cards of same suits is less than 3
    #[test]
    fn test_impossible_flush_outs_2() {
        let hand = Hand::new_from_str("Adkh").unwrap();
        let community = Hand::new_from_str("Jd8c3s").unwrap();
        let deck: Deck = get_unknown_cards(&hand, &community);
        assert_eq!(get_flush_outs(&deck, &hand, &community), -1);
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

    // In this case, we have two card with the same value in hand
    #[test]
    fn test_correct_flush_outs_3() {
        let hand = Hand::new_from_str("AdAh").unwrap();
        let community = Hand::new_from_str("Jd8d3d").unwrap();
        let deck: Deck = get_unknown_cards(&hand, &community);
        assert_eq!(get_flush_outs(&deck, &hand, &community), 9);
    }

    // In this case, we have two card with the same value
    #[test]
    fn test_correct_flush_outs_4() {
        let hand = Hand::new_from_str("Ad3d").unwrap();
        let community = Hand::new_from_str("Jd8dAh").unwrap();
        let deck: Deck = get_unknown_cards(&hand, &community);
        assert_eq!(get_flush_outs(&deck, &hand, &community), 9);
    }

    // In this case, we have a pair and a set already
    #[test]
    fn test_existing_full_house_1() {
        let hand = Hand::new_from_str("Ad3d").unwrap();
        let community = Hand::new_from_str("3s3hAh").unwrap();
        assert_eq!(get_full_house_outs(&hand, &community), 0);
    }

    #[test]
    fn test_existing_full_house_2() {
        let hand = Hand::new_from_str("AdAh").unwrap();
        let community = Hand::new_from_str("3s3h4h3c").unwrap();
        assert_eq!(get_full_house_outs(&hand, &community), 0);
    }

    // When there are two pairs already
    #[test]
    fn test_full_house_outs_1() {
        let hand = Hand::new_from_str("AdAh").unwrap();
        let community = Hand::new_from_str("4h3c3h").unwrap();
        assert_eq!(get_full_house_outs(&hand, &community), 4);
    }

    // When there are two pairs already
    #[test]
    fn test_full_house_outs_2() {
        let hand = Hand::new_from_str("Ad3h").unwrap();
        let community = Hand::new_from_str("4h3c5cAh").unwrap();
        assert_eq!(get_full_house_outs(&hand, &community), 4);
    }

    // When there is one pair already
    #[test]
    fn test_full_house_outs_3() {
        let hand = Hand::new_from_str("Ad3h").unwrap();
        let community = Hand::new_from_str("4h3c5c6h").unwrap();
        assert_eq!(get_full_house_outs(&hand, &community), 5);
    }

    // When there is one pair already
    #[test]
    fn test_full_house_outs_4() {
        let hand = Hand::new_from_str("4sAd").unwrap();
        let community = Hand::new_from_str("4h3c5c").unwrap();
        assert_eq!(get_full_house_outs(&hand, &community), 5);
    }

    // When there is nothing
    #[test]
    fn test_full_house_outs_5() {
        let hand = Hand::new_from_str("4sAd").unwrap();
        let community = Hand::new_from_str("6h3c5c").unwrap();
        assert_eq!(get_full_house_outs(&hand, &community), 6);
    }
}
