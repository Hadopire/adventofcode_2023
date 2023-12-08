fn node_name_to_idx(name: &[u8]) -> u16 {
    let mut idx = 0u16;
    for (i, c) in name.iter().enumerate() {
        idx |= ((c - b'A') as u16) << ((2 - i) * 5);
    }

    return idx;
}

fn do_next_instruction(map: &[(u16,u16)], instructions: &[u8], pos: &mut usize, iptr: &mut usize) {
    if instructions[*iptr] == b'L' {
        *pos = map[*pos].0 as usize;
    } else {
        *pos = map[*pos].1 as usize;
    }
    
    *iptr += 1;
    if *iptr == instructions.len() {
        *iptr = 0;
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    return gcd(b, a % b);
}

pub fn solution(input: &str) -> (String, String) {
    let mut map = vec![(0u16, 0u16); 1 << 15];
    let split_input = input.split_once("\n\n").unwrap();
    let instructions = split_input.0.as_bytes();
    let mut positions: Vec<usize> = vec![];

    for entry in split_input.1.split_terminator('\n') {
        let split_entry = entry.split_once('=').unwrap();
        let node_in = split_entry.0.as_bytes();
        let node_out = split_entry.1.as_bytes();

        let node_in_idx = node_name_to_idx(&node_in[0..3]) as usize;
        map[node_in_idx] = (node_name_to_idx(&node_out[2..5]), node_name_to_idx(&node_out[7..10]));

        if node_in[2] == b'A' {
            positions.push(node_in_idx);
        }
    }

    let mut pos = 0;
    let mut iptr = 0;
    let mut part_1 = 0;
    loop {
        part_1 += 1;
        do_next_instruction(&map[..], instructions, &mut pos, &mut iptr);
        if pos == node_name_to_idx(&[b'Z', b'Z', b'Z']) as usize {
            break;
        }
    }

    let mut distances = vec![0u64; positions.len()];
    for (i, pos) in positions.iter_mut().enumerate() {
        iptr = 0;
        loop {
            distances[i] += 1;
            do_next_instruction(&map[..], instructions, pos, &mut iptr);
            if (*pos & 31) == 25 {
                break;
            }
        }
    }

    let mut part_2 = distances[0];
    for dist in distances.iter().skip(1) {
        part_2 = (dist * part_2) / gcd(*dist, part_2)
    }

    return (part_1.to_string(), part_2.to_string());
}
