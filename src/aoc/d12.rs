fn count_ways(record: &[u8], groups: &[usize]) -> usize {
    let mut dp = vec![0usize; (record.len() + 1) * (groups.len() + 1)];
    let mut max_group_len = vec![0usize; record.len()];

    let mut next_sep = 0;
    for i in 0..record.len() {
        if record[i] == b'.' || (i != 0 && record[i - 1] == b'#') {
            continue;
        }

        if next_sep <= i {
            next_sep = i + record[i..].iter().position(|&c| c == b'.').unwrap_or(record.len() - i);
        }

        max_group_len[i] = next_sep - i;
    }

    let idx = |i: usize, j: usize| i * (groups.len() + 1) + j;

    dp[0] = 1;
    for (i, &char) in record.iter().enumerate() {
        for (j, &group) in groups.iter().enumerate() {
            if max_group_len[i] >= group && (i + group == record.len() || record[i + group] != b'#') {
                dp[idx(std::cmp::min(record.len(), i + group + 1), j + 1)] += dp[idx(i,j)];
            }
            
            if char != b'#' {
                dp[idx(i + 1, j)] += dp[idx(i, j)];
            }
        }
    
        if char != b'#' {
            dp[idx(i + 1, groups.len())] += dp[idx(i, groups.len())];
        }
    }

    return *dp.last().unwrap();
}

pub fn solution(input: &str) -> (String, String) {
    let mut part_1 = 0;
    let mut part_2 = 0;

    for (in_record, in_group) in input.lines().filter_map(|s| s.split_once(' ')) {
        let record = in_record.as_bytes().to_vec();
        let groups: Vec<_> = in_group.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        let mut big_record = Vec::with_capacity(record.len() * 5 + 4);
        let mut big_groups = Vec::with_capacity(groups.len() * 5 + 4);
        for i in 0..5 {
            big_record.extend_from_slice(&record);
            big_groups.extend_from_slice(&groups);
            if i < 4 {
                big_record.push(b'?');
            }
        }

        part_1 += count_ways(&record, &groups);
        part_2 += count_ways(&big_record, &big_groups);
    }

    return (part_1.to_string(), part_2.to_string());
}
