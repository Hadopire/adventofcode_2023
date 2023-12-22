use std::collections::VecDeque;

pub fn solution(input: &str) -> (String, String) {
    let width = input.as_bytes().iter().position(|&c| c == b'\n').unwrap();
    let height = input.lines().filter(|s| s.len() != 0).count();
    let map: Vec<_> = input.lines().flat_map(str::as_bytes).copied().collect();
    let mut visited = vec![false; width * 6 * width * 6];
    let visited_idx = |x, y| ((x + width as isize * 3) + ((y + width as isize * 3) * width as isize * 6)) as usize;
    let mut queue: VecDeque<((isize, isize), i64)> = VecDeque::with_capacity(map.len());
    let start = map.iter().position(|&c| c == b'S').unwrap();

    queue.push_back((((start % width) as isize, (start / width) as isize), 0));
    visited[visited_idx((start % width) as isize, (start / width) as isize)] = true;

    let mut part_1 = 0;
    let part_1_goal = 64;
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let x = [(width / 2) as i64, (width / 2 + width) as i64, (width / 2 + width * 2) as i64];
    let mut y = [0, 0, 0];
    while !queue.is_empty() {
        let (pos, dist) = queue.pop_front().unwrap();

        if dist <= part_1_goal && dist % 2 == part_1_goal % 2 {
            part_1 += 1;
        }

        for i in 0..x.len() {
            if dist <= x[i] && dist % 2 == x[i] % 2 {
                y[i] += 1;
            }
        }

        if dist == x[2] {
            continue;
        }

        for dir in dirs {
            let p = (pos.0 + dir.0, pos.1 + dir.1);
            if !visited[visited_idx(p.0, p.1)] {
                let mod_p = (p.0.rem_euclid(width as isize), p.1.rem_euclid(height as isize));
                let idx = mod_p.0 as usize + mod_p.1 as usize * width;
                if map[idx] != b'#' {
                    queue.push_back((p, dist + 1));
                    visited[visited_idx(p.0, p.1)] = true;
                }
            }
        }
    }

    let a = (y[2] - y[0]) / 2 - (y[1] - y[0]);
    let b = y[1] - a - y[0];
    let c = y[0];

    let x = (26501365 - x[0]) / width as i64;
    let part_2 = a * x * x + b * x + c;

    return (part_1.to_string(), part_2.to_string());
}
