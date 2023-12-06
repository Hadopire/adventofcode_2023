pub fn solution(input: &str) -> (String, String) {
    let lines = input.split_terminator('\n').map(|s| &s[s.find(": ").unwrap() + 2..]);
    let matches: Vec<_> = lines
        .map(|line| {
            let mut split = line.split('|');
            let win: Vec<_> = split.next().unwrap().split_whitespace().collect();
            return split.next().unwrap().split_whitespace().filter(|d| win.contains(d)).count()
                as u32;
        })
        .collect();

    let mut card_count = vec![1u32; matches.len()];
    for i in 0..card_count.len() {
        for j in 1..matches[i] + 1 {
            card_count[i + j as usize] += card_count[i];
        }
    }

    let part_1 = matches.iter().filter(|e| **e > 0).fold(0, |acc, e| acc + (1 << (e - 1)));
    let part_2 = card_count.iter().sum::<u32>();

    return (part_1.to_string(), part_2.to_string());
}
