fn extract_numbers(s: &str) -> Vec<u32> {
    return s
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
}

fn extract_big_number(s: &str) -> i64 {
    return s
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
}

pub fn solution(input: &str) -> (String, String) {
    let mut split = input.split_terminator('\n');
    let times: Vec<_> = extract_numbers(split.next().unwrap());
    let distances: Vec<_> = extract_numbers(split.next().unwrap());
    let part_1: u32 = (0..times.len())
        .map(|i| (0..times[i]).filter(|t| t * (times[i] - t) > distances[i]).count() as u32)
        .product();

    split = input.split_terminator('\n');
    let big_time = extract_big_number(split.next().unwrap());
    let big_distance = extract_big_number(split.next().unwrap());

    // big_distance = big_time * x - x^2
    // -x^2 + big_time*x - big_distance = 0
    let square_root = f64::sqrt((big_time*big_time + big_distance * -4) as f64) as i64;
    let lower_bound = (big_time - square_root) / 2;
    let upper_bound = (big_time + square_root) / 2;
    let part_2 = upper_bound - lower_bound;

    return (part_1.to_string(), part_2.to_string());
}
