use std::cmp::min;
use std::thread;
use std::thread::JoinHandle;

fn main() {

    let file_path = "src/sample.txt";
    let contents = std::fs::read_to_string(file_path).unwrap().leak();

    let mut maps = contents.split("\n\n").collect::<Vec<&str>>();

    let seeds_string = maps[0].split(": ").nth(1).unwrap().split_ascii_whitespace().map(|x| { x.parse::<u64>().unwrap() }).collect::<Vec<u64>>();
    maps.remove(0);

    let mut seeds: Vec<u64> = Vec::new();

    let mut handlers: Vec<JoinHandle<Vec<u64>>> = Vec::new();

    for i in seeds_string {
        seeds.push(i);
    }

    let mut lowest = u64::MAX;

    println!("{} {:?}", lowest, seeds);

    let mut handlers: Vec<JoinHandle<u64>> = Vec::new();

    for i in 0..seeds.len() {
        let seed = seeds[i];
        let map = maps.clone();

        let thr = thread::spawn(move || -> u64 {
            let res = calc(&seed, &map);

            println!("{}", res);

            res
        });

        handlers.push(thr);
    }

    for handler in handlers {
        let thr_result = handler.join().unwrap();

        if thr_result < lowest {
            lowest = thr_result;
        }

        println!("{}", lowest);
    }

    println!("{}", lowest);
}

pub fn calc(seed: &u64, maps: &Vec<&str>) -> u64 {

    let mut seed = seed.clone();

    for map in maps {

        let splitted = map.split(":\n").collect::<Vec<&str>>();

        let ranges = splitted[1].split('\n').collect::<Vec<&str>>();

        for i in 0..ranges.len() {
            let range = ranges[i];

            let nums = range.split_ascii_whitespace().map(|x| { x.parse::<i64>().unwrap() }).collect::<Vec<i64>>();

            let source = nums[1];
            let dest = nums[0];
            let range = nums[2];

            if seed >= source as u64 && seed <= (source + range - 1) as u64 {

                let diff = dest - source;

                if diff < 0 {
                    seed -= (-diff) as u64;
                } else {
                    seed += diff as u64;
                }

                break
            }
        }
    }

    seed
}