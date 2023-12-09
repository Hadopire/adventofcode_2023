pub fn solution(input: &str) -> (String, String) {
    let histories: Vec<_> = input
        .split_terminator('\n')
        .map(|s| {
            s.split_whitespace().map(|number| number.parse::<i32>().unwrap()).collect::<Vec<_>>()
        })
        .collect();
    let max_history_count = histories.iter().fold(0, |acc, v| acc.max(v.len()));

    let mut part_1 = 0;
    let mut part_2 = 0;

    let mut differences = vec![0; max_history_count];
    for history in &histories {
        let history_count = history.len();
        let mut difference_count = history_count;
        let mut all_zero = false;

        differences[..history_count].copy_from_slice(history);
        while !all_zero {
            all_zero = true;

            for i in 0..difference_count - 1 {
                differences[i] = differences[i+1] - differences[i];
                all_zero = all_zero && differences[i] == 0;
            }

            difference_count -= 1;
        }
        part_1 += (difference_count..history_count).fold(0, |acc, i| acc + differences[i]);


        difference_count = history_count;
        all_zero = false;

        differences[..history_count].copy_from_slice(history);
        while !all_zero {
            all_zero = true;

            for i in (history_count - difference_count + 1..history_count).rev() {
                differences[i] = differences[i] - differences[i - 1];
                all_zero = all_zero && differences[i] == 0;
            }

            difference_count -= 1;
        }
        part_2 += (0..history_count - difference_count).rev().fold(0, |acc, i| differences[i] - acc);
    }

    return (part_1.to_string(), part_2.to_string());
}
