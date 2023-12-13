fn reflect_cmp(
    pattern: &[u8],
    left: usize,
    right: usize,
    count: usize,
    row_count: usize,
    stride: usize,
    offset: usize,
    max_diff_count: u32,
) -> u32 {
    let mut diffs = 0;

    for i in 0..row_count {
        let a = left - offset * i;
        let b = right + offset * i;
        for j in 0..count {
            if pattern[a + stride * j] != pattern[b + stride * j] {
                diffs += 1;
                if diffs > max_diff_count {
                    return diffs;
                }
            }
        }
    }

    return diffs;
}

pub fn solution(input: &str) -> (String, String) {
    let mut part_1 = 0;
    let mut part_2 = 0;
    for pattern in input.split_terminator("\n\n").map(|s| s.as_bytes()) {
        let width = pattern.iter().position(|&c| c == b'\n').unwrap() + 1;
        let height = (pattern.len() + 1) / width;

        for col in 0..(width - 2) {
            let col_count = std::cmp::min(col + 1, (width - 1) - (col + 1));
            let diff_count = reflect_cmp(pattern, col, col + 1, height, col_count, width, 1, 1);
            match diff_count {
                0 => part_1 += col + 1,
                1 => part_2 += col + 1,
                _ => (),
            }
        }

        for row in 0..(height - 1) {
            let row_count = std::cmp::min(row + 1, height - row - 1);
            let diff_count = reflect_cmp(pattern, row * width, (row + 1) * width, width - 1, row_count, 1, width, 1);
            match diff_count {
                0 => part_1 += (row + 1) * 100,
                1 => part_2 += (row + 1) * 100,
                _ => (),
            }
        }
    }
    return (part_1.to_string(), part_2.to_string());
}
