use std::collections::HashMap;
use std::str::from_utf8;

pub fn solution(input: &str) -> (String, String) {
    let w = input.as_bytes().iter().position(|&c| c == b'\n').unwrap() + 2;
    let h = (input.len() + 1) / (w - 1) + 2;
    let mut schematic = vec![b'.'; w * h];
    let mut gears: HashMap<u32, (u32, u32)> = HashMap::new();

    for y in 1..h - 1 {
        schematic[y * w + 1..y * w + w - 1]
            .copy_from_slice(&input.as_bytes()[(y - 1) * (w - 1)..(y - 1) * (w - 1) + (w - 2)])
    }

    let mut part_1 = 0;
    let mut part_2 = 0;
    let mut update_gear = |p: usize, n: u32| {
        if schematic[p] == b'*' {
            let gear = gears.entry(p as u32).or_insert((0, 1));
            gear.0 += 1;
            gear.1 *= n;
        }
    };

    for y in 1..h - 1 {
        let mut x = 0;
        let line = &schematic[y * w..y * w + w];
        while x < line.len() {
            while x < line.len() && !(line[x] as char).is_numeric() {
                x += 1;
            }

            if x == line.len() {
                break;
            }
            let number_start = x;

            while x < line.len() && (line[x] as char).is_numeric() {
                x += 1;
            }

            let mut is_part = false;
            let start = y * w + number_start;
            let end = y * w + x;

            let number = from_utf8(&line[number_start..x]).unwrap().parse::<u32>().unwrap();

            for i in start - 1..end + 1 {
                if schematic[i - w] != b'.' || schematic[i + w] != b'.' {
                    is_part = true;
                }
                update_gear(i - w, number);
                update_gear(i + w, number);
            }
            update_gear(start - 1, number);
            update_gear(end, number);
            is_part = is_part || schematic[start - 1] != b'.' || schematic[end] != b'.';

            if is_part {
                part_1 += number;
            }
        }
    }

    for (_, gear) in gears {
        if gear.0 == 2 {
            part_2 += gear.1;
        }
    }

    return (part_1.to_string(), part_2.to_string());
}
