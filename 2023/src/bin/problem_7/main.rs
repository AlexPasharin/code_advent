#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

// use regex::Regex;
use utils::file_reader::FileReader;

#[derive(Ord, PartialEq, PartialOrd, Eq, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

#[derive(Ord, PartialEq, PartialOrd, Eq)]
enum Type {
    HighCard,
    Pair,
    TwoPairs,
    Three,
    House,
    Four,
    Five,
}

struct Hand {
    cards: Vec<Card>,
    hand_type: Type,
    bid: u32,
}

fn main() {
    let mut hands = Vec::new();

    FileReader::process_lines("./input/problem7.txt", &mut |line| {
        let mut line_parts = line.split_whitespace();

        let cards = line_parts.next().unwrap();
        let bid = line_parts.next().unwrap().parse::<u32>().unwrap();

        let cards = cards
            .chars()
            .map(|ch| match ch {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => Card::J,
                'Q' => Card::Q,
                'K' => Card::K,
                'A' => Card::A,
                _ => panic!("Char {} does not correspond to a card", ch),
            })
            .collect::<Vec<_>>();

        assert_eq!(cards.len(), 5);

        let card_counts = card_counts(&cards);

        let mut card_counts = card_counts.values().copied().collect::<Vec<_>>();

        card_counts.sort_unstable_by(|a, b| b.cmp(a));

        let hand_type = hand_type(card_counts);

        hands.push(Hand {
            cards,
            hand_type,
            bid,
        })
    });

    let total_winning = calc_winnings(&mut hands);

    println!("Part 1 total winning: {}", total_winning); // 249483956

    hands = hands
        .into_iter()
        .map(|hand| {
            let cards = hand
                .cards
                .into_iter()
                .map(|card| if card == Card::J { Card::Joker } else { card })
                .collect();

            let mut card_counts = card_counts(&cards);

            let joker_count = card_counts.remove(&Card::Joker).unwrap_or(0);

            let mut card_counts = card_counts.values().copied().collect::<Vec<_>>();

            card_counts.sort_unstable_by(|a, b| b.cmp(a));

            if card_counts.is_empty() {
                card_counts.push(0);
            }

            card_counts[0] += joker_count;

            let hand_type = hand_type(card_counts);

            Hand {
                cards,
                hand_type,
                ..hand
            }
        })
        .collect();

    let total_winning = calc_winnings(&mut hands);

    println!("Part 2 total winning: {}", total_winning); // 252137472
}

fn card_counts(cards: &Vec<Card>) -> HashMap<&Card, i32> {
    cards.iter().fold(HashMap::new(), |mut result, card| {
        let entry = result.entry(card).or_insert(0);

        *entry += 1;

        result
    })
}

fn hand_type(card_counts: Vec<i32>) -> Type {
    if card_counts[0] == 5 {
        Type::Five
    } else if card_counts[0] == 4 {
        Type::Four
    } else if card_counts[0] == 3 {
        if card_counts[1] == 2 {
            Type::House
        } else {
            Type::Three
        }
    } else if card_counts[0] == 2 {
        if card_counts[1] == 2 {
            Type::TwoPairs
        } else {
            Type::Pair
        }
    } else {
        Type::HighCard
    }
}

fn calc_winnings(hands: &mut Vec<Hand>) -> u32 {
    hands.sort_unstable_by(|hand1, hand2| {
        hand1
            .hand_type
            .cmp(&hand2.hand_type)
            .then(hand1.cards.cmp(&hand2.cards))
    });

    hands
        .iter()
        .zip(1..)
        .fold(0, |total, (hand, rank)| total + hand.bid * rank)
}
