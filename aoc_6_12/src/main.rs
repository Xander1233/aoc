#[derive(Debug)]
struct Race {
    pub time: u128,
    pub distance: u128
}

fn main() {

    let filepath = "src/sample.txt";
    let contents = std::fs::read_to_string(filepath).unwrap().leak();

    let lines = contents.split('\n').map(|x| { x.split(":").nth(1).unwrap().trim() }).collect::<Vec<&str>>();

    let time_line = lines[0].split_ascii_whitespace().collect::<Vec<&str>>().join("");
    let distance_line = lines[1].split_ascii_whitespace().collect::<Vec<&str>>().join("");

    let race = Race {
        time: time_line.parse::<u128>().unwrap(),
        distance: distance_line.parse::<u128>().unwrap()
    };

    let mut distances: Vec<u128> = Vec::new();

    for i in 1..race.time {

        let distance = (race.time - i) * i;

        distances.push(distance);
    }

    let total = distances.iter().filter(|x| { x > &&race.distance }).count() as u128;

    println!("{}", total);
}
