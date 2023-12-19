mod aoc;

use std::time::Instant;
use std::fs;
use std::env;

fn main() {
    let args : Vec<_> = env::args().collect();
    let days = Vec::from([
        ("d01", aoc::d01 as fn(&str) -> (String, String)),
        ("d02", aoc::d02),
        ("d03", aoc::d03),
        ("d04", aoc::d04),
        ("d05", aoc::d05),
        ("d06", aoc::d06),
        ("d07", aoc::d07),
        ("d08", aoc::d08),
        ("d09", aoc::d09),
        ("d10", aoc::d10),
        ("d11", aoc::d11),
        ("d12", aoc::d12),
        ("d13", aoc::d13),
        ("d14", aoc::d14),
        ("d15", aoc::d15),
        ("d16", aoc::d16),
        ("d17", aoc::d17),
        ("d18", aoc::d18),
        ("d19", aoc::d19),
    ]);

    let mut to_run : Vec<_> = days.iter().filter(|(s,_)| args.iter().any(|ss| s == ss)).collect();
    if to_run.len() == 0 {
        to_run = days.iter().collect();
    }

    let benchmark = args.iter().any(|s| s == "-bench");
    let mut total_time = 0.0;
    for (day, func) in to_run {
        let iter = if benchmark { 50 } else { 1 };
        let mut acc : f64 = 0.0;
        let input = fs::read_to_string(format!("inputs/{}.txt", day)).expect("unable to read file");
        
        for i in 0..iter {
            let now = Instant::now();
            let (part_1, part_2) = func(&input);
            acc += now.elapsed().as_micros() as f64;

            if i == 0 {
                print!("\x1b[92m{}\x1b[0m -- part_1: {:>10} - part_2: {:>15}", day.to_uppercase(), part_1, part_2);
            }
        }

        acc = acc / iter as f64;
        total_time += acc;
        if benchmark {
            println!(" - {:>8.5}ms", acc / 1000.0);
        } else {
            println!("");
        }
    }

    if benchmark {
        println!("{:42}\x1b[1m\x1b[96mTotal Time\x1b[0m: {:>.5}ms", "", total_time / 1000.0);
    }
}
