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
                print!("{}  part_1: {:12} part_2: {:12}", day, part_1, part_2);
            }
        }

        acc = acc / iter as f64;
        total_time += acc;
        if benchmark {
            println!("{:.5}ms", acc / 1000.0);
        } else {
            println!("");
        }
    }

    if benchmark {
        println!("total elapsed time: {:.5}ms", total_time / 1000.0);
    }
}
