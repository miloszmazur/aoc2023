use std::{cmp::Ordering, collections::HashSet};

fn main() {
    println!("boat races!");
    let input = include_str!("../../inputs/input.txt");
    dbg!(part2(input.to_string()));
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum HandStrength {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandStrength {
    fn from_cards(cards: &[u8; 5]) -> Self {
        let mut sorted_cards = cards.clone();
        sorted_cards.sort();
        match longest_repeat_in_cards(&sorted_cards) {
            5 => Self::FiveOfAKind,
            4 => Self::FourOfAKind,
            3 => full_house_or_thrips(sorted_cards),
            2 => two_pair_or_pair(sorted_cards),
            1 => Self::HighCard,
            _ => panic!("AAAAA"),
        }
    }
}

fn two_pair_or_pair(cards: [u8; 5]) -> HandStrength {
    let haha = HashSet::from(cards);
    if haha.len() == 3 ||  haha.len() == 4 && haha.contains(&0) {
        HandStrength::TwoPair
    } else {
        HandStrength::OnePair
    }
}

fn full_house_or_thrips(cards: [u8; 5]) -> HandStrength {
    let haha = HashSet::from(cards);
    if haha.len() == 2 || haha.len() == 3 && haha.contains(&0) {
        HandStrength::FullHouse
    } else {
        HandStrength::ThreeOfAKind
    }
}

fn longest_repeat_in_cards(cards: &[u8; 5]) -> u8 {
    let mut max_cards = 0;
    let mut current_occurences: u8 = 0;
    let mut current_value = cards[0];
    let mut jokers = 0;

    for index in 0..5 {
        if cards[index] == 0 {
            jokers += 1;
            current_value = 0;
            continue;
        }
        if current_value == 0 {
            current_value = cards[index];
        }
        if cards[index] == current_value {
            current_occurences += 1;
        } else {
            current_occurences = 1;
            current_value = cards[index];
        }
        if current_occurences > max_cards {
            max_cards = current_occurences;
        }
    }
    max_cards + jokers
}

#[derive(Eq, Debug)]
struct Hand {
    cards: [u8; 5],
    bid: u32,
    strength: HandStrength,
}

impl Hand {
    fn new(cards: [u8; 5], bid: u32) -> Hand {
        Hand {
            cards,
            bid,
            strength: HandStrength::from_cards(&cards),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.strength != other.strength {
            other.strength.cmp(&self.strength)
        } else {
            for index in 0..5 {
                if self.cards[index] != other.cards[index] {
                    return self.cards[index].cmp(&other.cards[index]);
                }
            }
            Ordering::Equal
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

fn part2(input: String) -> u32 {
    let mut hands = parse_hands(input);
    hands.sort();
    dbg!(&hands);
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) as u32 * hand.bid)
        .sum()
}

fn parse_hands(input: String) -> Vec<Hand> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let (cards, bid) = line.split_once(" ").expect("aaa");
            let parsed_cards: [u8; 5] = cards
                .chars()
                .map(|card| match card {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 0,
                    'T' => 10,
                    other => other.to_digit(10).unwrap() as u8,
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let bid = bid.parse().unwrap();
            Hand::new(parsed_cards, bid)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part2_works() {
        let input = "\
            32T3K 765\n\
            T55J5 684\n\
            KK677 28\n\
            KTJJT 220\n\
            QQQJA 483\n\
           ";
        assert_eq!(part2(input.to_string()), 5905);
    }
}
