struct Vec3 {
    x: u32,
    y: u32,
    z: u32,
}

impl Vec3 {
    fn new(_x: u32, _y: u32, _z: u32) -> Vec3 {
        return Vec3 { x: _x, y: _y, z: _z };
    }
}

struct Brick {
    start: Vec3,
    end: Vec3,
    supporting: Vec<usize>,
    supported_by: Vec<usize>,
}

impl Brick {
    fn new(_start: Vec3, _end: Vec3) -> Brick {
        return Brick { start: _start, end: _end, supporting: Vec::new(), supported_by: Vec::new() };
    }
}

fn count_chain_reaction(bricks: &[Brick], brick: usize, fallen: &mut [bool]) -> u64 {
    fallen[brick] = true;
    let mut count = 0;

    for &b in bricks[brick].supporting.iter() {
        if fallen[b] {
            continue;
        }

        let support_count = bricks[b].supported_by.iter().fold(0, |acc, &s| acc + !fallen[s] as u32);
        if support_count == 0 {
            count += 1 + count_chain_reaction(bricks, b, fallen);
        }
    }

    return count;
}

pub fn solution(input: &str) -> (String, String) {
    let mut width = 0;
    let mut depth = 0;
    let mut bricks: Vec<_> = input
        .lines()
        .map(|line| {
            let split = line.split_once('~').unwrap();

            let mut a = split.0.split(',');
            let mut b = split.1.split(',');

            let ax = a.next().unwrap().parse::<u32>().unwrap();
            let ay = a.next().unwrap().parse::<u32>().unwrap();
            let az = a.next().unwrap().parse::<u32>().unwrap();

            let bx = b.next().unwrap().parse::<u32>().unwrap();
            let by = b.next().unwrap().parse::<u32>().unwrap();
            let bz = b.next().unwrap().parse::<u32>().unwrap();

            width = width.max(bx as usize + 1);
            depth = depth.max(by as usize + 1);

            return Brick::new(Vec3::new(ax, ay, az), Vec3::new(bx, by, bz));
        })
        .collect();

    bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
    let mut highest = vec![(0, 0); width * depth];

    for i in 0..bricks.len() {
        let mut z = 0;
        let mut supports: Vec<usize> = Vec::new();

        for y in bricks[i].start.y..=bricks[i].end.y {
            for x in bricks[i].start.x..=bricks[i].end.x {
                let idx = x as usize + y as usize * width;
                if highest[idx].0 > z {
                    z = highest[idx].0;
                    supports.clear();
                    supports.push(highest[idx].1);
                } else if highest[idx].0 == z && z != 0 && !supports.contains(&highest[idx].1) {
                    supports.push(highest[idx].1);
                }
            }
        }

        let height = bricks[i].end.z - bricks[i].start.z;
        bricks[i].start.z = z + 1;
        bricks[i].end.z = bricks[i].start.z + height;
        for &j in supports.iter() {
            bricks[j].supporting.push(i);
        }
        bricks[i].supported_by = supports;

        for y in bricks[i].start.y..=bricks[i].end.y {
            for x in bricks[i].start.x..=bricks[i].end.x {
                let idx = x as usize + y as usize * width;
                highest[idx] = (bricks[i].end.z, i);
            }
        }
    }

    let mut part_1 = 0;
    let mut part_2 = 0;
    for (idx, b) in bricks.iter().enumerate() {
        if b.supporting.len() == 0 || b.supporting.iter().all(|&i| bricks[i].supported_by.len() > 1) {
            part_1 += 1;
        }

        let mut fallen = vec![false; bricks.len()];
        part_2 += count_chain_reaction(&bricks, idx, &mut fallen)
    }

    return (part_1.to_string(), part_2.to_string());
}
