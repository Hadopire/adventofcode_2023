pub fn solution(input: &str) -> (String, String) {
    let part_1: u32 = input
        .trim_end()
        .split_terminator(',')
        .map(|s| s.as_bytes().iter().fold(0, |acc, &c| (acc + c as u32) * 17 % 256))
        .sum();

    let mut boxes: Vec<Vec<(&[u8], u8)>> = vec![Vec::new(); 256];
    let entries = input.trim_end().split_terminator(',').map(|s| {
        let op_pos = s.as_bytes().iter().position(|&c| c == b'=' || c == b'-').unwrap();
        (&s.as_bytes()[..op_pos], &s.as_bytes()[op_pos..])
    });

    for (key, op) in entries {
        let hash = key.iter().fold(0, |acc, &c| (acc + c as usize) * 17 % 256);

        if op[0] == b'=' {
            if let Some(idx) = boxes[hash].iter().position(|(_key, _)| *_key == key) {
                boxes[hash][idx] = (key, op[1] - b'0');
            } else {
                boxes[hash].push((key, op[1] - b'0'));
            }
        } else if op[0] == b'-' {
            if let Some(idx) = boxes[hash].iter().position(|(_key, _)| *_key == key) {
                boxes[hash].remove(idx);
            }
        }
    }

    let part_2: u32 = boxes
        .iter()
        .enumerate()
        .filter(|(_, b)| !b.is_empty())
        .map(|(box_idx, b)| {
            b.iter()
                .enumerate()
                .map(move |(slot, (_, value))| ((box_idx + 1) * (slot + 1)) as u32 * *value as u32)
                .sum::<u32>()
        })
        .sum();

    return (part_1.to_string(), part_2.to_string());
}
