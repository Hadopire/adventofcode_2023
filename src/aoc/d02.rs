pub fn solution(input: &str) -> (String, String) {
    let mut part_1 = 0;
    let mut part_2 = 0;

    for (i, line) in input.split_terminator('\n').enumerate() {
        let mut impossible = false;
        let mut min_cube_count = [0, 0, 0];

        let s = &line[line.find(": ").unwrap() + 2..];
        for set in s.split(';') {
            let mut cube_count = [12, 13, 14];
            for mut cube in set.split(',') {
                cube = cube.trim_start();
                let v: Vec<&str> = cube.split(' ').collect();

                let count = v[0].parse::<i32>().unwrap();
                let color_idx = match v[1].as_bytes()[0] {
                    b'r' => 0,
                    b'g' => 1,
                    b'b' => 2,
                    _ => 0,
                };
                cube_count[color_idx] -= count;
            }

            if cube_count.iter().any(|&e| e < 0) {
                impossible = true;
            }

            cube_count.iter().enumerate().for_each(|(i, e)| {
                min_cube_count[i] = std::cmp::max((12 + i as i32) - e, min_cube_count[i])
            })
        }

        if !impossible {
            part_1 += i + 1;
        }

        part_2 += min_cube_count.iter().product::<i32>();
    }

    return (part_1.to_string(), part_2.to_string());
}
