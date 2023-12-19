pub fn solution(input: &str) -> (String, String) {
    let mut pos = (0, 0);
    let mut big_pos = (0, 0);
    let plan: Vec<_> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut split = line.split_terminator(' ');
            let dir = split.next().unwrap().as_bytes()[0];
            let count = split.next().unwrap().parse::<i32>().unwrap();
            let color = split.next().unwrap();

            pos = match dir {
                b'U' => (pos.0, pos.1 - count),
                b'R' => (pos.0 + count, pos.1),
                b'D' => (pos.0, pos.1 + count),
                b'L' => (pos.0 - count, pos.1),
                _ => (0, 0),
            };

            let big_count = i64::from_str_radix(&color[2..color.len() - 2], 16).unwrap();
            let big_dir = color.as_bytes()[color.len() - 2];

            big_pos = match big_dir {
                b'3' => (big_pos.0, big_pos.1 - big_count),
                b'0' => (big_pos.0 + big_count, big_pos.1),
                b'1' => (big_pos.0, big_pos.1 + big_count),
                b'2' => (big_pos.0 - big_count, big_pos.1),
                _ => (0, 0),
            };

            (pos, big_pos)
        })
        .collect();

    let mut area = 0;
    let mut perimeter = 0;
    let mut big_area = 0;
    let mut big_perimeter = 0;
    for i in 0..plan.len() {
        let (a, big_a) = plan[i];
        let (b, big_b) = plan[(i+1)%plan.len()];

        area += (a.1 + b.1) * (a.0 - b.0);
        perimeter += (a.1-b.1).abs() + (a.0 - b.0).abs();

        big_area += (big_a.1 + big_b.1) * (big_a.0 - big_b.0);
        big_perimeter += (big_a.1-big_b.1).abs() + (big_a.0-big_b.0).abs();
    }

    area = (area / 2).abs();
    let interior_points = area - perimeter / 2 + 1;
    let part_1 = interior_points + perimeter;

    big_area = (big_area / 2).abs();
    let big_interior_points = big_area - big_perimeter / 2 + 1;
    let part_2 = big_interior_points + big_perimeter;

    return (part_1.to_string(), part_2.to_string());
}
