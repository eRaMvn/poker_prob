use clap::{App, Arg};
use colored::*;
use rs_poker::core::{Deck, Hand};
use std::collections::HashMap;
mod calc_prob;

fn get_cli_args() -> (String, String, bool) {
    let matches = App::new("gsheet_writer")
        .version("0.1")
        .author("eRaMvn")
        .about("CLI to write to google sheet given ranges and values")
        .arg(
            Arg::new("my-hand")
                .long("mh")
                .value_name("STRING")
                .help("Set my hand")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("community-cards")
                .long("ch")
                .value_name("STRING")
                .help("Set community cards")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("all-in")
                .short('a')
                .help("Set whether this is all in or not")
                .takes_value(false),
        )
        .get_matches();

    (
        matches.value_of("my-hand").unwrap().to_string(),
        matches.value_of("community-cards").unwrap().to_string(),
        matches.is_present("all-in"),
    )
}

fn main() {
    let (my_cards_arg, community_cards_arg, all_in) = get_cli_args();
    let my_cards =
        Hand::new_from_str(my_cards_arg.as_str()).expect("Should be able to create a hand.");
    let community_cards =
        Hand::new_from_str(community_cards_arg.as_str()).expect("Should be able to create a hand.");
    let deck: Deck = calc_prob::get_unknown_cards(&my_cards, &community_cards);

    let ranks_to_check = HashMap::from([
        ("One Pair", calc_prob::HandRank::OnePair),
        ("Two Pair", calc_prob::HandRank::TwoPair),
        ("Three Of A Kind", calc_prob::HandRank::ThreeOfAKind),
        ("Straight", calc_prob::HandRank::Straight),
        ("Flush", calc_prob::HandRank::Flush),
        ("Full House", calc_prob::HandRank::FullHouse),
    ]);

    let mut outs: i8;
    let mut string_to_print: String;
    let mut four_and_two_prob: i8;
    let mut hand_name_colored: ColoredString;
    let mut prob_string_colored: ColoredString;

    for (name, hand_rank) in ranks_to_check {
        outs = hand_rank.calc_outs(&deck, &my_cards, &community_cards);
        four_and_two_prob =
            calc_prob::calc_4_and_2_probs(all_in, community_cards.len() as i8, outs);

        if four_and_two_prob < 10 {
            hand_name_colored = name.red();
            prob_string_colored = (four_and_two_prob.to_string() + "%").red();
        } else if four_and_two_prob > 10 {
            hand_name_colored = name.green();
            prob_string_colored = (four_and_two_prob.to_string() + "%").green();
        } else {
            hand_name_colored = name.normal();
            prob_string_colored = (four_and_two_prob.to_string() + "%").normal();
        }
        string_to_print = format!(
            "{} has the probability of {}",
            hand_name_colored, prob_string_colored
        );
        println!("{}", string_to_print);
    }

    // Sample usage
    // let flush_outs = calc_prob::HandRank::Flush;
    // println!("{:?}", flush_outs.calc_outs(&deck, &hand, &community));
    // let some_card: Card = Card { value: (Value::King), suit: (Suit::Heart) };
}
