use std::fs;

struct Round {
    pub red: u32,
    pub green: u32,
    pub blue: u32
}

struct Game {
    pub _id: u32,
    pub rounds: Vec<Round>
}

fn main() {

    let filename = "./src/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut games: Vec<Game> = Vec::new();

    for line in contents.split('\n') {

        let splitted = line.split(": ").collect::<Vec<&str>>();

        let game_id = splitted[0];
        let game_rounds = splitted[1];

        let game_id = game_id.split_ascii_whitespace().nth(1).unwrap().parse::<u32>().unwrap();

        let game_rounds = game_rounds.split("; ").collect::<Vec<&str>>();

        let mut rounds: Vec<Round> = Vec::new();

        for round in game_rounds {

            let cubes_round = round.split(", ").collect::<Vec<&str>>();

            let mut round_obj = Round {
                red: 0,
                green: 0,
                blue: 0
            };

            for cube in cubes_round {

                let splitted = cube.split_ascii_whitespace().collect::<Vec<&str>>();

                let count = splitted[0].parse::<u32>().unwrap();
                let color = splitted[1];

                match color {
                    "green" => round_obj.green = count,
                    "blue" => round_obj.blue = count,
                    "red" => round_obj.red = count,
                    _ => {}
                };
            }

            rounds.push(round_obj);
        }

        games.push(Game {
            _id: game_id,
            rounds
        });
    }

    let mut sum: u32 = 0;

    for game in games {

        let mut max_blue: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_red: u32 = 0;

        for rounds in game.rounds {
            if rounds.blue > max_blue {
                max_blue = rounds.blue;
            }

            if rounds.red > max_red {
                max_red = rounds.red;
            }

            if rounds.green > max_green {
                max_green = rounds.green;
            }
        }

        sum += max_blue * max_green * max_red;
    }

    println!("{}", sum);
}
