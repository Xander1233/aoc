use std::fs;

#[derive(Clone, Debug)]
struct Card {
    pub i: u32,
    pub winner_nums: Vec<u32>,
    pub nums: Vec<u32>,
    pub clones: u32
}

fn main() {

    let filename = "./src/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut cards: Vec<Card> = Vec::new();

    for line in contents.split('\n') {

        let splitted = line.split(": ").collect::<Vec<&str>>();

        let card_identifier = splitted[0];
        let card_nums = splitted[1];

        let card_identifier: u32 = card_identifier.split(' ').filter(|x| { !x.is_empty() }).collect::<Vec<&str>>()[1].parse::<u32>().unwrap();

        let card_nums = card_nums.split(" | ").collect::<Vec<&str>>();

        let winner_nums = card_nums[0].trim().split(' ').filter(|x| { !x.is_empty() }).map(|x| { x.parse::<u32>().unwrap() }).collect::<Vec<u32>>();

        let nums = card_nums[1].split(' ').filter(|x| { !x.is_empty() }).map(|x| { x.parse::<u32>().unwrap() }).collect::<Vec<u32>>();

        cards.push(Card {
            i: card_identifier,
            winner_nums,
            nums,
            clones: 1
        });
    }

    println!("{}", question2(&mut cards))
}

fn _question1(cards: &Vec<Card>) -> u32 {

    let mut sum: u32 = 0;

    for card in cards {
        let mut total_points = 0;

        for i in card.nums.clone() {
            if card.winner_nums.contains(&i) {
                if total_points == 0 {
                    total_points = 1;
                } else {
                    total_points *= 2;
                }
            }
        }

        sum += total_points;
    }

    sum
}

fn n_card_wins(card: &Card) -> u32 {
    let mut sum = 0;

    for i in card.nums.clone() {
        if card.winner_nums.contains(&i) {
            sum += 1;
        }
    }

    sum
}

fn question2(cards: &mut Vec<Card>) -> usize {

    let mut cards_max = cards.len();

    for i in 0..cards_max {
        let card = cards[i].clone();

        let card_wins: usize = n_card_wins(&card) as usize;

        let start_i: usize = (card.i) as usize;

        for j in start_i..(card_wins + start_i) {
            cards[j].clones += card.clones;
        }

        cards_max += card_wins * card.clones as usize;
    }

    cards_max
}