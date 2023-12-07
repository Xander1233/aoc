fn main() {

    let filepath = "src/sample.txt";
    let contents = std::fs::read_to_string(filepath).unwrap();

    let mut hands: Vec<(String, u32)> = Vec::new();

    for line in contents.lines() {
        let line_split = line.split(" ").collect::<Vec<&str>>();

        // Replace T, J, Q, K, A with :, ;, <, =, >
        let mut hand = line_split[0].to_string();

        hand = hand.replace("T", ":");
        hand = hand.replace("J", ";");
        hand = hand.replace("Q", "<");
        hand = hand.replace("K", "=");
        hand = hand.replace("A", ">");

        hands.push((hand, line_split[1].parse::<u32>().unwrap()));
    }

    hands.sort_by(|a, b| {
        let a_strength = strength(a.0.clone());
        let b_strength = strength(b.0.clone());
        if a_strength == b_strength {
            return a.0.cmp(&b.0);
        }
        return a_strength.cmp(&b_strength);
    });

    let mut total = 0;

    for i in 0..hands.len() {
        total += hands[i].1 * (i as u32 + 1);
    }

    println!("{}", total);
}

fn strength(hand: String) -> u32 {

    let hand_split = hand.split("").collect::<Vec<&str>>();
    let hand_split = &hand_split[1..(hand_split.len() - 1)];

    let nums: Vec<char> = hand_split.to_vec().iter().map(|x| { x.parse::<char>().unwrap() }).collect::<Vec<char>>();

    let mut count = vec_counter(nums);
    count.sort();

    if count == vec![5] {
        return 10;
    }

    if count == vec![1, 4] {
        return 9;
    }

    if count == vec![2, 3] {
        return 8;
    }

    if count == vec![1, 1, 3] {
        return 7;
    }

    if count == vec![1, 2, 2] {
        return 6;
    }

    if count == vec![1, 1, 1, 2] {
        return 5;
    }

    if count == vec![1, 1, 1, 1, 1] {
        return 4;
    }

    panic!("Invalid hand");
}

fn vec_counter(arr: Vec<char>) -> Vec<u32> {

    let mut counter: Vec<(char, u32)> = Vec::new();

    for i in arr {
        let mut found = false;
        for j in counter.iter_mut() {
            if j.0 == i {
                j.1 += 1;
                found = true;
                break;
            }
        }
        if !found {
            counter.push((i, 1));
        }
    }

    counter.iter().map(|e| { e.1 }).collect::<Vec<u32>>()
}