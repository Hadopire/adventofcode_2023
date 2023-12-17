use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn minimum_heat_loss(map: &[u8], width: usize, height: usize, min_step: usize, max_step: usize) -> u32 {
    let mut visited = vec![false; width * height * 4];
    let mut dists = vec![u32::MAX; width * height * 2];
    let mut pqueue: BinaryHeap<(Reverse<u32>, usize)> = BinaryHeap::new();
    pqueue.push((Reverse(0), 1));
    pqueue.push((Reverse(0), 2));

    return loop {
        let (dist, pos_dir) = pqueue.pop().unwrap();
        let pos = pos_dir >> 2;

        if pos == map.len() - 1 {
            break dist.0;
        }

        if visited[pos_dir] {
            continue;
        }
        visited[pos_dir] = true;

        let vh_dir = (pos_dir ^ 1) & 1;
        for new_dir in [vh_dir, vh_dir | 2] {
            let mut new_dist = dist.0;
            let mut new_pos = pos;
            let (stride, step_count) = match new_dir {
                0 => (-( width as isize ), max_step.min(              pos / width )),
                1 => (                  1, max_step.min(  width - pos % width - 1 )),
                2 => (     width as isize, max_step.min( height - pos / width - 1 )),
                3 => (                 -1, max_step.min(              pos % width )),
                _ => (0, 0),
            };

            for step in 1..=step_count {
                new_pos = new_pos.wrapping_add_signed(stride);
                new_dist += map[new_pos] as u32;
                if step >= min_step && new_dist < dists[(new_pos << 1) | (new_dir & 1)] {
                    dists[(new_pos << 1) | (new_dir & 1)] = new_dist;
                    pqueue.push((Reverse(new_dist), (new_pos << 2) | new_dir));
                }
            }
        }
    };
}

pub fn solution(input: &str) -> (String, String) {
    let width = input.as_bytes().iter().position(|&c| c == b'\n').unwrap();
    let height = input.lines().filter(|s| s.len() != 0).count();
    let map: Vec<_> = input.lines().flat_map(str::as_bytes).map(|&c| c - b'0').collect();

    let part_1 = minimum_heat_loss(&map, width, height, 1, 3);
    let part_2 = minimum_heat_loss(&map, width, height, 4, 10);

    return (part_1.to_string(), part_2.to_string());
}
