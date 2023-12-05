use std::fs;

#[derive(Debug)]
struct Number {
    pub string: String,
    pub number: i32
}

fn main() {

    let nums: Vec<Number> = vec!(
        Number { string: String::from("zero"), number: 0 },
        Number { string: String::from("one"), number: 1 },
        Number { string: String::from("two"), number: 2 },
        Number { string: String::from("three"), number: 3 },
        Number { string: String::from("four"), number: 4 },
        Number { string: String::from("five"), number: 5 },
        Number { string: String::from("six"), number: 6 },
        Number { string: String::from("seven"), number: 7 },
        Number { string: String::from("eight"), number: 8 },
        Number { string: String::from("nine"), number: 9 }
    );

    let filename = "./src/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut sum = 0;

    for line in contents.split("\n") {
        let mut left = -1;
        let mut right = -1;

        for i in 0..line.len() {
            let character = line.chars().nth(i).unwrap();
            if !character.is_digit(10) {
                for num in &nums {
                    if i + num.string.len() - 1 < line.len() {
                        if num.string == line[i..i + num.string.len()] {
                            if left == -1 {
                                left = num.number;
                                right = left;
                            } else {
                                right = num.number;
                            }
                        }
                    }
                }
            } else {
                let parsed = character.to_string().parse::<i32>();
                if parsed.is_ok() {
                    if left == -1 {
                        left = parsed.unwrap();
                        right = left;
                    } else {
                        right = parsed.unwrap();
                    }
                }
            }
        }

        sum += left * 10 + right;
    }

    println!("{sum}")
}
