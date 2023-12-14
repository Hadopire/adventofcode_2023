use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn tilt(map: &mut [u8], start: i32, lane_count: i32, lane_stride: i32, elem_count: i32, elem_stride: i32) {
    for l in 0..lane_count {
        let mut pos = start + lane_stride * l;
        let mut empty_cell = pos;
        for _ in 0..elem_count {
            match map[pos as usize] {
                b'O' => {
                    if pos != empty_cell {
                        map[empty_cell as usize] = b'O';
                        map[pos as usize] = b'.';
                    }
                    empty_cell += elem_stride;
                }
                b'#' => empty_cell = pos + elem_stride,
                _ => (),
            }

            pos += elem_stride;
        }
    }
}

fn map_weight(map: &[u8], width: usize, height: usize) -> usize {
    let mut weight = 0;

    for x in 0..width {
        for y in 0..height {
            match map[y * width + x] {
                b'O' => weight += height - y,
                _ => (),
            }
        }
    }

    return weight;
}

pub fn solution(input: &str) -> (String, String) {
    let width = input.as_bytes().iter().position(|&c| c == b'\n').unwrap();
    let height = (input.len() + 1) / (width + 1);

    let mut map = vec![0u8; width * height];
    for i in 0..height {
        map[i * width..i * width + width].copy_from_slice(&input.as_bytes()[i * (width + 1)..i * (width + 1) + width]);
    }

    let mut part_1 = 0;
    for x in 0..width {
        let mut row = height;
        for y in 0..height {
            match map[y * width + x] {
                b'O' => {
                    part_1 += row;
                    row -= 1;
                }
                b'#' => row = height - y - 1,
                _ => (),
            }
        }
    }

    let mut weights = vec![];
    let mut sequence: HashMap<Vec<u8>, usize> = HashMap::new();
    let mut part_2 = 0;
    for i in 0..1000000000 {
        match sequence.entry(map.clone()) {
            Entry::Occupied(e) => {
                let cycle_index = e.get();
                part_2 = weights[cycle_index + (1000000000 - cycle_index) % (i - cycle_index)];
                break;
            },
            Entry::Vacant(e) => {
                e.insert(i);
            }
        }

        weights.push(map_weight(&map, width, height));

        tilt(&mut map[..], 0, width as i32, 1, height as i32, width as i32);
        tilt(&mut map[..], 0, height as i32, width as i32, width as i32, 1);
        tilt(&mut map[..], (width * height - width) as i32, width as i32, 1, height as i32, -(width as i32));
        tilt(&mut map[..], (width - 1) as i32, height as i32, width as i32, width as i32, -1);
    }

    return (part_1.to_string(), part_2.to_string());
}
