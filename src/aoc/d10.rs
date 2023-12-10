fn add_dir(p: usize, dir: &(i32, i32), width: usize) -> usize {
    return (p as i32 + dir.1 * width as i32 + dir.0) as usize;
}

fn get_pipe_directions(pipe: u8) -> Option<[(i32, i32); 2]> {
    return match pipe {
        b'|' => Some([(0, -1), (0, 1)]),
        b'-' => Some([(-1, 0), (1, 0)]),
        b'L' => Some([(0, -1), (1, 0)]),
        b'J' => Some([(0, -1), (-1, 0)]),
        b'7' => Some([(-1, 0), (0, 1)]),
        b'F' => Some([(1, 0), (0, 1)]),
        _ => None,
    };
}

fn connects_to(from: usize, to: usize, map: &[u8], width: usize) -> bool {
    if let Some(dirs) = get_pipe_directions(map[from]) {
        return add_dir(from, &dirs[0], width) == to || add_dir(from, &dirs[1], width) == to;
    }

    return false;
}

fn find_start_pipe(start: usize, map: &[u8], width: usize) -> u8 {
    let top = connects_to(start - width, start, map, width);
    let left = connects_to(start - 1, start, map, width);
    let right = connects_to(start + 1, start, map, width);
    let bot = connects_to(start + width, start, map, width);

    return match (top, left, right, bot) {
        (true, false, false, true) => b'|',
        (false, true, true, false) => b'-',
        (true, false, true, false) => b'L',
        (true, true, false, false) => b'J',
        (false, true, false, true) => b'7',
        (false, false, true, true) => b'F',
        _ => b'.',
    };
}

pub fn solution(input: &str) -> (String, String) {
    let width = input.as_bytes().iter().position(|c| *c == b'\n').unwrap() + 2;
    let height = (input.len() + 1) / (width - 1) + 2;

    let mut map = vec![b'.'; width * height];
    for (i, line) in input.split_terminator('\n').enumerate() {
        map[(i + 1) * width + 1..(i + 2) * width - 1].copy_from_slice(line.as_bytes());
    }

    let start = map.iter().position(|c| *c == b'S').unwrap();
    map[start] = find_start_pipe(start, &mut map, width);

    let mut part_1 = 0;
    let mut dists = vec![0; width * height];
    let (mut minx, mut miny, mut maxx, mut maxy) = (usize::MAX, usize::MAX, 0, 0);
    for dir in get_pipe_directions(map[start]).unwrap() {
        let mut p = add_dir(start, &dir, width);
        let mut from = start;

        while p != start {
            dists[p] = dists[from] + 1;

            let dirs = get_pipe_directions(map[p]).unwrap();
            (p, from) =
                (dirs.iter().map(|d| add_dir(p, d, width)).find(|pp| *pp != from).unwrap(), p);

            minx = std::cmp::min(minx, p % width);
            miny = std::cmp::min(miny, p / width);
            maxx = std::cmp::max(maxx, p % width);
            maxy = std::cmp::max(maxy, p / width);

            if dists[p] != 0 && dists[p] < dists[from] {
                part_1 = dists[from];
                break;
            }
        }
    }
    dists[start] = 1; // I use dist != 0 to check if a tile is part of the loop kek

    let mut part_2 = 0;
    for y in miny..maxy + 1 {
        let mut in_loop = false;
        let mut x = minx;
        while x <= maxx {
            let idx = y * width + x;
            if dists[idx] != 0 && (map[idx] == b'L' || map[idx] == b'F') {
                let start = idx;
                x += 1;

                while x <= maxx && map[y*width+x] == b'-' {
                    x += 1;
                }

                if (map[start] == b'L' && map[y*width+x] == b'7') || (map[start] == b'F' && map[y*width+x] == b'J') {
                    in_loop = !in_loop;
                }

                x += 1;
                continue;
            }

            if dists[idx] != 0 {
                in_loop = !in_loop;
            } else if in_loop {
                part_2 += 1;
            }

            x += 1;
        }
    }

    return (part_1.to_string(), part_2.to_string());
}
