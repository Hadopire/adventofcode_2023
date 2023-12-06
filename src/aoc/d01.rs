pub fn solution(input: &str) -> (String, String) {
    let part_1 = input.split_terminator("\n").fold(0, |mut acc, line| {
        let isdigit = |c: &char| return *c >= '0' && *c <= '9';
        let first = line.chars().find(isdigit).unwrap();
        let last = line.chars().rev().find(isdigit).unwrap();
        acc += first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap();
        return acc;
    });

    let spelled_digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let part_2 = input.split_terminator("\n").fold(0, |mut acc, line| {
        let isdigit = |c: char| return c >= '0' && c <= '9';
        let mut first_pos = line.chars().position(isdigit).unwrap();
        let mut first_digit = line.chars().nth(first_pos).unwrap().to_digit(10).unwrap() * 10;
        let mut second_pos = line.chars().rev().position(isdigit).unwrap();
        let mut second_digit = line.chars().rev().nth(second_pos).unwrap().to_digit(10).unwrap();
        second_pos = line.len() - second_pos - 1;
        for (i, s) in spelled_digits.iter().enumerate() {
            let pos = line.find(s);
            if let Some(p) = pos {
                if p < first_pos {
                    first_pos = p;
                    first_digit = (i as u32 + 1) * 10;
                }
            }

            let pos = line.rfind(s);

            if let Some(p) = pos {
                if p > second_pos {
                    second_pos = p;
                    second_digit = i as u32 + 1;
                }
            }
        }

        acc += first_digit + second_digit;
        return acc;
    });

    return (part_1.to_string(), part_2.to_string());
}
