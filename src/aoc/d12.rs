use std::collections::HashMap;

// with the input record_len et groups_len are small numbers, so it's better to fit
// the 4 values together in a 64bit number instead of using a tuple as the hash table key
fn hash(spring: u8, current_group: u16, record_len: usize, groups_len: usize) -> u64 {
    return (spring) as u64 | ((current_group as u64) << 16) | ((record_len as u64) << 32) | ((groups_len as u64) << 48);
}

fn count_ways(current_group: Option<u16>, record: &mut[u8], groups: &[u16], cache: &mut HashMap<u64, u64>) -> u64 {
    if record.is_empty() {
        match (current_group, groups.len()) {
            (None, 0) => return 1,
            (Some(n), 1) => return (n == groups[0]) as u64,
            _ => return 0,
        };
    }

    if let Some(n) = current_group {
        if n > groups[0] {
            return 0;
        }
    }

    let hash = hash(record[0], current_group.unwrap_or(u16::MAX), record.len(), groups.len());
    if let Some(n) = cache.get(&hash) {
        return *n;
    }

    let n = match (record[0], current_group) {
        (b'#', Some(n)) => count_ways(Some(n + 1), &mut record[1..], groups, cache),
        (b'#', None) => {
            if groups.is_empty() {
                0
            } else {
                count_ways(Some(1), &mut record[1..], &groups, cache)
            }
        }
        (b'.', Some(n)) => {
            if n < groups[0] {
                0
            } else {
                count_ways(None, &mut record[1..], &groups[1..], cache)
            }
        }
        (b'.', None) => count_ways(None, &mut record[1..], &groups, cache),
        (b'?', _) => {
            let mut n = 0;
            record[0] = b'#';
            n += count_ways(current_group, record, groups, cache);
            record[0] = b'.';
            n += count_ways(current_group, record, groups, cache);
            record[0] = b'?';
            n
        },
        _ => 0
    };

    cache.insert(hash, n);
    return n;
}

pub fn solution(input: &str) -> (String, String) {
    let mut records : Vec<Vec<u8>> = vec![];
    let mut groups : Vec<Vec<u16>> = vec![];

    for line in input.split_terminator('\n') {
        let split = line.split_once(' ').unwrap();
        records.push(split.0.as_bytes().to_vec());
        groups.push(split.1.split(',').map(|s| s.parse::<u16>().unwrap()).collect());
    }

    let first = (0..records.len()).fold(0, |acc, i| {
        let capacity = groups[i].len() * records[i].len() * *groups[i].iter().max().unwrap() as usize;
        let mut cache: HashMap<u64, u64> = HashMap::with_capacity(capacity * 3);
        return acc + count_ways(None, &mut records[i], &groups[i], &mut cache);
    });

    let mut big_records : Vec<Vec<u8>> = Vec::with_capacity(records.len());
    let mut big_groups : Vec<Vec<u16>> = Vec::with_capacity(groups.len());

    for i in 0..groups.len() {
        let mut record : Vec<u8> = Vec::with_capacity(records[i].len() * 5 + 4);
        for j in 0..5 {
            if j != 0 {
                record.push(b'?');
            }
            record.extend(&records[i]);
        }

        let mut group : Vec<u16> = Vec::with_capacity(groups[i].len() * 5);
        (0..5).for_each(|_| group.extend(&groups[i]));

        big_records.push(record);
        big_groups.push(group);
    }

    let second = (0..big_records.len()).fold(0, |acc, i| {
        let capacity = big_groups[i].len() * big_records[i].len() * *groups[i].iter().max().unwrap() as usize;
        let mut cache: HashMap<u64, u64> = HashMap::with_capacity(capacity);
        return acc + count_ways(None, &mut big_records[i], &big_groups[i], &mut cache)
    });
    
    return (first.to_string(), second.to_string());
}