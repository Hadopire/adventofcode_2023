struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Vec2 {
        return Vec2 { x: x, y: y };
    }
}

struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let width = input.as_bytes().iter().position(|&c| c == b'\n').unwrap() + 2;
        let height = (input.len() + 1) / (width - 1) + 2;

        let mut grid: Vec<u8> = vec![b'o'; width * height];
        for y in 0..height - 2 {
            grid[(y + 1) * width + 1..(y + 1) * width + width - 1]
                .copy_from_slice(&input.as_bytes()[y * (width - 1)..y * (width - 1) + width - 2]);
        }

        return Grid { data: grid, width: width, height: height };
    }

    fn get(&self, pos: &Vec2) -> u8 {
        return self.data[pos.y as usize * self.width + pos.x as usize];
    }
}

fn cast_laser(p: &Vec2, d: &Vec2, grid: &Grid, visited: &mut [bool], energized: &mut [bool]) -> usize {
    let upx = p.x as usize;
    let upy = p.y as usize;
    let udx = (d.x + 1) as usize;
    let udy = (d.y + 1) as usize;
    let visited_idx = upx + upy * grid.width + udx * grid.width * grid.height + udy * grid.width * grid.height * 3;
    if visited[visited_idx] || grid.get(p) == b'o' {
        return 0;
    }
    visited[visited_idx] = true;

    let mut count = 0;
    if !energized[p.x as usize + grid.width * p.y as usize] {
        energized[p.x as usize + grid.width * p.y as usize] = true;
        count += 1;
    };

    match grid.get(p) {
        b'.' => count += cast_laser(&Vec2::new(p.x + d.x, p.y + d.y), d, grid, visited, energized),
        b'\\' => count += cast_laser(&Vec2::new(p.x + d.y, p.y + d.x), &Vec2::new(d.y, d.x), grid, visited, energized),
        b'/' => count += cast_laser(&Vec2::new(p.x - d.y, p.y - d.x), &Vec2::new(-d.y, -d.x), grid, visited, energized),
        b'-' if d.x != 0 => count += cast_laser(&Vec2::new(p.x + d.x, p.y + d.y), d, grid, visited, energized),
        b'|' if d.x == 0 => count += cast_laser(&Vec2::new(p.x + d.x, p.y + d.y), d, grid, visited, energized),
        b'-' => {
            count += cast_laser(&Vec2::new(p.x - 1, p.y), &Vec2::new(-1, 0), grid, visited, energized);
            count += cast_laser(&Vec2::new(p.x + 1, p.y), &Vec2::new(1, 0), grid, visited, energized);
        }
        b'|' => {
            count += cast_laser(&Vec2::new(p.x, p.y - 1), &Vec2::new(0, -1), grid, visited, energized);
            count += cast_laser(&Vec2::new(p.x, p.y + 1), &Vec2::new(0, 1), grid, visited, energized);
        }
        _ => (),
    }

    return count;
}

fn energized_count(grid: &Grid, start: &Vec2, dir: &Vec2) -> usize {
    let mut visited: Vec<bool> = vec![false; grid.width * grid.height * 3 * 3];
    let mut energized: Vec<bool> = vec![false; grid.width * grid.height];
    
    return cast_laser(start, dir, grid, &mut visited, &mut energized);
}

pub fn solution(input: &str) -> (String, String) {
    let grid = Grid::new(input);
    let part_1 = energized_count(&grid, &Vec2::new(1, 1), &Vec2::new(1, 0));

    let mut part_2 = 0;
    for i in 1..grid.width-1 {
        part_2 = std::cmp::max(part_2, energized_count(&grid, &Vec2::new(i as i32, 1), &Vec2::new(0, 1)));
        part_2 = std::cmp::max(part_2, energized_count(&grid, &Vec2::new(i as i32, grid.height as i32 - 2), &Vec2::new(0, -1)));
        part_2 = std::cmp::max(part_2, energized_count(&grid, &Vec2::new(1, i as i32), &Vec2::new(1, 0)));
        part_2 = std::cmp::max(part_2, energized_count(&grid, &Vec2::new(grid.width as i32 - 2, i as i32), &Vec2::new(-1, 0)));
    }

    return (part_1.to_string(), part_2.to_string());
}
